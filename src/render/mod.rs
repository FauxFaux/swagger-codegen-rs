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
}

pub fn render_top(
    p: &Field,
    rendered_as: &HashMap<usize, String>,
    structs: &Vec<Struct>,
    into: &mut Vec<Rendered>,
) -> Result<FlatField, Error> {
    Ok(match &p.data_type {
        FieldType::Inner(id) => {
            let name = p.name.to_string();
            // BORROW CHECKER
            let created = Rendered::Struct {
                name: name.to_string(),
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
        FieldType::Simple(simple) => FlatField::Data(simple.clone()),
        other => unimplemented!("top-level type: {:?}", other),
    })
}
