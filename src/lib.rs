extern crate cast;
#[macro_use]
extern crate failure;
extern crate mime;
extern crate result;
extern crate yaml_rust;

mod float;
mod render;
mod swagger;

use failure::Error;
use failure::ResultExt;
use swagger::Endpoint;

pub fn go() -> Result<(), Error> {
    let doc = yaml_rust::YamlLoader::load_from_str(include_str!("../examples/docker.yaml"))?;
    let doc = &doc[0];

    let mut all_structs = Vec::new();

    let endpoints = swagger::name::definitions(
        doc["definitions"]
            .as_hash()
            .ok_or_else(|| format_err!("no definitions"))?,
        doc["paths"]
            .as_hash()
            .ok_or_else(|| format_err!("no paths"))?,
    ).with_context(|_| format_err!("processing definitions"))?
        .into_iter()
        .map(|e| {
            e.map_type(|t| {
                let id = all_structs.len();
                all_structs.push(t);
                Ok(id)
            })
        })
        .collect::<Result<Vec<Endpoint<usize>>, Error>>()?;

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
                        &all_structs[param.param_type],
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
