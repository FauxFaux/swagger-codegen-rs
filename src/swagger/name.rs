use std::collections::HashMap;

use failure::Error;
use failure::ResultExt;
use yaml_rust::yaml::Hash;

use swagger::definitions::properties_to_fields;
use swagger::Endpoint;
use swagger::Field;
use swagger::PartialType;
use swagger::Struct;

type Defs = HashMap<String, Field<PartialType>>;

pub fn definitions(definitions: &Hash, paths: &Hash) -> Result<Vec<Endpoint>, Error> {
    let definitions: Defs = properties_to_fields(&[], definitions)
        .with_context(|_| format_err!("processing definitions"))?
        .into_iter()
        .map(|field| (field.name.to_string(), field))
        .collect();

    let mut endpoints = super::paths::paths(paths)?;

    for e in &mut endpoints {
        for op in e.ops.values_mut() {
            for param in &mut op.params {
                if let Some(new) = deref(&definitions, &param.param_type)? {
                    param.param_type = new;
                }

                let maybe_new = if let PartialType::AllOf(ref inner) = param.param_type {
                    Some(PartialType::Fields(flatten_fields(inner)?))
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

    Ok((endpoints))
}

fn flatten_fields(inner: &[PartialType]) -> Result<Vec<Field<PartialType>>, Error> {
    let mut all_fields = Vec::new();
    for child in inner {
        match child {
            PartialType::Fields(fields) => all_fields.extend(fields.iter().cloned()),
            PartialType::Unknown => all_fields.push(Field {
                name: "_".to_string(),
                data_type: PartialType::Unknown,
                description: String::new(),
                nullable: None,
                required: false,
            }),
            other => bail!("unsupported flatten: {:?}", other),
        }
    }
    Ok(all_fields)
}

fn maybe_transform_fields<F>(fields: &mut [Field<PartialType>], mut func: F) -> Result<(), Error>
where
    F: FnMut(&mut Field<PartialType>) -> Result<Option<PartialType>, Error>,
{
    for field in fields {
        if let Some(new_data_type) = func(field)? {
            field.data_type = new_data_type;
        }
    }
    Ok(())
}

/// `Some(new)` if it needs to change, otherwise `None`
fn deref(definitions: &Defs, data_type: &PartialType) -> Result<Option<PartialType>, Error> {
    Ok(match data_type {
        PartialType::Ref(ref id) => {
            ensure!(
                id.starts_with("#/definitions/"),
                "non-definitions ref: {}",
                id
            );
            let id = id["#/definitions/".len()..].to_string();
            let new_block: PartialType = definitions
                .get(&id)
                .ok_or_else(|| format_err!("definition not found: {}", id))?
                .data_type
                .clone();

            Some(deref(definitions, &new_block)?.unwrap_or(new_block))
        }
        PartialType::Array { tee, constraints } => if let Some(new) = deref(definitions, tee)? {
            // this would potentially be less horribly ugly if there was real struct, not an enum-embedded struct
            Some(PartialType::Array {
                tee: Box::new(new),
                constraints: *constraints,
            })
        } else {
            None
        },
        PartialType::AllOf(inner) => {
            let mut new = Vec::new();
            for child in inner {
                new.push(deref(definitions, child)?.unwrap_or(child.clone()));
            }
            // TODO: would love to unpack the error handling into a map here
            // TODO: can we work out if we didn't change anything, then not copy? Irrelevant.
            Some(PartialType::AllOf(new))
        }

        _ => None,
    })
}
