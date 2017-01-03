// This file is auto-generated from regression.idl(0.6.4-33-gcc8d7ca9) with jenerator version 1.0.0-6-gebf1c263/support-rust-client-for-jenerator
// *** DO NOT EDIT ***

use std::collections::HashMap;
use common::datum::Datum;
use msgpack::Value;
use msgpack::value::Float;
use msgpack::value::Integer;

#[derive(Default, Debug, Clone)]
pub struct ScoredDatum {
    pub score: f64,
    pub data: Datum,
}

impl ScoredDatum {
    pub fn to_msgpack_value(&self) -> Value {
        Value::Array(vec![Value::Float(Float::F64(self.score)), self.data.to_msgpack_value()])
    }

    pub fn from_msgpack_value(data: Value) -> ScoredDatum {
        let s = data.as_array().unwrap();
        ScoredDatum {
            score: s[0].as_f64().unwrap(),
            data: Datum::from_msgpack_value(s[1].clone()),
        }
    }
}
