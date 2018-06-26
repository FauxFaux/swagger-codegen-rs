use std::collections::HashMap;
use std::collections::HashSet;

use cast::f64;
use failure::Error;
use failure::ResultExt;
use mime::Mime;
use ordered_float::OrderedFloat;
use result::ResultOptionExt;
use yaml_rust::yaml::Hash;
use yaml_rust::Yaml;

mod full;
mod name;
mod partial_definitions;
mod partial_paths;

pub fn load(
    definitions: &Hash,
    paths: &Hash,
) -> Result<
    (
        Vec<Endpoint<NamedType>>,
        Vec<(String, NamingType<NamedType>)>,
    ),
    Error,
> {
    let (endpoints, def_names) = full::load_endpoints_and_names(definitions, paths)?;
    name::to_named_types(endpoints, def_names)
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum NamingType<T> {
    Field(Vec<Field<T>>),
    Enum(Vec<String>, Option<String>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Field<T> {
    pub name: String,
    pub data_type: T,
    pub description: String,
    pub nullable: Option<bool>,
    pub required: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum NamedType {
    Simple(DataType),
    Name(String),
    Array {
        tee: Box<NamedType>,
        constraints: ArrayConstraints,
    },
    Unknown,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(self) enum FullType {
    Simple(DataType),
    Fields(Vec<Field<FullType>>),
    Array {
        tee: Box<FullType>,
        constraints: ArrayConstraints,
    },
    Enum {
        values: Vec<String>,
        default: Option<String>,
    },
    Unknown,
}

#[derive(Debug, Clone)]
pub(self) enum PartialType {
    Simple(DataType),
    Fields(Vec<Field<PartialType>>),
    Array {
        tee: Box<PartialType>,
        constraints: ArrayConstraints,
    },
    Enum {
        values: Vec<String>,
        default: Option<String>,
    },
    AllOf(Vec<PartialType>),
    Ref(String),
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ArrayConstraints {
    min_items: Option<usize>,
    max_items: Option<usize>,
    null_default: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum IntegerFormat {
    Unspecified,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum NumberFormat {
    Unspecified,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DataType {
    Integer {
        min: Option<i64>,
        max: Option<i64>,
        default: Option<i64>,
        format: IntegerFormat,
    },
    Number {
        min: Option<OrderedFloat<f64>>,
        max: Option<OrderedFloat<f64>>,
        default: Option<OrderedFloat<f64>>,
        format: NumberFormat,
    },
    Bool {
        default: Option<bool>,
    },
    IpAddr,
    DateTime,
    Json,
    Binary,
    MatchString {
        pattern: String,
    },
    String {
        default: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub struct Endpoint<T> {
    pub path_url: String,
    pub ops: HashMap<HttpMethod, Operation<T>>,
}

#[derive(Debug, Clone)]
pub struct Operation<T> {
    pub id: String,
    pub params: Vec<Param<T>>,
    pub consumes: Vec<Mime>,
    pub produces: Vec<Mime>,
    pub responses: HashMap<u16, Response<T>>,
}

#[derive(Debug, Clone)]
pub struct Param<T> {
    name: String,
    loc: ParamLocation,
    description: String,
    required: Option<bool>,
    param_type: T,
}

#[derive(Debug, Clone)]
pub struct Response<T> {
    description: String,
    headers: HashMap<String, Header>,
    resp_type: Option<T>,
}

#[derive(Debug, Clone)]
struct Header {
    header_type: DataType,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum HttpMethod {
    GET,
    POST,
    HEAD,
    PUT,
    DELETE,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ParamLocation {
    Query,
    Body,
    Path,
    Header,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct StructContext {
    method: HttpMethod,
    id: Option<String>,
    place: StructContextPlace,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum StructContextPlace {
    Response(u16),
    Parameter(String),
    Unknown,
}

impl<T> Endpoint<T> {
    fn map_type<F, R>(self, mut func: F) -> Result<Endpoint<R>, Error>
    where
        F: FnMut(T) -> Result<R, Error>,
    {
        Ok(Endpoint::<R> {
            path_url: self.path_url,
            ops: self
                .ops
                .into_iter()
                .map(|(method, op)| op.map_type(&mut func).map(|op| (method, op)))
                .collect::<Result<HashMap<HttpMethod, Operation<R>>, Error>>()?,
        })
    }

    fn visit_type<F>(&self, mut func: F)
    where
        F: FnMut(&T, StructContext),
    {
        for (method, op) in &self.ops {
            op.visit_type(
                &mut func,
                StructContext {
                    method: *method,
                    id: None,
                    place: StructContextPlace::Unknown,
                },
            );
        }
    }
}

impl<T> Field<T> {
    fn map_type<F, R>(self, func: F) -> Result<Field<R>, Error>
    where
        F: FnOnce(T) -> Result<R, Error>,
    {
        Ok(Field::<R> {
            name: self.name,
            data_type: func(self.data_type)?,
            description: self.description,
            nullable: self.nullable,
            required: self.required,
        })
    }
}

impl<T> Operation<T> {
    fn map_type<F, R>(self, mut func: F) -> Result<Operation<R>, Error>
    where
        F: FnMut(T) -> Result<R, Error>,
    {
        Ok(Operation::<R> {
            id: self.id,
            consumes: self.consumes,
            produces: self.produces,
            params: self
                .params
                .into_iter()
                .map(|p| p.map_type(&mut func))
                .collect::<Result<Vec<Param<R>>, Error>>()?,
            responses: self
                .responses
                .into_iter()
                .map(|(code, resp)| resp.map_type(&mut func).map(|resp| (code, resp)))
                .collect::<Result<HashMap<u16, Response<R>>, Error>>()?,
        })
    }

    fn visit_type<F>(&self, mut func: F, mut name_hint: StructContext)
    where
        F: FnMut(&T, StructContext),
    {
        name_hint.id = Some(self.id.to_string());

        for param in &self.params {
            param.visit_type(&mut func, name_hint.clone());
        }

        for (code, response) in &self.responses {
            name_hint.place = StructContextPlace::Response(*code);
            response.visit_type(&mut func, name_hint.clone());
        }
    }
}

impl<T> Param<T> {
    fn map_type<F, R>(self, func: F) -> Result<Param<R>, Error>
    where
        F: FnOnce(T) -> Result<R, Error>,
    {
        Ok(Param::<R> {
            name: self.name,
            loc: self.loc,
            description: self.description,
            required: self.required,
            param_type: func(self.param_type)?,
        })
    }

    fn visit_type<F>(&self, mut func: F, mut name_hint: StructContext)
    where
        F: FnMut(&T, StructContext),
    {
        name_hint.place = StructContextPlace::Parameter(self.name.to_string());

        func(&self.param_type, name_hint)
    }
}

impl<T> Response<T> {
    fn map_type<F, R>(self, func: F) -> Result<Response<R>, Error>
    where
        F: FnOnce(T) -> Result<R, Error>,
    {
        Ok(Response::<R> {
            description: self.description,
            headers: self.headers,
            resp_type: self.resp_type.map(func).invert()?,
        })
    }

    fn visit_type<F>(&self, mut func: F, name_hint: StructContext)
    where
        F: FnMut(&T, StructContext),
    {
        if let Some(resp_type) = self.resp_type.as_ref() {
            func(resp_type, name_hint);
        }
    }
}

impl StructContext {
    fn recommended_names(&self) -> impl Iterator<Item = String> {
        let mut ret = Vec::new();
        use heck::CamelCase;

        let id = self
            .id
            .as_ref()
            .expect("should only be uninitialised during construction?");

        ret.push(id.to_string());
        match &self.place {
            StructContextPlace::Response(code) => {
                if let Some(code) = name_http_code(*code) {
                    ret.push(format!("{}{}", id, code));
                    ret.push(format!("{}Response{}", id, code));
                }

                ret.push(format!("{}{}", id, code));
                ret.push(format!("{}Response{}", id, code));
            }
            StructContextPlace::Parameter(name) => {
                ret.push(format!("{}{}", id, name.to_camel_case()));
                ret.push(format!("{}Param{}", id, name.to_camel_case()));
            }
            StructContextPlace::Unknown => (),
        }

        ret.into_iter()
    }
}

// intentionally not an exhaustive list
fn name_http_code(code: u16) -> Option<&'static str> {
    match code {
        200 => Some("Ok"),
        201 => Some("Created"),
        202 => Some("Accepted"),
        204 => Some("NoContent"),
        206 => Some("PartialContent"),
        300 => Some("MultipleChoices"),
        301 => Some("MovedPermanently"),
        302 => Some("Found"),
        303 => Some("SeeOther"),
        307 => Some("TemporaryRedirect"),
        400 => Some("BadRequest"),
        401 => Some("Unauthorised"),
        403 => Some("Forbidden"),
        404 => Some("NotFound"),
        409 => Some("Conflict"),
        410 => Some("Gone"),
        412 => Some("PreconditionFailed"),
        500 => Some("ServerError"),
        501 => Some("NotImplemented"),
        503 => Some("ServiceUnavailable"),
        _ => None,
    }
}

fn keys(hash: &Hash) -> Result<HashSet<&str>, Error> {
    hash.keys()
        .map(|k| {
            k.as_str()
                .ok_or_else(|| format_err!("invalid non-string key in hash: {:?}", k))
        })
        .collect()
}

fn get<'h>(hash: &'h Hash, key: &str) -> Result<&'h Yaml, Error> {
    hash.get(&Yaml::String(key.to_string()))
        .ok_or_else(|| format_err!("key '{}' missing", key))
}

fn get_string<'h>(hash: &'h Hash, key: &str) -> Result<&'h str, Error> {
    get(hash, key).and_then(as_str)
}

fn as_str(yaml: &Yaml) -> Result<&str, Error> {
    yaml.as_str()
        .ok_or_else(|| format_err!("not string: {:?}", yaml))
}

fn get_bool(hash: &Hash, key: &str) -> Result<bool, Error> {
    Ok(get(hash, key)
        .and_then(as_bool)
        .with_context(|_| format_err!("key: {}", key))?)
}

fn optional_bool(hash: &Hash, key: &str) -> Result<Option<bool>, Error> {
    Ok(get(hash, key)
        .ok()
        .map(as_bool)
        .invert()
        .with_context(|_| format_err!("key: {}", key))?)
}

fn as_bool(val: &Yaml) -> Result<bool, Error> {
    val.as_bool()
        .ok_or_else(|| format_err!("not bool: {:?}", val))
}

fn optional_number(hash: &Hash, key: &str) -> Result<Option<OrderedFloat<f64>>, Error> {
    Ok(get(hash, key)
        .ok()
        .map(as_number)
        .invert()
        .with_context(|_| format_err!("key: {}", key))?)
}

fn as_number(val: &Yaml) -> Result<OrderedFloat<f64>, Error> {
    Ok(OrderedFloat(
        val.as_f64()
            .or_else(|| val.as_i64().map(f64))
            .ok_or_else(|| format_err!("not number: {:?}", val))?,
    ))
}

fn optional_integer(hash: &Hash, key: &str) -> Result<Option<i64>, Error> {
    Ok(get(hash, key)
        .ok()
        .map(as_integer)
        .invert()
        .with_context(|_| format_err!("key: {}", key))?)
}

fn as_integer(val: &Yaml) -> Result<i64, Error> {
    val.as_i64()
        .ok_or_else(|| format_err!("not integer: {:?}", val))
}

fn get_vec<'h>(hash: &'h Hash, key: &str) -> Result<&'h Vec<Yaml>, Error> {
    get(hash, key).and_then(|y| {
        y.as_vec()
            .ok_or_else(|| format_err!("key '{}' not vec: {:?}", key, y))
    })
}

fn get_hash<'h>(hash: &'h Hash, key: &str) -> Result<&'h Hash, Error> {
    get(hash, key).and_then(|y| {
        y.as_hash()
            .ok_or_else(|| format_err!("key '{}' not hash: {:?}", key, y))
    })
}
