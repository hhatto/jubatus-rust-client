// This file is auto-generated from clustering.idl(0.9.4-18-g4935b2bd) with jenerator version 1.0.0-27-ge6a9293f/support-rust-client-for-jenerator
// *** DO NOT EDIT ***

use std::collections::HashMap;
use common::datum::Datum;
use rmpv::Value;

#[derive(Default, Debug, Clone)]
pub struct WeightedDatum {
    pub weight: f64,
    pub point: Datum,
}

impl WeightedDatum {
    pub fn to_msgpack_value(&self) -> Value {
        Value::Array(vec![Value::F64(self.weight), self.point.to_msgpack_value()])
    }

    pub fn from_msgpack_value(data: Value) -> WeightedDatum {
        let s = data.as_array().unwrap();
        WeightedDatum { weight: s[0].as_f64().unwrap(),
                        point: Datum::from_msgpack_value(s[1].clone()), }
    }
}

#[derive(Default, Debug, Clone)]
pub struct IndexedPoint {
    pub id: String,
    pub point: Datum,
}

impl IndexedPoint {
    pub fn to_msgpack_value(&self) -> Value {
        Value::Array(vec![Value::String(self.id.to_owned()),
                          self.point.to_msgpack_value()])
    }

    pub fn from_msgpack_value(data: Value) -> IndexedPoint {
        let s = data.as_array().unwrap();
        IndexedPoint { id: s[0].as_str().unwrap().to_string(),
                       point: Datum::from_msgpack_value(s[1].clone()), }
    }
}

#[derive(Default, Debug, Clone)]
pub struct WeightedIndex {
    pub weight: f64,
    pub id: String,
}

impl WeightedIndex {
    pub fn to_msgpack_value(&self) -> Value {
        Value::Array(vec![Value::F64(self.weight), Value::String(self.id.to_owned())])
    }

    pub fn from_msgpack_value(data: Value) -> WeightedIndex {
        let s = data.as_array().unwrap();
        WeightedIndex { weight: s[0].as_f64().unwrap(),
                        id: s[1].as_str().unwrap().to_string(), }
    }
}
