// This file is auto-generated from classifier.idl(0.8.9-17-gd4c007f7) with jenerator version 1.0.0-6-gebf1c263/support-rust-client-for-jenerator
// *** DO NOT EDIT ***

use std::collections::HashMap;
use rustc_serialize::{Encodable, Decodable};
use common::datum::Datum;
use msgpack::Value;
use msgpack::value::Float;
use msgpack::value::Integer;

#[derive(RustcEncodable, RustcDecodable, Default, Debug, Clone)]
pub struct EstimateResult {
    pub label: String,
    pub score: f64,
}

impl EstimateResult {
    pub fn to_msgpack_value(&self) -> Value {
        Value::Array(vec![Value::String(self.label.to_owned()), Value::Float(Float::F64(self.score))])
    }

    pub fn from_msgpack_value(data: Value) -> EstimateResult {
        let s = data.as_array().unwrap();
        EstimateResult {
            label: s[0].as_str().unwrap().to_string(),
            score: s[1].as_f64().unwrap(),
        }
    }
}

#[derive(RustcEncodable, RustcDecodable, Default, Debug, Clone)]
pub struct LabeledDatum {
    pub label: String,
    pub data: Datum,
}

impl LabeledDatum {
    pub fn to_msgpack_value(&self) -> Value {
        Value::Array(vec![Value::String(self.label.to_owned()), self.data.to_msgpack_value()])
    }

    pub fn from_msgpack_value(data: Value) -> LabeledDatum {
        let s = data.as_array().unwrap();
        LabeledDatum {
            label: s[0].as_str().unwrap().to_string(),
            data: Datum::from_msgpack_value(s[1].clone()),
        }
    }
}
