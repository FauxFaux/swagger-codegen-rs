extern crate cast;
#[macro_use]
extern crate failure;
extern crate heck;
extern crate mime;
extern crate result;
extern crate yaml_rust;

use std::collections::HashSet;

mod float;
mod render;
mod swagger;

use failure::Error;
use failure::ResultExt;
use std::collections::HashMap;
use swagger::Endpoint;
use swagger::FullType;

pub fn go() -> Result<(), Error> {
    let doc = yaml_rust::YamlLoader::load_from_str(include_str!("../examples/docker.yaml"))?;
    let doc = &doc[0];

    let (mut def_names, endpoints) = swagger::name::definitions(
        doc["definitions"]
            .as_hash()
            .ok_or_else(|| format_err!("no definitions"))?,
        doc["paths"]
            .as_hash()
            .ok_or_else(|| format_err!("no paths"))?,
    ).with_context(|_| format_err!("processing definitions"))?;

    endpoints
        .into_iter()
        .map(|e| {
            e.map_type(|t, name_hints| {
                if let FullType::Fields(fields) = &t {
                    def_names
                        .entry(fields.clone())
                        .or_insert_with(|| Vec::new())
                        .extend(name_hints.recommended_names());
                }

                Ok(t)
            })
        })
        .collect::<Result<Vec<Endpoint<FullType>>, Error>>()?;

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

    let banned_names = banned_names;

    for (key, val) in def_names {
        println!("{}", first_not_in(&val, &banned_names)?);
    }

    Ok(())
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

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        ::go().unwrap()
    }
}
