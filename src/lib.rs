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

use swagger::NamingType;

pub fn go() -> Result<(), Error> {
    let doc = yaml_rust::YamlLoader::load_from_str(include_str!("../examples/docker.yaml"))?;
    let doc = &doc[0];

    let (endpoints, definitions) =
        swagger::load(
            doc["definitions"]
                .as_hash()
                .ok_or_else(|| format_err!("no definitions"))?,
            doc["paths"]
                .as_hash()
                .ok_or_else(|| format_err!("no paths"))?,
        ).with_context(|_| format_err!("loading full types from yaml"))?;

    for (name, naming) in definitions {
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
