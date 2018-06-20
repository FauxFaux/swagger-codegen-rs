use std::collections::HashMap;

use failure::Error;
use failure::ResultExt;

use swagger::ArrayConstraints;
use swagger::DataType;
use swagger::Field;
use swagger::FieldType;
use swagger::Struct;

#[derive(Debug, Clone)]
pub enum Rendered {
    Struct {
        name: String,
        description: String,
        fields: Vec<FlatField>,
    },
}

#[derive(Debug, Clone)]
pub enum FlatField {
    Data(DataType),
    InternalType(String),
    Array {
        tee: Box<FlatField>,
        constraints: ArrayConstraints,
    },
    Tainted,
}

pub fn render_top(
    p: &Field,
    rendered_as: &HashMap<usize, String>,
    structs: &Vec<Struct>,
    into: &mut Vec<Rendered>,
) -> Result<FlatField, Error> {
    let name = p.name.to_string();
    Ok(render_type(&name, &p.data_type, into).with_context(|_| format_err!("named {}", name))?)
}

pub fn render_type(
    name_hint: &str,
    data_type: &FieldType,
    into: &mut Vec<Rendered>,
) -> Result<FlatField, Error> {
    Ok(match data_type {
        FieldType::Fields(fields) => {
            bail!("unimplemented! {:?}", fields);
        }
        FieldType::Array { tee, constraints } => FlatField::Array {
            tee: Box::new(
                render_type("unimplemented!", &item_type, into)
                    .with_context(|_| format_err!("unpacking array"))?,
            ),
            constraints,
        },
        FieldType::Simple(simple) => FlatField::Data(simple.clone()),
        FieldType::Unknown => FlatField::Tainted,
        other => bail!("type: {:?}", other),
    })
}
