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

use std::io::Write;
use swagger::NamingType;

pub fn go<W: Write>(mut into: W) -> Result<(), Error> {
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

    render::render_definitions(&mut into, &definitions)?;

    render::render_endpoints(&mut into, &endpoints)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate tempfile_fast;

    #[test]
    fn test() {
        let mut temp = self::tempfile_fast::PersistableTempFile::new_in("demo/src").unwrap();
        ::go(&mut temp).unwrap();
        temp.persist_by_rename("demo/src/docker_gen.rs").unwrap();
    }
}
