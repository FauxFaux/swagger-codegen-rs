use std::collections::HashMap;

use failure::Error;
use failure::ResultExt;
use yaml_rust::yaml::Hash;

use swagger::definitions::properties_to_fields;
use swagger::Field;
use swagger::FieldType;
use swagger::Struct;

type Defs = HashMap<String, FieldType>;

pub fn definitions(definitions: &Hash) -> Result<(Vec<Field>, Vec<Struct>), Error> {
    let mut structs = Vec::new();

    let mut props = properties_to_fields(&mut structs, &[], definitions)
        .with_context(|_| format_err!("processing definitions"))?;

    let mut definitions: Defs = HashMap::with_capacity(props.len());

    for field in &props {
        definitions.insert(field.name.to_string(), field.data_type.clone());
    }

    deref_fields(&definitions, &mut props)?;

    for s in &mut structs {
        deref_fields(&definitions, &mut s.fields)?;
    }

    Ok((props, structs))
}

fn deref_fields(definitions: &Defs, fields: &mut [Field]) -> Result<(), Error> {
    for field in fields {
        if let Some(new_data_type) = deref(&definitions, &field.data_type)? {
            field.data_type = new_data_type;
        }
    }
    Ok(())
}

fn deref(
    definitions: &Defs,
    data_type: &FieldType,
) -> Result<Option<FieldType>, Error> {
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

        _ => None,
    })
}
