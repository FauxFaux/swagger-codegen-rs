use std::collections::HashSet;
use std::io::Write;

use cast::i64;
use failure::Error;
use failure::ResultExt;

use swagger::DataType;
use swagger::Endpoint;
use swagger::Field;
use swagger::HttpMethod;
use swagger::IntegerFormat;
use swagger::NamedType;
use swagger::NamingType;
use swagger::Operation;
use swagger::ParamLocation;
use swagger::Response;

pub fn render_definitions<W: Write>(
    mut into: W,
    definitions: &[(String, NamingType<NamedType>)],
) -> Result<(), Error> {
    for (name, naming) in definitions {
        match naming {
            NamingType::Field(fields) => render_struct(&mut into, &name, &fields)?,
            NamingType::Enum(values, default) => {
                render_enum(&mut into, &name, &values, default.as_ref())?
            }
        }
        writeln!(into)?;
    }

    Ok(())
}

pub fn render(t: &NamedType) -> String {
    match t {
        NamedType::Name(name) => name.to_string(),
        NamedType::Simple(simple) => render_simple(simple),
        NamedType::Array { tee, .. } => format!("Vec<{}>", render(tee)),
        NamedType::Unknown => "::serde_json::Value".to_string(),
    }
}

pub fn render_ref(t: &NamedType) -> String {
    // TODO: named types could be passed by value?
    // TODO: some simple types (like dates) could probably be passed by ref
    match t {
        NamedType::Name(name) => format!("&{}", name),
        NamedType::Simple(DataType::String { .. })
        | NamedType::Simple(DataType::MatchString { .. }) => "&str".to_string(),
        NamedType::Simple(simple) => render_simple(simple),
        NamedType::Array { tee, .. } => format!("&[{}]", render(tee)),
        NamedType::Unknown => "::serde_json::Value".to_string(),
    }
}

fn render_simple(simple: &DataType) -> String {
    use self::DataType::*;
    use self::IntegerFormat::*;

    match simple {
        Bool { .. } => "bool".to_string(),
        String { .. } | MatchString { .. } => "String".to_string(),
        Number { .. } => "f64".to_string(),
        IpAddr => "::std::net::IpAddr".to_string(),
        DateTime => "::chrono::DateTime<::chrono::Utc>".to_string(),
        Json => "::serde_json::Value".to_string(),
        Binary => "(/* binary */)".to_string(),
        Integer {
            min, max, format, ..
        } => {
            let unsigned = min.unwrap_or(-1) >= 0 || match format {
                U8 | U16 | U32 | U64 => true,
                _ => false,
            };

            let format_bits = match format {
                U8 | I8 => 8,
                U16 | I16 => 16,
                U32 | I32 => 32,
                U64 | I64 => 64,
                Unspecified => 64,
            };

            // TODO: should really cope with signs here
            let max_bits = if let Some(max) = *max {
                if max > i64(::std::i32::MAX) {
                    64
                } else if max > i64(::std::i16::MAX) {
                    32
                } else if max > i64(::std::i8::MAX) {
                    16
                } else {
                    8
                }
            } else {
                0
            };

            format!(
                "{}{}",
                if unsigned { "u" } else { "i" },
                format_bits.max(max_bits)
            )
        }
    }
}

pub fn render_struct<W: Write>(
    mut into: W,
    name: &str,
    fields: &[Field<NamedType>],
) -> Result<(), Error> {
    writeln!(into, "#[derive(Clone, PartialEq, Serialize, Deserialize)]")?;
    writeln!(into, "struct {} {{", name)?;
    for field in fields {
        writeln!(into, "    #[serde(rename = \"{}\")]", field.name)?;
        writeln!(
            into,
            "    {}: {},",
            rustify_field_name(&field.name),
            render(&field.data_type)
        )?;
    }
    writeln!(into, "}}")?;

    Ok(())
}

pub fn render_enum<W: Write>(
    mut into: W,
    name: &str,
    values: &[String],
    default: Option<&String>,
) -> Result<(), Error> {
    let mut used = HashSet::with_capacity(values.len());

    let mut unmap = Vec::new();

    writeln!(
        into,
        "#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]"
    )?;
    writeln!(into, "enum {} {{", name)?;
    for value in values {
        let rusty_name = rustify_enum_constant(value);

        writeln!(into, "    #[serde(rename = \"{}\")]", value)?;
        writeln!(into, "    {},", rusty_name)?;
        unmap.push((rusty_name.to_string(), value));
        if !used.insert(rusty_name) {
            bail!("generated duplicate rusty name for {:?}", values);
        }
    }
    writeln!(into, "}}")?;

    if let Some(default) = default {
        writeln!(into)?;
        writeln!(into, "impl Default for {} {{", name)?;
        writeln!(into, "    fn default() -> Self {{")?;
        writeln!(into, "        {}::{}", name, rustify_enum_constant(default))?;
        writeln!(into, "    }}")?;
        writeln!(into, "}}")?;
    }

    writeln!(into)?;
    writeln!(into, "impl ToString for {} {{", name)?;
    writeln!(into, "    fn to_string(&self) -> String {{")?;
    writeln!(into, "        match self {{")?;
    for (k, v) in unmap {
        writeln!(into, "            {}::{} => \"{}\",", name, k, v)?;
    }
    writeln!(into, "        }}.to_string()")?;
    writeln!(into, "    }}")?;
    writeln!(into, "}}")?;

    Ok(())
}

fn rustify_field_name(name: &str) -> String {
    use heck::SnakeCase;
    let mut name = name.to_snake_case();
    if ["type"].contains(&name.as_ref()) {
        name.push('_');
    }
    name
}

fn rustify_enum_constant(name: &str) -> String {
    let without_non_identifier: String =
        name.chars().filter(|c| c.is_ascii_alphanumeric()).collect();

    if without_non_identifier.is_empty() {
        return "Empty".to_string();
    }

    use heck::CamelCase;

    let potentially_valid = if without_non_identifier
        .chars()
        .next()
        .expect("just checked it wasn't empty..")
        .is_ascii_digit()
    {
        format!("N{}", without_non_identifier)
    } else {
        without_non_identifier
    };

    potentially_valid.to_camel_case()
}

#[cfg(test)]
mod tests {
    #[test]
    fn name_munging() {
        use super::rustify_enum_constant;
        assert_eq!("Foobar", rustify_enum_constant("foo.bar")); // not ideal
    }
}

pub fn render_endpoints<W: Write>(
    mut into: W,
    endpoints: &[Endpoint<NamedType>],
) -> Result<(), Error> {
    for endpoint in endpoints {
        render_endpoint(&mut into, endpoint)
            .with_context(|_| format_err!("rendering endpoint: {:?}", endpoint.path_url))?;
    }

    Ok(())
}

fn render_endpoint<W: Write>(mut into: W, endpoint: &Endpoint<NamedType>) -> Result<(), Error> {
    let mut endpoint_ops: Vec<(&HttpMethod, &Operation<NamedType>)> = endpoint.ops.iter().collect();
    endpoint_ops.sort_by_key(|(&method, _)| method);
    for (method, op) in endpoint_ops {
        render_op(&mut into, &endpoint.path_url, *method, op)?;
    }
    Ok(())
}

fn render_op<W: Write>(
    mut into: W,
    path_url: &str,
    method: HttpMethod,
    op: &Operation<NamedType>,
) -> Result<(), Error> {
    #[cfg(never)]
    // looking at this is kinda useless as it can list two types, with no association with where they go
    match op.produces.len() {
        0 => (),
        1 => match op.produces[0].as_ref() {
            "application/json" => (),
            "application/octet-stream" => (),
            "text/plain" => (),
            "application/vnd.docker.raw-stream" => (),
            "application/x-tar" => (),
            other => bail!("unimplemented production type: {:?}", other),
        },
        _ => bail!("wrong number of productions: {:?}", op.produces),
    }

    #[cfg(never)] // sigh, this either
    match op.consumes.len() {
        0 | 1 => (),
        _ => bail!("wrong number of consumptions: {:?}", op.consumes),
    }

    let mut resp_codes: Vec<(&u16, &Response<NamedType>)> = op.responses.iter().collect();
    resp_codes.sort_by_key(|(&code, _)| code);

    let codes_enum_name = format!("{}Codes", op.id);

    writeln!(into, "#[derive(Clone, PartialEq)]")?;
    writeln!(into, "enum {} {{", codes_enum_name)?;
    for (&resp_code, response) in &resp_codes {
        if !response.description.is_empty() {
            writeln!(into, "    /// {}", response.description)?;
        }
        if let Some(resp_type) = response.resp_type.as_ref() {
            writeln!(into, "    {}({}),", code_enum_for_http_code(resp_code), render(resp_type))?;
        } else {
            writeln!(into, "    {},", code_enum_for_http_code(resp_code))?;
        }
    }
    writeln!(into, "}}")?;
    writeln!(into)?;

    writeln!(into, "fn {}(", rustify_field_name(&op.id))?;
    writeln!(into, "    client: &Client,")?;

    let mut queries = Vec::new();
    let mut body = None;
    let mut headers = Vec::new();
    let mut paths = Vec::new();

    for param in &op.params {
        let param_name = rustify_field_name(&param.name);
        let type_text = render_ref(&param.param_type);
        if param.required.unwrap_or(false) || ParamLocation::Body == param.loc {
            writeln!(into, "    {}: {},", param_name, type_text)?;
        } else {
            writeln!(into, "    {}: Option<{}>,", param_name, type_text)?;
        }

        match param.loc {
            ParamLocation::Body => {
                ensure!(body.is_none(), "can't have multiple bodies");
                body = Some(param);
            }
            ParamLocation::Query => queries.push(param),
            ParamLocation::Header => headers.push(param),
            ParamLocation::Path => paths.push(param),
        }
    }

    queries.sort_by_key(|p| &p.name);
    headers.sort_by_key(|p| &p.name);

    writeln!(into, ") -> Result<{}, Error> {{", codes_enum_name)?;
    let mut url = if paths.is_empty() {
        format!("\"{}\"", path_url)
    } else {
        writeln!(into, "    let url = format!(\"{}\",", path_url)?;
        for path in paths {
            writeln!(into, "        {0}={0},", path.name)?;
        }
        writeln!(into, "    );",)?;
        writeln!(into)?;
        "&url".to_string()
    };

    if !queries.is_empty() {
        writeln!(into, "    let url = {{")?;
        writeln!(
            into,
            "        let mut params = Vec::with_capacity({});",
            queries.len()
        )?;
        for query in queries {
            let input = rustify_field_name(&query.name);
            let type_string = match query.param_type {
                // TODO: this is what DOCKER wants, surely other people want different things?
                NamedType::Array { .. } => format!("{}.to_vec().join(\",\")", input),
                _ => format!("{}.to_string()", input),
            };
            let optional = !query.required.unwrap_or(false);

            if optional {
                writeln!(into, "        if let Some({0}) = {0} {{", input)?;
                write!(into, "    ")?;
            }

            writeln!(
                into,
                "        params.push((\"{}\", {}));",
                input, type_string
            )?;

            if optional {
                writeln!(into, "        }}")?;
            }
        }
        writeln!(into, "        Url::parse_with_params({}, &params)?", url)?;
        writeln!(into, "    }};")?;
        writeln!(into)?;

        url = "url".to_string();
    }

    if !headers.is_empty() {
        writeln!(into, "    let mut headers = Headers::new();")?;
        for header in &headers {
            let input = rustify_field_name(&header.name);
            let optional = !header.required.unwrap_or(false);
            if optional {
                writeln!(into, "    if let Some({0}) = {0} {{", input)?;
                write!(into, "    ")?;
            }
            writeln!(
                into,
                "    headers.set_raw(\"{}\", {}.to_string());",
                header.name, input
            )?;
            if optional {
                writeln!(into, "    }}")?;
            }
        }

        writeln!(into)?;
    }

    writeln!(
        into,
        "    let mut resp = client.{}({})",
        method.reqwest_method_name(),
        url,
    )?;

    if !headers.is_empty() {
        writeln!(into, "        .headers(headers)")?;
    }

    if let Some(body) = body {
        match body.param_type {
            NamedType::Unknown | NamedType::Simple(DataType::Binary) => {
                writeln!(into, "        // TODO: unknown body type")?
            }
            _ => writeln!(into, "        .json({})", rustify_field_name(&body.name))?,
        }
    }
    writeln!(into, "        .send()?;")?;
    writeln!(into)?;

    writeln!(into, "    Ok(match resp.status().as_u16() {{")?;

    for (&resp_code, response) in resp_codes {
        if response.resp_type.is_some() {
            writeln!(
                into,
                "        {} => {}::{}(resp.json()?),",
                resp_code, codes_enum_name, code_enum_for_http_code(resp_code)
            )?;
        } else {
            writeln!(
                into,
                "        {} => {}::{},",
                resp_code, codes_enum_name, code_enum_for_http_code(resp_code)
            )?;
        }
    }

    writeln!(
        into,
        "        other => bail!(\"unexpected server response {{}}\", other),"
    )?;
    writeln!(into, "    }})")?;
    writeln!(into, "}}")?;
    writeln!(into)?;

    Ok(())
}

fn code_enum_for_http_code(code: u16) -> String {
    if let Some(nice) = super::swagger::name_http_code(code) {
        nice.to_string()
    } else {
        format!("Http{}", code)
    }
}
