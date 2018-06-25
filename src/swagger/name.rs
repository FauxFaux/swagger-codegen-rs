use std::collections::HashMap;
use std::collections::HashSet;

use failure::Error;
use failure::ResultExt;

use swagger::full::Defs;
use swagger::Endpoint;
use swagger::Field;
use swagger::FullType;
use swagger::NamedType;
use swagger::StructContext;
use NamingType;

pub fn to_named_types(
    endpoints: Vec<Endpoint<FullType>>,
    mut def_names: HashMap<NamingType, Vec<String>>,
) -> Result<(Vec<Endpoint<NamedType>>, HashMap<NamingType, String>), Error> {
    for endpoint in &endpoints {
        endpoint.visit_type(|t, name_hints| extract_names(&t, &name_hints, &mut def_names));
    }

    let mut banned_names = HashSet::new();
    'trying: loop {
        let mut used = HashSet::new();
        for possible_names in def_names.values() {
            let chosen = first_not_in(possible_names, &banned_names)?;

            if used.contains(chosen) {
                banned_names.insert(chosen.to_string());
                continue 'trying;
            }

            used.insert(chosen);
        }

        break;
    }

    let name_lookup = def_names
        .into_iter()
        .map(|(field, possible_names)| {
            first_not_in(&possible_names, &banned_names).map(|new| (field, new.to_string()))
        })
        .collect::<Result<HashMap<NamingType, String>, Error>>()?;

    let endpoints = endpoints
        .into_iter()
        .map(|e| e.map_type(|t| Ok(name_type(t, &name_lookup))))
        .collect::<Result<Vec<Endpoint<NamedType>>, Error>>()?;

    Ok((endpoints, name_lookup))
}

fn extract_names(
    t: &FullType,
    name_hints: &StructContext,
    def_names: &mut HashMap<NamingType, Vec<String>>,
) {
    match t {
        FullType::Fields(fields) => {
            def_names
                .entry(NamingType::Field(fields.clone()))
                .or_insert_with(|| Vec::new())
                .extend(name_hints.recommended_names());

            for field in fields {
                let mut name_hints = name_hints.clone();
                name_hints.id = name_hints.id.map(|id| format!("{}{}", id, field.name));
                extract_names(&field.data_type, &name_hints, def_names);
            }
        }
        FullType::Enum { values, default } => def_names
            .entry(NamingType::Enum(values.clone(), default.clone()))
            .or_insert_with(|| Vec::new())
            .extend(name_hints.recommended_names()),
        // TODO: could add extra hints here that we're in an array,
        // TODO: not really expecting multi-level arrays to be relevant
        FullType::Array { tee, .. } => extract_names(tee, name_hints, def_names),
        FullType::Simple(_) | FullType::Unknown => (),
    }
}

pub fn name_type(t: FullType, names: &HashMap<NamingType, String>) -> NamedType {
    match t {
        FullType::Fields(fields) => NamedType::Name(names[&NamingType::Field(fields)].to_string()),
        FullType::Enum { values, default } => {
            NamedType::Name(names[&NamingType::Enum(values, default)].to_string())
        }
        FullType::Array { tee, constraints } => NamedType::Array {
            tee: Box::new(name_type(*tee, names)),
            constraints,
        },
        FullType::Simple(simple) => NamedType::Simple(simple),
        FullType::Unknown => NamedType::Unknown,
    }
}

fn first_not_in<'s>(
    container: &'s [String],
    blacklist: &HashSet<String>,
) -> Result<&'s String, Error> {
    container
        .iter()
        .filter(|n| !blacklist.contains(*n))
        .next()
        .ok_or_else(|| format_err!("No remaining names: {:?}", container))
}
