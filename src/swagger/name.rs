use std::collections::HashMap;

use failure::Error;
use failure::ResultExt;
use yaml_rust::yaml::Hash;

use swagger::definitions::properties_to_fields;
use swagger::Field;
use swagger::FieldType;
use swagger::Struct;
use swagger::Endpoint;

type Defs = HashMap<String, FieldType>;

pub fn definitions(definitions: &Hash, paths: &Hash) -> Result<(Vec<Field>, Vec<Struct>), Error> {
    let mut structs = Vec::new();

    let mut props = properties_to_fields(&mut structs, &[], definitions)
        .with_context(|_| format_err!("processing definitions"))?;

    let mut endpoints = super::paths::paths(paths, &mut structs)?;

    let mut definitions: Defs = HashMap::with_capacity(props.len());

    for field in &props {
        definitions.insert(field.name.to_string(), field.data_type.clone());
    }

    #[cfg(maybe_optimisation)]
    maybe_transform_fields(&mut props, |f| deref(&definitions, &f.data_type))?;

    for e in &mut endpoints {
        for op in e.ops.values_mut() {
            for param in &mut op.params {
                if let Some(new) = deref(&definitions, &param.param_type)? {
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

    //maybe_transform_fields(&mut props, |f| denest(&mut structs, &f.data_type))?;

    Ok((props, structs))
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

fn denest(structs: &mut Vec<Struct>, data_type: &FieldType) -> Result<Option<FieldType>, Error> {
    Ok(match data_type {
        FieldType::AllOf(inner) => {
            let mut all_fields = Vec::new();
            for child in inner {
                all_fields.extend(
                    match child {
                        FieldType::Inner(id) => structs[*id].fields.iter(),
                        other => bail!("unsupported denest: {:?}", other),
                    }.cloned(),
                );
            }
            let id = structs.len();
            structs.push(Struct { fields: all_fields });
            Some(FieldType::Inner(id))
        }
        _ => None,
    })
}
