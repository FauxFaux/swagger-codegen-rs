use std::collections::HashMap;

use failure::Error;
use failure::ResultExt;
use yaml_rust::yaml::Hash;

use swagger::definitions::properties_to_fields;
use swagger::Field;
use swagger::FieldType;
use swagger::Struct;

pub fn definitions(definitions: &Hash) -> Result<(Vec<Field>, Vec<Struct>), Error> {
    let mut structs = Vec::new();

    let mut props = properties_to_fields(&mut structs, &[], definitions)
        .with_context(|_| format_err!("processing definitions"))?;

    let mut definitions = HashMap::with_capacity(props.len());

    for field in &props {
        if let FieldType::Inner(id) = field.data_type {
            definitions.insert(field.name.to_string(), id);
        }
    }

    for field in &mut props {
        let borrow_thoroughly_checked = match field.data_type {
            FieldType::Ref(ref id) => {
                ensure!(
                    id.starts_with("#/definitions/"),
                    "non-definitions ref: {}",
                    id
                );
                let id = id["#/definitions/".len()..].to_string();
                let id = *definitions
                    .get(&id)
                    .ok_or_else(|| format_err!("definition not found: {}", id))?;
                Some(FieldType::Inner(id))
            }
            _ => None,
        };

        if let Some(val) = borrow_thoroughly_checked {
            field.data_type = val;
        }
    }

    Ok((props, structs))
}
