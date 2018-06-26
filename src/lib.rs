extern crate cast;
#[macro_use]
extern crate failure;
extern crate heck;
extern crate ordered_float;
extern crate result;
extern crate yaml_rust;

mod render;
mod swagger;

use std::io::Write;

use failure::Error;
use failure::ResultExt;

use swagger::NamingType;

pub fn go() -> Result<(), Error> {
    let doc = yaml_rust::YamlLoader::load_from_str(include_str!("../examples/docker.yaml"))?;
    let doc = &doc[0];

    let stdout = ::std::io::stdout();
    let mut stdout = stdout.lock();

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
        use heck::SnakeCase;

        match naming {
            NamingType::Field(fields) => {
                writeln!(stdout, "struct {} {{", name)?;
                for field in fields {
                    writeln!(
                        stdout,
                        "    {}: {},",
                        field.name.to_snake_case(),
                        render::render(&field.data_type,)
                    )?;
                }
                writeln!(stdout, "}}")?;
            }

            NamingType::Enum(values, default) => {
                render::render_enum(&mut stdout, &name, &values, default.as_ref())?
            }
        }
    }

    render::render_endpoints(&mut stdout, &endpoints)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        ::go().unwrap()
    }
}
