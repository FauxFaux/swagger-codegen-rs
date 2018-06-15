use std::collections::HashMap;

use failure::Error;
use failure::ResultExt;

use swagger::DataType;
use swagger::Field;
use swagger::FieldType;
use swagger::Struct;

pub enum Rendered {
    Struct {
        name: String,
        description: String,
        fields: Vec<FlatField>,
    },
}

pub enum FlatField {
    Data(DataType),
    InternalType(String),
    Tainted,
}

pub fn render_top(
    p: &Field,
    rendered_as: &HashMap<usize, String>,
    structs: &Vec<Struct>,
    into: &mut Vec<Rendered>,
) -> Result<FlatField, Error> {
    let name = p.name.to_string();
    let typed = render_type(&name, &p.data_type, rendered_as, structs, into)
        .with_context(|_| format_err!("named {}", name))?;
    Ok(typed)
}

pub fn render_type(
    name_hint: &str,
    data_type: &FieldType,
    rendered_as: &HashMap<usize, String>,
    structs: &Vec<Struct>,
    into: &mut Vec<Rendered>,
) -> Result<FlatField, Error> {
    Ok(match data_type {
        FieldType::Inner(id) => {
            // BORROW CHECKER
            let name = name_hint.to_string();
            let created = Rendered::Struct {
                name: name_hint.to_string(),
                description: String::new(), // TODO,
                fields: structs[*id]
                    .fields
                    .iter()
                    .map(|f| render_top(f, rendered_as, structs, into))
                    .collect::<Result<Vec<FlatField>, Error>>()?,
            };
            into.push(created);
            FlatField::InternalType(name)
        }
        FieldType::Array { item_type, .. } => {
            // TODO: nullable / item limits / fixed size array?
            render_type("", &item_type, rendered_as, structs, into)
                .with_context(|_| format_err!("unpacking array"))?
        }
        #[cfg(nah)]
        FieldType::Ref(id) if id.starts_with("#/definitions/") =>
        {
            FlatField::InternalType(id["#/definitions/".len()..].to_string())
        }
        FieldType::Simple(simple) => FlatField::Data(simple.clone()),
        FieldType::Unknown => FlatField::Tainted,
        other => bail!("type: {:?}", other),
    })
}
