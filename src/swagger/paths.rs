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

        println!("{}", path_url);

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
            )?;
        }
    }
    Ok(())
}

fn process_method(op: &Hash) -> Result<(), Error> {
    let mut current_keys = keys(op)?;

    println!("current_keys: {:?}", current_keys);

    current_keys.remove("summary");
    current_keys.remove("description");
    current_keys.remove("produces");
    current_keys.remove("consumes");
    current_keys.remove("tags");
    current_keys.remove("operationId");

    current_keys.remove("responses");
    if current_keys.remove("parameters") {
        get_vec(op, "parameters")?;
    }

    get_hash(op, "responses")?;

    ensure!(
        current_keys.is_empty(),
        "unrecognised keys: {:?}",
        current_keys
    );
    Ok(())
}
