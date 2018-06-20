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

#[derive(Debug, Clone)]
pub struct Struct<T> {
    pub fields: Vec<Field<T>>,
}

#[derive(Debug, Clone)]
pub struct Field<T> {
    pub name: String,
    pub data_type: T,
    pub description: String,
    pub nullable: Option<bool>,
    pub required: bool,
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

#[derive(Debug, Copy, Clone)]
pub struct ArrayConstraints {
    min_items: Option<usize>,
    max_items: Option<usize>,
    null_default: bool,
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub enum NumberFormat {
    Unspecified,
}

#[derive(Debug, Clone)]
pub enum DataType {
    Integer {
        min: Option<i64>,
        max: Option<i64>,
        default: Option<i64>,
        format: IntegerFormat,
    },
    Number {
        min: Option<f64>,
        max: Option<f64>,
        default: Option<f64>,
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
pub struct Endpoint {
    pub path_url: String,
    pub ops: HashMap<HttpMethod, Operation>,
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub id: String,
    pub params: Vec<Param>,
    pub consumes: Vec<Mime>,
    pub produces: Vec<Mime>,
    pub responses: HashMap<u16, Response>,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub loc: ParamLocation,
    pub description: String,
    pub required: Option<bool>,
    pub param_type: PartialType,
}

#[derive(Debug, Clone)]
pub struct Response {
    description: String,
    headers: HashMap<String, Header>,
    resp_type: Option<PartialType>,
}

#[derive(Debug, Clone)]
pub struct Header {
    header_type: PartialType,
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

fn optional_number(hash: &Hash, key: &str) -> Result<Option<f64>, Error> {
    Ok(get(hash, key)
        .ok()
        .map(as_number)
        .invert()
        .with_context(|_| format_err!("key: {}", key))?)
}

fn as_number(val: &Yaml) -> Result<f64, Error> {
    val.as_f64()
        .or_else(|| val.as_i64().map(f64))
        .ok_or_else(|| format_err!("not number: {:?}", val))
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
