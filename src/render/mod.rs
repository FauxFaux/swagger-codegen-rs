use failure::Error;
use failure::ResultExt;

use swagger::ArrayConstraints;
use swagger::DataType;
use swagger::Field;
use swagger::FullType;

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

pub fn render_top(p: &Field<FullType>, into: &mut Vec<Rendered>) -> Result<FlatField, Error> {
    let name = p.name.to_string();
    Ok(render_type(&name, &p.data_type, into).with_context(|_| format_err!("named {}", name))?)
}

pub fn render_type(
    name_hint: &str,
    data_type: &FullType,
    into: &mut Vec<Rendered>,
) -> Result<FlatField, Error> {
    Ok(match data_type {
        FullType::Fields(fields) => {
            bail!("unimplemented rendering fields {:?}", fields);
        }
        FullType::Array { tee, constraints } => FlatField::Array {
            tee: Box::new(
                render_type("unimplemented!", &tee, into)
                    .with_context(|_| format_err!("unpacking array"))?,
            ),
            constraints: constraints.clone(),
        },
        FullType::Simple(simple) => FlatField::Data(simple.clone()),
        FullType::Unknown => FlatField::Tainted,
    })
}
