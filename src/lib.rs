extern crate cast;
#[macro_use]
extern crate failure;
extern crate result;
extern crate yaml_rust;

mod swagger;

use failure::Error;
use failure::ResultExt;

pub fn go() -> Result<(), Error> {
    let doc = yaml_rust::YamlLoader::load_from_str(include_str!("../examples/docker.yaml"))?;
    let doc = &doc[0];

    let mut structs = Vec::new();

    for p in swagger::definitions::properties_to_fields(
        &mut structs,
        &[],
        doc["definitions"]
            .as_hash()
            .ok_or_else(|| format_err!("no definitions"))?,
    ).with_context(|_| format_err!("processing defintions"))?
    {
        println!("{:?}", p)
    }

    for s in &structs {
        println!("{:?}", s);
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
