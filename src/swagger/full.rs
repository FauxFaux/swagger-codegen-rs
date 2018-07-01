use std::collections::HashMap;
use std::collections::HashSet;

use failure::Error;
use failure::ResultExt;
use yaml_rust::yaml::Hash;

use swagger::Endpoint;
use swagger::Field;
use swagger::FullType;
use swagger::NamingType;
use swagger::PartialType;

type Defs = HashMap<String, Field<PartialType>>;
type DefNames = HashMap<NamingType<FullType>, Vec<String>>;
type Endpoints = Vec<Endpoint<FullType>>;

pub(super) fn load_endpoints_and_names(
    definitions: &Hash,
    paths: &Hash,
) -> Result<(Endpoints, DefNames), Error> {
    let definitions: Defs = super::partial_definitions::properties_to_fields(&[], definitions)
        .with_context(|_| format_err!("processing definitions"))?
        .into_iter()
        .map(|field| (field.name.to_string(), field))
        .collect();

    let endpoints = super::partial_paths::paths(paths)
        .with_context(|_| format_err!("processing paths"))?
        .into_iter()
        .map(|e| e.map_type(|t| deref(&definitions, t)))
        .collect::<Result<Endpoints, Error>>()?;

    Ok((endpoints, reverse_definitions(&definitions)?))
}

fn reverse_definitions(
    definitions: &HashMap<String, Field<PartialType>>,
) -> Result<HashMap<NamingType<FullType>, Vec<String>>, Error> {
    let mut reverse = HashMap::with_capacity(definitions.len());

    for (name, field) in definitions {
        match deref(definitions, field.data_type.clone())? {
            FullType::Fields(fields) => reverse
                .entry(NamingType::Field(fields))
                .or_insert_with(HashSet::new)
                .insert(name.to_string()),
            FullType::Enum { values, default } => reverse
                .entry(NamingType::Enum(values, default))
                .or_insert_with(HashSet::new)
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

    Ok(reverse)
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

            let mut seen = HashSet::new();
            let mut dedup = Vec::new();
            for field in new {
                if !seen.insert(field.name.to_string()) {
                    // TODO: validation, merge attributes, etc.
                    // TODO: No, really, this needs doing.
                }
                dedup.push(field);
            }

            FullType::Fields(dedup)
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
