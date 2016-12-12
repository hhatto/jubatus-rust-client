// This file is auto-generated from weight.idl(0.9.0-24-gda613834) with jenerator version 0.9.4-42-g70f75391/master
// *** DO NOT EDIT ***

use std::collections::HashMap;
use rustc_serialize::{Encodable, Decodable};
use common::datum::Datum;
use msgpack::Value;
use msgpack::value::Float;
use msgpack::value::Integer;

#[derive(RustcEncodable, RustcDecodable, Default, Debug, Clone)]
pub struct Feature {
    pub key: String,
    pub value: f64,
}

impl Feature {
    pub fn to_msgpack_value(&self) -> Value {
        Value::Array(vec![Value::String(self.key.to_owned()), Value::Float(Float::F64(self.value))])
    }

    pub fn from_msgpack_value(data: Value) -> Feature {
        let s = data.as_array().unwrap();
        Feature {
            key: s[0].as_str().unwrap().to_string(),
            value: s[1].as_f64().unwrap(),
        }
    }
}
