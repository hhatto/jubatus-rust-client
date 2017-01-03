// This file is auto-generated from nearest_neighbor.idl(0.8.2-20-g8e4dc3b5) with jenerator version 1.0.0-6-gebf1c263/support-rust-client-for-jenerator
// *** DO NOT EDIT ***

use std::collections::HashMap;
use common::datum::Datum;
use msgpack::Value;
use msgpack::value::Float;
use msgpack::value::Integer;

#[derive(Default, Debug, Clone)]
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
