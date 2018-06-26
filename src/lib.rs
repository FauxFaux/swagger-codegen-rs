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

use failure::Error;
use failure::ResultExt;

use swagger::Field;

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

    for (name, naming) in name_lookup {
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
