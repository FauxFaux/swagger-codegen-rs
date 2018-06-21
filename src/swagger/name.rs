use std::collections::HashMap;

use failure::Error;
use failure::ResultExt;
use result::ResultOptionExt;
use yaml_rust::yaml::Hash;

use swagger::definitions::properties_to_fields;
use swagger::Endpoint;
use swagger::Field;
use swagger::FullType;
use swagger::HttpMethod;
use swagger::Operation;
use swagger::Param;
use swagger::PartialType;
use swagger::Response;

type Defs = HashMap<String, Field<PartialType>>;

pub fn definitions(definitions: &Hash, paths: &Hash) -> Result<Vec<Endpoint<FullType>>, Error> {
    let definitions: Defs = properties_to_fields(&[], definitions)
        .with_context(|_| format_err!("processing definitions"))?
        .into_iter()
        .map(|field| (field.name.to_string(), field))
        .collect();

    let mut endpoints = Vec::new();

    for e in super::paths::paths(paths)? {
        endpoints.push(Endpoint {
            path_url: e.path_url,
            ops: e
                .ops
                .into_iter()
                .map(|(code, op)| translate_op(op, &definitions).map(|op| (code, op)))
                .collect::<Result<HashMap<HttpMethod, Operation<FullType>>, Error>>()?,
        });
    }

    Ok(endpoints)
}

fn translate_op(
    op: Operation<PartialType>,
    definitions: &Defs,
) -> Result<Operation<FullType>, Error> {
    Ok(Operation::<FullType> {
        id: op.id,
        consumes: op.consumes,
        produces: op.produces,

        params: op
            .params
            .into_iter()
            .map(|p| {
                deref(definitions, p.param_type.clone()).map(|new| Param::<FullType> {
                    name: p.name,
                    loc: p.loc,
                    description: p.description,
                    required: p.required,
                    param_type: new,
                })
            })
            .collect::<Result<Vec<Param<FullType>>, Error>>()?,

        responses: op
            .responses
            .into_iter()
            .map(|(code, resp)| deref_response(definitions, (code, resp)))
            .collect::<Result<HashMap<u16, Response<FullType>>, Error>>()?,
    })
}

fn deref_response(
    definitions: &Defs,
    (code, response): (u16, Response<PartialType>),
) -> Result<(u16, Response<FullType>), Error> {
    Ok((
        code,
        Response::<FullType> {
            description: response.description,
            headers: response.headers,
            resp_type: response.resp_type.map(|r| deref(definitions, r)).invert()?,
        },
    ))
}

fn deref(definitions: &Defs, data_type: PartialType) -> Result<FullType, Error> {
    Ok(match data_type {
        PartialType::Ref(ref id) => {
            ensure!(
                id.starts_with("#/definitions/"),
                "non-definitions ref: {}",
                id
            );
            let id = id["#/definitions/".len()..].to_string();
            let new_block: PartialType = definitions
                .get(&id)
                .ok_or_else(|| format_err!("definition not found: {}", id))?
                .data_type
                .clone();

            deref(definitions, new_block)?
        }
        PartialType::Array { tee, constraints } => FullType::Array {
            tee: Box::new(deref(definitions, *tee)?),
            constraints,
        },
        PartialType::AllOf(inner) => {
            let mut new = Vec::new();
            for child in inner {
                match deref(definitions, child)? {
                    FullType::Fields(fields) => new.extend(fields),
                    FullType::Unknown => (),
                    other => bail!("can't all-of {:?}", other),
                }
            }
            FullType::Fields(new)
        }
        PartialType::Fields(fields) => FullType::Fields(
            fields
                .into_iter()
                .map(|f| f.map_type(|t| deref(definitions, t)))
                .collect::<Result<Vec<Field<FullType>>, Error>>()?,
        ),
        PartialType::Simple(data_type) => FullType::Simple(data_type.clone()),
        PartialType::Unknown => FullType::Unknown,
    })
}
