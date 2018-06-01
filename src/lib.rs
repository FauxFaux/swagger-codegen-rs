#[macro_use]
extern crate failure;
extern crate yaml_rust;

use failure::Error;
use std::collections::HashSet;
use yaml_rust::yaml::Hash;
use yaml_rust::Yaml;

pub fn go() -> Result<(), Error> {
    let doc = yaml_rust::YamlLoader::load_from_str(include_str!("../examples/docker.yaml"))?;
    let doc = &doc[0];

    println!(
        "{:?}",
        keys(doc
            .as_hash()
            .ok_or_else(|| format_err!("doc must be a hash"))?)?
    );

    for (def_name, def) in doc["definitions"]
        .as_hash()
        .ok_or_else(|| format_err!("no definitions"))?
        .into_iter()
    {
        let def_name: &str = def_name
            .as_str()
            .ok_or_else(|| format_err!("non-string definition name: {:?}", def_name))?;
        let def: &Hash = def
            .as_hash()
            .ok_or_else(|| format_err!("non-hash definition body"))?;

        let mut current_keys = keys(def)?;

        if (current_keys.remove("description")) {
            get_string(def, "description")?;
        }
        current_keys.remove("example");
        current_keys.remove("required");
        current_keys.remove("x-nullable");

        if current_keys.remove("allOf") {
            current_keys.remove("type"); // must be object?
        } else if current_keys.remove("type") {
            match get_string(def, "type")? {
                "object" => {
                    ensure!(
                        current_keys.remove("properties")
                            || current_keys.remove("additionalProperties"),
                        "objects must have properties"
                    );
                }
                "array" => {
                    ensure!(current_keys.remove("items"), "arrays must have items");
                }
                "string" => {
                    ensure!(current_keys.remove("enum"), "strings must be enums?");
                    current_keys.remove("default");
                }
                other => bail!("unimplemented def type: {}", other),
            }
        } else if current_keys.remove("properties") {
            // fake object?
        }

        ensure!(
            current_keys.is_empty(),
            "unrecognised keys: {:?}",
            current_keys
        );
    }
    let paths = &doc["paths"];

    Ok(())
}

fn keys(hash: &Hash) -> Result<HashSet<&str>, Error> {
    hash.keys()
        .map(|k| {
            k.as_str()
                .ok_or_else(|| format_err!("invalid non-string key in hash: {:?}", k))
        })
        .collect()
}

fn get_string<'h>(hash: &'h Hash, key: &str) -> Result<&'h str, Error> {
    hash.get(&Yaml::String(key.to_string()))
        .ok_or_else(|| format_err!("key '{}' missing", key))
        .and_then(|y| {
            y.as_str()
                .ok_or_else(|| format_err!("key '{}' not string: {:?}", key, y))
        })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        ::go().unwrap()
    }
}
