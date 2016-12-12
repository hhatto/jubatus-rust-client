// This file is auto-generated from bandit.idl(0.7.2-79-g2db27d79) with jenerator version 0.9.4-42-g70f75391/master
// *** DO NOT EDIT ***

use std::collections::HashMap;
use rustc_serialize::{Encodable, Decodable};
use common::datum::Datum;
use msgpack::Value;
use msgpack::value::Float;
use msgpack::value::Integer;

#[derive(RustcEncodable, RustcDecodable, Default, Debug, Clone)]
pub struct ArmInfo {
    pub trial_count: i64,
    pub weight: f64,
}

impl ArmInfo {
    pub fn to_msgpack_value(&self) -> Value {
        Value::Array(vec![Value::Integer(Integer::I64(self.trial_count)), Value::Float(Float::F64(self.weight))])
    }

    pub fn from_msgpack_value(data: Value) -> ArmInfo {
        let s = data.as_array().unwrap();
        ArmInfo {
            trial_count: s[0].as_i64().unwrap(),
            weight: s[1].as_f64().unwrap(),
        }
    }
}
