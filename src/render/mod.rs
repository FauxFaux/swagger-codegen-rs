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

enum FlatField {
    Data(DataType),
}

pub fn render_top(
    p: &Field,
    rendered_as: &HashMap<usize, String>,
    structs: &Vec<Struct>,
) -> Result<Rendered, Error> {
    Ok(match &p.data_type {
        FieldType::Inner(id) => {
            render(&p.name, &structs[*id]).with_context(|_| format_err!("rendering struct"))?
        }
        other => unimplemented!("top-level type: {:?}", other),
    })
}

fn render(name: &str, s: &Struct) -> Result<Rendered, Error> {
    Ok(Rendered::Struct {
        name: name.to_string(),
        description: String::new(), // TODO,
        fields: s
            .fields
            .iter()
            .map(render_field)
            .collect::<Result<Vec<FlatField>, Error>>()?,
    })
}

fn render_field(f: &Field) -> Result<FlatField, Error> {
    Ok(match &f.data_type {
        FieldType::Simple(simple) => FlatField::Data(simple.clone()),
        other => bail!("{}: unimplemented field type: {:?}", f.name, other),
    })
}
