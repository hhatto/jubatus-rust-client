// This file is auto-generated from recommender.idl(0.6.4-33-gcc8d7ca9) with jenerator version 0.9.4-42-g70f75391/master
// *** DO NOT EDIT ***

use std::collections::HashMap;
use rustc_serialize::{Encodable, Decodable};
use common::datum::Datum;
use msgpack::Value;
use msgpack::value::Float;
use msgpack::value::Integer;

#[derive(RustcEncodable, RustcDecodable, Default, Debug, Clone)]
pub struct IdWithScore {
    pub id: String,
    pub score: f64,
}

impl IdWithScore {
    pub fn to_msgpack_value(&self) -> Value {
        Value::Array(vec![Value::String(self.id.to_owned()), Value::Float(Float::F64(self.score))])
    }

    pub fn from_msgpack_value(data: Value) -> IdWithScore {
        let s = data.as_array().unwrap();
        IdWithScore {
            id: s[0].as_str().unwrap().to_string(),
            score: s[1].as_f64().unwrap(),
        }
    }
}
