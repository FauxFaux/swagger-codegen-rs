extern crate cast;
#[macro_use]
extern crate failure;
extern crate heck;
extern crate mime;
extern crate ordered_float;
extern crate result;
extern crate yaml_rust;

mod render;
mod swagger;

use std::collections::HashMap;

use failure::Error;
use failure::ResultExt;

use swagger::Field;
use swagger::FullType;
use swagger::NamedType;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum NamingType<T> {
    Field(Vec<Field<T>>),
    Enum(Vec<String>, Option<String>),
}

pub fn go() -> Result<(), Error> {
    let doc = yaml_rust::YamlLoader::load_from_str(include_str!("../examples/docker.yaml"))?;
    let doc = &doc[0];

    let (endpoints, def_names) = swagger::full::load_endpoints_and_names(
        doc["definitions"]
            .as_hash()
            .ok_or_else(|| format_err!("no definitions"))?,
        doc["paths"]
            .as_hash()
            .ok_or_else(|| format_err!("no paths"))?,
    ).with_context(|_| format_err!("loading full types from yaml"))?;

    let (endpoints, name_lookup) = swagger::name::to_named_types(endpoints, def_names)?;

    let mut render_order = name_lookup
        .iter()
        .map(|(k, v)| (v, k))
        .collect::<Vec<(&String, &NamingType<FullType>)>>();

    render_order.sort_by_key(|(k, _)| k.to_string());

    //let render_order = render_order.into_iter().map(|(name, naming)|
    //    (name, match naming {
    //        NamingType::Field(fields) => NamingType::Field(fields.into_iter().map(|f| f.map_type(|t| {
    //            Ok(swagger::name::name_type(t, &name_lookup))
    //        })).collect::<Result<Vec<Field<NamedType>>, Error>>()?)
    //    })).collect::<Result<Vec<(&String, &NamingType<NamedType>)>, Error>>()?;

    let mut nicer_order = HashMap::new();

    for (name, naming) in render_order {
        nicer_order.insert(
            name.to_string(),
            match naming {
                NamingType::Field(fields) => NamingType::Field(
                    fields
                        .into_iter()
                        .map(|f| {
                            f.clone()
                                .map_type(|t| Ok(swagger::name::name_type(t, &name_lookup)))
                        })
                        .collect::<Result<Vec<Field<NamedType>>, Error>>()?,
                ),
                NamingType::Enum(values, default) => {
                    NamingType::Enum(values.to_vec(), default.clone())
                }
            },
        );
    }

    for (name, naming) in nicer_order {
        use heck::MixedCase;

        match naming {
            NamingType::Field(fields) => {
                println!("struct {} {{", name);
                for field in fields {
                    println!(
                        "    {}: {},",
                        field.name.to_mixed_case(),
                        render::render(&field.data_type,)
                    );
                }
                println!("}}");
            }

            NamingType::Enum(values, _default) => {
                println!("enum {} {{", name);
                for value in values {
                    println!("    {},", value);
                }
                println!("}}");
            }
        }
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
