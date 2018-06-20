use std::collections::HashSet;

use cast::usize;
use failure::Error;
use yaml_rust::yaml::Hash;

use super::*;

pub fn properties_to_fields(required: &[&str], hash: &Hash) -> Result<Vec<Field>, Error> {
    let mut ret = Vec::new();
    let mut required: HashSet<&str> = required.into_iter().cloned().collect();

    for (name, field) in hash {
        let name: String = name
            .as_str()
            .ok_or_else(|| format_err!("non-string field name: {:?}", field))?
            .to_string();
        let field: &Hash = field
            .as_hash()
            .ok_or_else(|| format_err!("non-hash field body"))?;

        let mut current_keys = keys(field)?;

        let description = if current_keys.remove("description") {
            get_string(field, "description")?.to_string()
        } else {
            String::new()
        };

        current_keys.remove("example");

        let nullable = if current_keys.remove("x-nullable") {
            Some(get_bool(field, "x-nullable")?)
        } else {
            None
        };

        let required = required.remove(name.as_str());

        let data_type = field_type(field, &mut current_keys)
            .with_context(|_| format_err!("determining type of {}", name))?;

        current_keys.remove("x-go-name"); // TODO?

        ensure!(
            current_keys.is_empty(),
            "{}: unrecognised keys: {:?}",
            &name,
            current_keys
        );

        ret.push(Field {
            name,
            description,
            data_type,
            nullable,
            required,
        })
    }

    ensure!(
        required.is_empty(),
        "required properties which aren't present: {:?}",
        required
    );

    Ok(ret)
}

pub fn field_type(field: &Hash, current_keys: &mut HashSet<&str>) -> Result<FieldType, Error> {
    Ok(if current_keys.remove("properties") {
        if current_keys.remove("type") {
            let object = get_string(field, "type")?;
            ensure!(
                "object" == object,
                "field with `properties` must have no type, or type object, not {:?}",
                object
            );
        }

        let required = if current_keys.remove("required") {
            get_vec(field, "required")?
                .into_iter()
                .map(|yaml| {
                    yaml.as_str()
                        .ok_or_else(|| format_err!("required param must be string, not {:?}", yaml))
                })
                .collect::<Result<Vec<&str>, Error>>()?
        } else {
            Vec::new()
        };

        FieldType::Fields(properties_to_fields(
            &required,
            get_hash(field, "properties")?,
        )?)
    } else if current_keys.remove("additionalProperties") {
        current_keys.remove("type");

        // ???

        // ?????
        current_keys.remove("default");
        FieldType::Unknown
    } else if current_keys.remove("allOf") {
        if current_keys.remove("type") {
            let object = get_string(field, "type")?;
            ensure!(
                "object" == object,
                "field with `allOf` must have no type, or type object, not {:?}",
                object
            );
        }

        let mut ret = Vec::new();

        for child in get_vec(field, "allOf")? {
            let child = child
                .as_hash()
                .ok_or_else(|| format_err!("non-hash in allOf"))?;
            ret.push(field_type(child, &mut keys(child)?)?)
        }

        FieldType::AllOf(ret)
    } else if current_keys.remove("type") {
        match get_string(field, "type")? {
            "object"
                if current_keys.is_empty()
                    // the `example:` case here seems to be just an attempt to provide an example
                    // to something else in an existing allOf union type
                    || (1 == current_keys.len() && current_keys.contains("example")) =>
            {
                // TODO: Total bullshit. No idea what this should do.
                FieldType::Unknown
            }
            "object" => bail!(
                "type object, but no properties, only {:?} (nothing would be a different case)",
                current_keys
            ),
            "array" => {
                ensure!(current_keys.remove("items"), "arrays must have items");
                let null_default = if current_keys.remove("default") {
                    let default = get(field, "default")?;

                    if !default.is_null() {
                        bail!("unsupported array default: {:?}", default)
                    }

                    true
                } else {
                    false
                };

                current_keys.remove("minItems");
                current_keys.remove("maxItems");
                let items = get_hash(field, "items")?;
                let items = field_type(items, &mut keys(items)?)?;

                FieldType::Array {
                    tee: Box::new(items),
                    constraints: ArrayConstraints {
                        min_items: optional_integer(field, "minItems")?.map(usize).invert()?,
                        max_items: optional_integer(field, "maxItems")?.map(usize).invert()?,
                        null_default,
                    },
                }
            }
            "integer" => {
                let format = if current_keys.remove("format") {
                    match get_string(field, "format")? {
                        "int8" => IntegerFormat::I8,
                        "int16" => IntegerFormat::I16,
                        "int32" => IntegerFormat::I32,
                        "int64" => IntegerFormat::I64,
                        "uint8" => IntegerFormat::U8,
                        "uint16" => IntegerFormat::U16,
                        "uint32" => IntegerFormat::U32,
                        "uint64" => IntegerFormat::U64,
                        other => bail!("unsupported integer format: {}", other),
                    }
                } else {
                    IntegerFormat::Unspecified
                };

                current_keys.remove("minimum");
                current_keys.remove("maximum");
                current_keys.remove("default");
                current_keys.remove("enum"); // TODO: ? Maybe not worth supporting.

                // Kind:
                //  description: "Kind of change"
                //  type: "integer"
                //  format: "uint8"
                //  enum: [0, 1, 2]
                //  x-nullable: false

                FieldType::Simple(DataType::Integer {
                    min: optional_integer(field, "minimum")?,
                    max: optional_integer(field, "maximum")?,
                    default: optional_integer(field, "default")?,
                    format,
                })
            }
            "number" => {
                current_keys.remove("default");

                FieldType::Simple(DataType::Number {
                    min: optional_number(field, "minimum")?,
                    max: optional_number(field, "maximum")?,
                    default: optional_number(field, "default")?,
                    format: NumberFormat::Unspecified,
                })
            }
            "boolean" => {
                current_keys.remove("default");
                FieldType::Simple(DataType::Bool {
                    default: optional_bool(field, "default")?,
                })
            }
            "string" => FieldType::Simple(if current_keys.remove("format") {
                match get_string(field, "format")? {
                    "ip-address" => DataType::IpAddr,
                    "dateTime" => DataType::DateTime,
                    "json" => DataType::Json,
                    "binary" => DataType::Binary,
                    other => bail!("unsupported string format: {}", other),
                }
            } else if current_keys.remove("enum") {
                let default = if current_keys.remove("default") {
                    Some(get_string(field, "default")?.to_string())
                } else {
                    None
                };

                DataType::Enum {
                    values: get_vec(field, "enum")?
                        .into_iter()
                        .map(as_str)
                        .map(|result| result.map(|r| r.to_string()))
                        .collect::<Result<Vec<String>, Error>>()?,
                    default,
                }
            } else if current_keys.remove("pattern") {
                DataType::MatchString {
                    pattern: get_string(field, "pattern")?.to_string(),
                }
            } else {
                let default = if current_keys.remove("default") {
                    Some(get_string(field, "default")?.to_string())
                } else {
                    None
                };

                DataType::String { default }
            }),
            other => bail!("unimplemented def type: {}", other),
        }
    } else if 1 == current_keys.len() && current_keys.remove("$ref") {
        FieldType::Ref(get_string(field, "$ref")?.to_string())
    } else {
        bail!(
            "bad type, couldn't guess expectation from: {:?}",
            current_keys
        );
    })
}
