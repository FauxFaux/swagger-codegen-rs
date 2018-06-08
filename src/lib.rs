extern crate cast;
#[macro_use]
extern crate failure;
extern crate result;
extern crate yaml_rust;

use cast::f64;
use failure::Error;
use failure::ResultExt;
use result::ResultOptionExt;
use std::collections::HashSet;
use yaml_rust::yaml::Hash;
use yaml_rust::Yaml;

#[derive(Debug, Clone)]
struct Struct {
    fields: Vec<Field>,
}

#[derive(Debug, Clone)]
struct Field {
    name: String,
    data_type: FieldType,
    description: String,
    nullable: Option<bool>,
    required: bool,
}

#[derive(Debug, Clone)]
enum FieldType {
    Ref(String),
    Inner(usize),
    Simple(DataType),
    #[deprecated]
    Unknown,
}

#[derive(Debug, Copy, Clone)]
enum IntegerFormat {
    Unspecified,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
}

#[derive(Debug, Copy, Clone)]
enum NumberFormat {
    Unspecified,
}

#[derive(Copy, Debug, Clone)]
enum DataType {
    Integer {
        min: Option<i64>,
        max: Option<i64>,
        default: Option<i64>,
        format: IntegerFormat,
    },
    Number {
        min: Option<f64>,
        max: Option<f64>,
        default: Option<f64>,
        format: NumberFormat,
    },
    Bool {
        default: Option<bool>,
    },
    IpAddr,
    String,
}

pub fn go() -> Result<(), Error> {
    let doc = yaml_rust::YamlLoader::load_from_str(include_str!("../examples/docker.yaml"))?;
    let doc = &doc[0];

    println!(
        "{:?}",
        keys(
            doc.as_hash()
                .ok_or_else(|| format_err!("doc must be a hash"))?
        )?
    );

    let mut structs = Vec::new();

    for p in properties_to_fields(
        &mut structs,
        &[],
        doc["definitions"]
            .as_hash()
            .ok_or_else(|| format_err!("no definitions"))?,
    ).with_context(|_| format_err!("processing defintions"))?
    {
        println!("{:?}", p)
    }

    for s in structs {
        println!("{:?}", s);
    }

    for (path_url, path) in doc["paths"]
        .as_hash()
        .ok_or_else(|| format_err!("no paths"))?
        .into_iter()
    {
        let path_url: &str = path_url
            .as_str()
            .ok_or_else(|| format_err!("non-string path url: {:?}", path_url))?;
        let path: &Hash = path
            .as_hash()
            .ok_or_else(|| format_err!("non-hash path body"))?;

        println!("{}", path_url);

        for (http_method, op) in path.into_iter() {
            let http_method = match http_method
                .as_str()
                .ok_or_else(|| format_err!("non-string http method: {:?}", http_method))?
            {
                "get" => HttpMethod::GET,
                "post" => HttpMethod::POST,
                "head" => HttpMethod::HEAD,
                "put" => HttpMethod::PUT,
                "delete" => HttpMethod::DELETE,
                other => bail!("unsupported http method: {}", other),
            };

            process_method(
                op.as_hash()
                    .ok_or_else(|| format_err!("non-hash op body: {:?}", op))?,
            )?;
        }
    }
    Ok(())
}

fn process_method(op: &Hash) -> Result<(), Error> {
    let mut current_keys = keys(op)?;

    println!("current_keys: {:?}", current_keys);

    current_keys.remove("summary");
    current_keys.remove("description");
    current_keys.remove("produces");
    current_keys.remove("consumes");
    current_keys.remove("tags");
    current_keys.remove("operationId");

    current_keys.remove("responses");
    if current_keys.remove("parameters") {
        get_vec(op, "parameters")?;
    }

    get_hash(op, "responses")?;

    ensure!(
        current_keys.is_empty(),
        "unrecognised keys: {:?}",
        current_keys
    );
    Ok(())
}

fn properties_to_fields(
    new_structs: &mut Vec<Struct>,
    required: &[&str],
    hash: &Hash,
) -> Result<Vec<Field>, Error> {
    let mut ret = Vec::new();
    let mut required: HashSet<&str> = required.into_iter().cloned().collect();

    for (name, field) in hash.into_iter() {
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

        let data_type = if current_keys.remove("properties") {
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
                        yaml.as_str().ok_or_else(|| {
                            format_err!("required param must be string, not {:?}", yaml)
                        })
                    })
                    .collect::<Result<Vec<&str>, Error>>()?
            } else {
                Vec::new()
            };

            let new_struct = Struct {
                fields: properties_to_fields(
                    new_structs,
                    &required,
                    get_hash(field, "properties")?,
                )?,
            };

            let new_id = new_structs.len();
            new_structs.push(new_struct);
            FieldType::Inner(new_id)
        } else if current_keys.remove("additionalProperties") {
            current_keys.remove("type");

            // ???

            // ?????
            current_keys.remove("default");
            FieldType::Unknown
        } else if current_keys.remove("allOf") {
            current_keys.remove("type"); // must be object?
            FieldType::Unknown
        } else if current_keys.remove("type") {
            match get_string(field, "type")? {
                "object" if current_keys.is_empty() => {
                    // TODO: Total bullshit. No idea what this should do.
                    FieldType::Unknown
                }
                "object" => bail!("{}: type object, but no properties", name),
                "array" => {
                    ensure!(
                        current_keys.remove("items"),
                        "{}: arrays must have items",
                        name
                    );
                    current_keys.remove("default");

                    FieldType::Unknown
                }
                "integer" => {
                    current_keys.remove("format");
                    current_keys.remove("minimum");
                    current_keys.remove("maximum");
                    current_keys.remove("default");

                    FieldType::Simple(DataType::Integer {
                        min: optional_integer(field, "minimum")?,
                        max: optional_integer(field, "maximum")?,
                        default: optional_integer(field, "default")?,
                        format: IntegerFormat::Unspecified,
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
                "string" => {
                    current_keys.remove("enum");
                    current_keys.remove("format");
                    current_keys.remove("default");
                    FieldType::Simple(DataType::String)
                }
                other => bail!("unimplemented def type: {}", other),
            }
        } else if 1 == current_keys.len() && current_keys.remove("$ref") {
            FieldType::Ref(get_string(field, "$ref")?.to_string())
        } else {
            bail!("bad type");
        };

        ensure!(
            current_keys.is_empty(),
            "unrecognised keys: {:?}",
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

fn keys(hash: &Hash) -> Result<HashSet<&str>, Error> {
    hash.keys()
        .map(|k| {
            k.as_str()
                .ok_or_else(|| format_err!("invalid non-string key in hash: {:?}", k))
        })
        .collect()
}

fn get<'h>(hash: &'h Hash, key: &str) -> Result<&'h Yaml, Error> {
    hash.get(&Yaml::String(key.to_string()))
        .ok_or_else(|| format_err!("key '{}' missing", key))
}

fn get_string<'h>(hash: &'h Hash, key: &str) -> Result<&'h str, Error> {
    get(hash, key).and_then(|y| {
        y.as_str()
            .ok_or_else(|| format_err!("key '{}' not string: {:?}", key, y))
    })
}

fn get_bool(hash: &Hash, key: &str) -> Result<bool, Error> {
    Ok(get(hash, key)
        .and_then(as_bool)
        .with_context(|_| format_err!("key: {}", key))?)
}

fn optional_bool(hash: &Hash, key: &str) -> Result<Option<bool>, Error> {
    Ok(get(hash, key)
        .ok()
        .map(as_bool)
        .invert()
        .with_context(|_| format_err!("key: {}", key))?)
}

fn as_bool(val: &Yaml) -> Result<bool, Error> {
    val.as_bool()
        .ok_or_else(|| format_err!("not bool: {:?}", val))
}

fn optional_number(hash: &Hash, key: &str) -> Result<Option<f64>, Error> {
    Ok(get(hash, key)
        .ok()
        .map(as_number)
        .invert()
        .with_context(|_| format_err!("key: {}", key))?)
}

fn as_number(val: &Yaml) -> Result<f64, Error> {
    val.as_f64()
        .or_else(|| val.as_i64().map(f64))
        .ok_or_else(|| format_err!("not number: {:?}", val))
}

fn optional_integer(hash: &Hash, key: &str) -> Result<Option<i64>, Error> {
    Ok(get(hash, key)
        .ok()
        .map(as_integer)
        .invert()
        .with_context(|_| format_err!("key: {}", key))?)
}

fn as_integer(val: &Yaml) -> Result<i64, Error> {
    val.as_i64()
        .ok_or_else(|| format_err!("not integer: {:?}", val))
}

fn get_vec<'h>(hash: &'h Hash, key: &str) -> Result<&'h Vec<Yaml>, Error> {
    get(hash, key).and_then(|y| {
        y.as_vec()
            .ok_or_else(|| format_err!("key '{}' not vec: {:?}", key, y))
    })
}

fn get_hash<'h>(hash: &'h Hash, key: &str) -> Result<&'h Hash, Error> {
    get(hash, key).and_then(|y| {
        y.as_hash()
            .ok_or_else(|| format_err!("key '{}' not hash: {:?}", key, y))
    })
}

enum HttpMethod {
    GET,
    POST,
    HEAD,
    PUT,
    DELETE,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        ::go().unwrap()
    }
}
