use std::collections::HashMap;

use failure::Error;
use failure::ResultExt;
use yaml_rust::yaml::Hash;

use swagger::definitions::properties_to_fields;
use swagger::Endpoint;
use swagger::Field;
use swagger::FieldType;
use swagger::Struct;

type Defs = HashMap<String, Field>;

pub fn definitions(
    definitions: &Hash,
    paths: &Hash,
) -> Result<(Vec<Endpoint>, Vec<Struct>), Error> {
    let mut structs = Vec::new();

    let definitions: Defs = properties_to_fields(&mut structs, &[], definitions)
        .with_context(|_| format_err!("processing definitions"))?
        .into_iter()
        .map(|field| (field.name.to_string(), field))
        .collect();

    let mut endpoints = super::paths::paths(paths, &mut structs)?;

    for e in &mut endpoints {
        for op in e.ops.values_mut() {
            for param in &mut op.params {
                if let Some(new) = deref(&definitions, &param.param_type)? {
                    param.param_type = new;
                }

                let maybe_new = if let FieldType::AllOf(ref inner) = param.param_type {
                    let fields = flatten_fields(&structs, inner)?;
                    let id = structs.len();
                    structs.push(Struct { fields });
                    Some(FieldType::Inner(id))
                } else {
                    None
                };

                if let Some(new) = maybe_new {
                    param.param_type = new;
                }
            }

            for resp in op.responses.values_mut() {
                // BORROW CHECKER prevents
                // if let Some(ref resp_type) = r.resp_type { r.resp_type = ..
                if resp.resp_type.is_none() {
                    continue;
                }
                if let Some(new) = deref(&definitions, resp.resp_type.as_ref().unwrap())? {
                    resp.resp_type = Some(new);
                }
            }
        }
    }

    for s in &mut structs {
        maybe_transform_fields(&mut s.fields, |f| deref(&definitions, &f.data_type))?;
    }

    loop {
        let old_structs = structs.clone();

        let mut new_structs = Vec::new();

        for s in &mut structs {
            for f in &mut s.fields {
                let maybe_new = if let FieldType::AllOf(ref inner) = f.data_type {
                    let fields = flatten_fields(&old_structs, inner)?;
                    let id = old_structs.len() + new_structs.len();
                    new_structs.push(Struct { fields });
                    Some(FieldType::Inner(id))
                } else {
                    None
                };

                if let Some(new) = maybe_new {
                    f.data_type = new;
                }
            }
        }

        if new_structs.is_empty() {
            break;
        }

        structs.extend(new_structs);
    }

    Ok((endpoints, structs))
}

fn flatten_fields(structs: &[Struct], inner: &[FieldType]) -> Result<Vec<Field>, Error> {
    let mut all_fields = Vec::new();
    for child in inner {
        match child {
            FieldType::Inner(id) => all_fields.extend(structs[*id].fields.iter().cloned()),
            FieldType::Unknown => all_fields.push(Field {
                name: "_".to_string(),
                data_type: FieldType::Unknown,
                description: String::new(),
                nullable: None,
                required: false,
            }),
            other => bail!("unsupported flatten: {:?}", other),
        }
    }
    Ok(all_fields)
}

fn maybe_transform_fields<F>(fields: &mut [Field], mut func: F) -> Result<(), Error>
where
    F: FnMut(&mut Field) -> Result<Option<FieldType>, Error>,
{
    for field in fields {
        if let Some(new_data_type) = func(field)? {
            field.data_type = new_data_type;
        }
    }
    Ok(())
}

/// `Some(new)` if it needs to change, otherwise `None`
fn deref(definitions: &Defs, data_type: &FieldType) -> Result<Option<FieldType>, Error> {
    Ok(match data_type {
        FieldType::Ref(ref id) => {
            ensure!(
                id.starts_with("#/definitions/"),
                "non-definitions ref: {}",
                id
            );
            let id = id["#/definitions/".len()..].to_string();
            let new_block: FieldType = definitions
                .get(&id)
                .ok_or_else(|| format_err!("definition not found: {}", id))?
                .data_type
                .clone();

            Some(deref(definitions, &new_block)?.unwrap_or(new_block))
        }
        FieldType::Array {
            item_type,
            min_items,
            max_items,
            null_default,
        } => if let Some(new) = deref(definitions, item_type)? {
            // this would potentially be less horribly ugly if there was real struct, not an enum-embedded struct
            Some(FieldType::Array {
                item_type: Box::new(new),
                min_items: *min_items,
                max_items: *max_items,
                null_default: *null_default,
            })
        } else {
            None
        },
        FieldType::AllOf(inner) => {
            let mut new = Vec::new();
            for child in inner {
                new.push(deref(definitions, child)?.unwrap_or(child.clone()));
            }
            // TODO: would love to unpack the error handling into a map here
            // TODO: can we work out if we didn't change anything, then not copy? Irrelevant.
            Some(FieldType::AllOf(new))
        }

        _ => None,
    })
}
