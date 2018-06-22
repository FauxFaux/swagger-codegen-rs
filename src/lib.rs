extern crate cast;
#[macro_use]
extern crate failure;
extern crate heck;
extern crate mime;
extern crate result;
extern crate yaml_rust;

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

    let mut structs = HashMap::new();

    let (def_names, endpoints) = swagger::name::definitions(
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
                    if !def_names.contains_key(fields) {
                        structs
                            .entry(fields.clone())
                            .or_insert_with(|| Vec::new())
                            .extend(name_hints.recommended_names());
                    }
                }

                Ok(t)
            })
        })
        .collect::<Result<Vec<Endpoint<FullType>>, Error>>()?;

    for (key, val) in structs {
        println!("{:?} {:?}", val, key);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        ::go().unwrap()
    }
}
