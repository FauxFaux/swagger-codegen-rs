use cast::f64;
use failure::Error;
use failure::ResultExt;
use mime::Mime;
use result::ResultOptionExt;
use std::collections::HashMap;
use std::collections::HashSet;
use yaml_rust::yaml::Hash;
use yaml_rust::Yaml;

pub mod definitions;
pub mod name;
pub mod paths;

use float::TextualFloat;

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
    Fields(Vec<Field<NamedType>>),
    Array {
        tee: Box<NamedType>,
        constraints: ArrayConstraints,
    },
    Unknown,
}

#[derive(Debug, Clone)]
pub enum FullType {
    Simple(DataType),
    Fields(Vec<Field<FullType>>),
    Array {
        tee: Box<FullType>,
        constraints: ArrayConstraints,
    },
    Unknown,
}

#[derive(Debug, Clone)]
pub enum PartialType {
    Simple(DataType),
    Fields(Vec<Field<PartialType>>),
    Array {
        tee: Box<PartialType>,
        constraints: ArrayConstraints,
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
        min: Option<TextualFloat>,
        max: Option<TextualFloat>,
        default: Option<TextualFloat>,
        format: NumberFormat,
    },
    Bool {
        default: Option<bool>,
    },
    IpAddr,
    DateTime,
    Json,
    Binary,
    Enum {
        values: Vec<String>,
        default: Option<String>,
    },
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
    pub name: String,
    pub loc: ParamLocation,
    pub description: String,
    pub required: Option<bool>,
    pub param_type: T,
}

#[derive(Debug, Clone)]
pub struct Response<T> {
    description: String,
    headers: HashMap<String, Header>,
    resp_type: Option<T>,
}

#[derive(Debug, Clone)]
pub struct Header {
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
pub enum ParamLocation {
    Query,
    Body,
    Path,
    Header,
}

impl<T> Endpoint<T> {
    fn map_type<F, R>(self, func: F) -> Result<Endpoint<R>, Error>
    where
        F: Fn(T) -> Result<R, Error>,
    {
        Ok(Endpoint::<R> {
            path_url: self.path_url,
            ops: self
                .ops
                .into_iter()
                .map(|(code, op)| op.map_type(&func).map(|op| (code, op)))
                .collect::<Result<HashMap<HttpMethod, Operation<R>>, Error>>()?,
        })
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
    fn map_type<F, R>(self, func: F) -> Result<Operation<R>, Error>
    where
        F: Fn(T) -> Result<R, Error>,
    {
        Ok(Operation::<R> {
            id: self.id,
            consumes: self.consumes,
            produces: self.produces,
            params: self
                .params
                .into_iter()
                .map(|p| p.map_type(&func))
                .collect::<Result<Vec<Param<R>>, Error>>()?,
            responses: self
                .responses
                .into_iter()
                .map(|(code, resp)| resp.map_type(&func).map(|resp| (code, resp)))
                .collect::<Result<HashMap<u16, Response<R>>, Error>>()?,
        })
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

fn optional_number(hash: &Hash, key: &str) -> Result<Option<TextualFloat>, Error> {
    Ok(get(hash, key)
        .ok()
        .map(as_number)
        .invert()
        .with_context(|_| format_err!("key: {}", key))?)
}

fn as_number(val: &Yaml) -> Result<TextualFloat, Error> {
    Ok(TextualFloat {
        val: val
            .as_f64()
            .or_else(|| val.as_i64().map(f64))
            .ok_or_else(|| format_err!("not number: {:?}", val))?,
    })
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
