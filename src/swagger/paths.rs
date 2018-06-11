use failure::Error;
use yaml_rust::yaml::Hash;

use super::*;

pub fn paths(paths: &Hash) -> Result<(), Error> {
    for (path_url, path) in paths.into_iter() {
        let path_url: &str = path_url
            .as_str()
            .ok_or_else(|| format_err!("non-string path url: {:?}", path_url))?;
        let path: &Hash = path
            .as_hash()
            .ok_or_else(|| format_err!("non-hash path body"))?;

        process_methods(path).with_context(|_| format_err!("processing path: {:?}", path_url))?;
    }
    Ok(())
}

fn process_methods(path: &Hash) -> Result<(), Error> {
    for (http_method, op) in path.into_iter() {
        let http_method = match http_method
            .as_str()
            .ok_or_else(|| format_err!("non-string http method: {:?}", http_method))?
        {
            "get" => HttpMethod::GET,
            "post" => HttpMethod::POST,
            "head" => HttpMethod::HEAD,
            "put" => HttpMethod::PUT,
            "delete" => HttpMethod::DELETE,
            other => bail!("unsupported http method: {}", other),
        };

        process_method(
            op.as_hash()
                .ok_or_else(|| format_err!("non-hash op body: {:?}", op))?,
        ).with_context(|_| format_err!("processing {:?}", http_method))?;
    }

    Ok(())
}

fn process_method(op: &Hash) -> Result<(), Error> {
    let mut current_keys = keys(op)?;

    current_keys.remove("summary");
    current_keys.remove("description");
    current_keys.remove("produces");
    current_keys.remove("consumes");
    current_keys.remove("tags");
    current_keys.remove("operationId");

    current_keys.remove("responses");
    if current_keys.remove("parameters") {
        for param in get_vec(op, "parameters")? {
            process_param(
                param
                    .as_hash()
                    .ok_or_else(|| format_err!("non-hash parameter"))?,
            )?;
        }
    }

    get_hash(op, "responses")?;

    ensure!(
        current_keys.is_empty(),
        "unrecognised keys: {:?}",
        current_keys
    );
    Ok(())
}

fn process_param(param: &Hash) -> Result<(), Error> {
    let mut current_keys = keys(param)?;

    current_keys.remove("name");
    current_keys.remove("in");

    let name = get_string(param, "name")?;
    let loc = match get_string(param, "in")? {
        "query" => (),
        "body" => (),
        "path" => (),
        "header" => (),
        other => bail!("invalid `in` location: {}", other),
    };

    let description = if current_keys.remove("description") {
        get_string(param, "description")?
    } else {
        ""
    };

    current_keys.remove("required"); // TODO

    if current_keys.remove("schema") {
        // TODO
    } else {
        definitions::field_type(param, &mut current_keys, &mut Vec::new())
            .with_context(|_| format_err!("finding type of {}", name))?; // TODO: new_structs
    }

    ensure!(
        current_keys.is_empty(),
        "{:?}: unrecognised keys: {:?}",
        name,
        current_keys
    );

    Ok(())
}
