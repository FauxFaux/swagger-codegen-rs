extern crate cast;
#[macro_use]
extern crate failure;
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

    let mut rendered_as = HashMap::new();

    let mut rendered = Vec::new();

    let (props, mut structs) = swagger::name::definitions(
        doc["definitions"]
            .as_hash()
            .ok_or_else(|| format_err!("no definitions"))?,
    ).with_context(|_| format_err!("processing definitions"))?;
    for p in props {
        render::render_top(&p, &mut rendered_as, &structs, &mut rendered)
            .with_context(|_| format_err!("rendering definition {}", p.name))?;
    }

    for s in &structs {
        println!("{:?}", s)
    }

    for p in swagger::paths::paths(
        doc["paths"]
            .as_hash()
            .ok_or_else(|| format_err!("no paths"))?,
        &mut structs,
    )? {
        println!("{:?}", p);
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
