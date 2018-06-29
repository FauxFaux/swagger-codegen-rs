extern crate cast;
#[macro_use]
extern crate failure;
extern crate heck;
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

    render::render_definitions(&mut stdout, &definitions)?;

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
