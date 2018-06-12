use cast::u16;
use failure::Error;
use yaml_rust::yaml::Hash;

use super::*;

pub fn paths(paths: &Hash, new_structs: &mut Vec<Struct>) -> Result<(), Error> {
    for (path_url, path) in paths.into_iter() {
        let path_url: &str = path_url
            .as_str()
            .ok_or_else(|| format_err!("non-string path url: {:?}", path_url))?;
        let path: &Hash = path
            .as_hash()
            .ok_or_else(|| format_err!("non-hash path body"))?;

        process_methods(path, new_structs)
            .with_context(|_| format_err!("processing path: {:?}", path_url))?;
    }
    Ok(())
}

fn process_methods(path: &Hash, new_structs: &mut Vec<Struct>) -> Result<(), Error> {
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
            new_structs,
        ).with_context(|_| format_err!("processing {:?}", http_method))?;
    }

    Ok(())
}

fn process_method(op: &Hash, new_structs: &mut Vec<Struct>) -> Result<(), Error> {
    let mut current_keys = keys(op)?;

    current_keys.remove("summary");
    current_keys.remove("description");
    current_keys.remove("produces");
    current_keys.remove("consumes");
    current_keys.remove("tags");
    current_keys.remove("operationId");

    if current_keys.remove("parameters") {
        for param in get_vec(op, "parameters")? {
            process_param(
                param
                    .as_hash()
                    .ok_or_else(|| format_err!("non-hash parameter"))?,
                new_structs,
            )?;
        }
    }

    current_keys.remove("responses");
    for (code, resp) in get_hash(op, "responses")? {
        let code = u16(as_integer(code)?)?;
        let resp: &Hash = resp
            .as_hash()
            .ok_or_else(|| format_err!("non-hash response {}", code))?;
        process_response(resp, new_structs).with_context(|_| format_err!("response {}", code))?;
    }

    ensure!(
        current_keys.is_empty(),
        "unrecognised keys: {:?}",
        current_keys
    );
    Ok(())
}

fn process_param(param: &Hash, new_structs: &mut Vec<Struct>) -> Result<(), Error> {
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

    let required = if current_keys.remove("required") {
        Some(get_bool(param, "required"))
    } else {
        None
    };

    let param_type = if current_keys.remove("schema") {
        let schema = get_hash(param, "schema")?;
        let mut schema_keys = keys(schema)?;
        schema_keys.remove("example");

        let field_result = definitions::field_type(schema, &mut schema_keys, new_structs);

        ensure!(
            schema_keys.is_empty(),
            "unrecognised schema keys: {:?}",
            schema_keys
        );

        field_result
    } else {
        definitions::field_type(param, &mut current_keys, new_structs)
    }.with_context(|_| format_err!("finding type of {:?}", name))?;

    ensure!(
        current_keys.is_empty(),
        "{:?}: unrecognised keys: {:?}",
        name,
        current_keys
    );

    Ok(())
}

fn process_response(resp: &Hash, new_structs: &mut Vec<Struct>) -> Result<(), Error> {
    let mut current_keys = keys(resp)?;
    current_keys.remove("description");
    current_keys.remove("examples");

    if current_keys.remove("headers") {
        for (header_name, header) in get_hash(resp, "headers")? {
            let header_name = as_str(header_name)?;
            let header = header
                .as_hash()
                .ok_or_else(|| format_err!("non-hash header: {:?}", header))?;
            let mut header_keys = keys(header)?;
            header_keys.remove("description");
            let header_type = definitions::field_type(header, &mut header_keys, new_structs)?;
            ensure!(
                header_keys.is_empty(),
                "unsupported header keys: {:?}",
                header_keys
            );
        }
    }

    if current_keys.remove("schema") {
        let schema = get_hash(resp, "schema")?;
        let mut schema_keys = keys(schema)?;
        definitions::field_type(schema, &mut schema_keys, new_structs)?;
    }

    ensure!(
        current_keys.is_empty(),
        "unrecognised response body keys: {:?}",
        current_keys
    );

    Ok(())
}
