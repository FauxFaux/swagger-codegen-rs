use std::collections::HashSet;
use std::io::Write;

use cast::i64;
use failure::Error;
use failure::ResultExt;

use swagger::DataType;
use swagger::Endpoint;
use swagger::Field;
use swagger::IntegerFormat;
use swagger::NamedType;
use swagger::NamingType;

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
        NamedType::Unknown => "()".to_string(),
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
        Json => "::serde_json::Json".to_string(),
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
    writeln!(
        into,
        "#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]"
    )?;
    writeln!(into, "struct {} {{", name)?;
    for field in fields {
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

    writeln!(
        into,
        "#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]"
    )?;
    writeln!(into, "enum {} {{", name)?;
    for value in values {
        let rusty_name = rustify_enum_constant(value);

        writeln!(into, "    #[serde(rename = \"{}\")]", value)?;
        writeln!(into, "    {},", rusty_name)?;
        if !used.insert(rusty_name) {
            bail!("generated duplicate rusty name for {:?}", values);
        }
    }
    writeln!(into, "}}")?;

    if let Some(default) = default {
        writeln!(into)?;
        writeln!(into, "impl Default for {} {{", name)?;
        writeln!(into, "    fn default() -> Self {{")?;
        writeln!(into, "        Self::{}", rustify_enum_constant(default))?;
        writeln!(into, "    }}")?;
        writeln!(into, "}}")?;
    }
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
    for (method, op) in &endpoint.ops {
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

        writeln!(into, "// {:?} {}", method, endpoint.path_url)?;
        for param in &op.params {
            writeln!(into, "//   {:?} {}", method, endpoint.path_url)?;
        }
    }
    Ok(())
}
