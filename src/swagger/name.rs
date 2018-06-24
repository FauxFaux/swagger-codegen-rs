use std::collections::HashMap;
use std::collections::HashSet;

use failure::Error;
use failure::ResultExt;
use yaml_rust::yaml::Hash;

use swagger::definitions::properties_to_fields;
use swagger::Endpoint;
use swagger::Field;
use swagger::FullType;
use swagger::HttpMethod;
use swagger::Operation;
use swagger::Param;
use swagger::PartialType;
use swagger::Response;
use NamingType;

pub type Defs = HashMap<String, Field<PartialType>>;
pub type DefNames = HashMap<NamingType, Vec<String>>;
pub type Endpoints = Vec<Endpoint<FullType>>;

// TODO: this really should be like three methods, or an object or something
pub fn definitions(definitions: &Hash, paths: &Hash) -> Result<(DefNames, Endpoints), Error> {
    let definitions: Defs = properties_to_fields(&[], definitions)
        .with_context(|_| format_err!("processing definitions"))?
        .into_iter()
        .map(|field| (field.name.to_string(), field))
        .collect();

    let mut reverse = HashMap::with_capacity(definitions.len());

    for (name, field) in &definitions {
        match deref(&definitions, field.data_type.clone())? {
            FullType::Fields(fields) => reverse
                .entry(NamingType::Field(fields))
                .or_insert_with(|| HashSet::new())
                .insert(name.to_string()),
            FullType::Enum { values, default } => reverse
                .entry(NamingType::Enum(values, default))
                .or_insert_with(|| HashSet::new())
                .insert(name.to_string()),
            _ => false, // sigh
        };
    }

    let reverse: DefNames = reverse
        .into_iter()
        .map(|(k, v)| {
            let mut v: Vec<String> = v.into_iter().collect();
            v.sort();
            (k, v)
        })
        .collect();

    let endpoints = super::paths::paths(paths)?
        .into_iter()
        .map(|e| e.map_type(|t, _| deref(&definitions, t)))
        .collect::<Result<Endpoints, Error>>()?;

    Ok((reverse, endpoints))
}

fn deref(definitions: &Defs, data_type: PartialType) -> Result<FullType, Error> {
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

            deref(definitions, new_block)?
        }
        PartialType::Array { tee, constraints } => FullType::Array {
            tee: Box::new(deref(definitions, *tee)?),
            constraints,
        },
        PartialType::AllOf(inner) => {
            let mut new = Vec::new();
            for child in inner {
                match deref(definitions, child)? {
                    FullType::Fields(fields) => new.extend(fields),
                    FullType::Unknown => (),
                    other => bail!("can't all-of {:?}", other),
                }
            }
            FullType::Fields(new)
        }
        PartialType::Fields(fields) => FullType::Fields(
            fields
                .into_iter()
                .map(|f| f.map_type(|t| deref(definitions, t)))
                .collect::<Result<Vec<Field<FullType>>, Error>>()?,
        ),
        PartialType::Enum { values, default } => FullType::Enum { values, default },
        PartialType::Simple(data_type) => FullType::Simple(data_type.clone()),
        PartialType::Unknown => FullType::Unknown,
    })
}
