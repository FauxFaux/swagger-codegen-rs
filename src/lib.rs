extern crate cast;
#[macro_use]
extern crate failure;
extern crate mime;
extern crate result;
extern crate yaml_rust;

mod render;
mod swagger;

use std::collections::HashMap;

use failure::Error;
use failure::ResultExt;

pub fn go() -> Result<(), Error> {
    let doc = yaml_rust::YamlLoader::load_from_str(include_str!("../examples/docker.yaml"))?;
    let doc = &doc[0];

    let (endpoints, structs) = swagger::name::definitions(
        doc["definitions"]
            .as_hash()
            .ok_or_else(|| format_err!("no definitions"))?,
        doc["paths"]
            .as_hash()
            .ok_or_else(|| format_err!("no paths"))?,
    ).with_context(|_| format_err!("processing definitions"))?;

    for endpoint in endpoints {
        for (method, op) in endpoint.ops {
            for param in op.params {
                println!(
                    "{} {:?} {} {:?} {:?}",
                    endpoint.path_url,
                    method,
                    param.name,
                    param.loc,
                    render::render_type(
                        &param.name,
                        &param.param_type,
                        &mut Vec::new()
                    ).with_context(|_| format_err!("rendering param {}", param.name))?
                );
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
