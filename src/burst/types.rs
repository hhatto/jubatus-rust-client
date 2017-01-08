// This file is auto-generated from burst.idl(0.6.4-96-g66ed74d5) with jenerator version 1.0.0-26-g0d84e505/support-rust-client-for-jenerator
// *** DO NOT EDIT ***

use std::collections::HashMap;
use common::datum::Datum;
use rmpv::Value;

#[derive(Default, Debug, Clone)]
pub struct KeywordWithParams {
    pub keyword: String,
    pub scaling_param: f64,
    pub gamma: f64,
}

impl KeywordWithParams {
    pub fn to_msgpack_value(&self) -> Value {
        Value::Array(vec![Value::String(self.keyword.to_owned()), Value::F64(self.scaling_param), Value::F64(self.gamma)])
    }

    pub fn from_msgpack_value(data: Value) -> KeywordWithParams {
        let s = data.as_array().unwrap();
        KeywordWithParams {
            keyword: s[0].as_str().unwrap().to_string(),
            scaling_param: s[1].as_f64().unwrap(),
            gamma: s[2].as_f64().unwrap(),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Batch {
    pub all_data_count: i64,
    pub relevant_data_count: i64,
    pub burst_weight: f64,
}

impl Batch {
    pub fn to_msgpack_value(&self) -> Value {
        Value::Array(vec![Value::I64(self.all_data_count), Value::I64(self.relevant_data_count), Value::F64(self.burst_weight)])
    }

    pub fn from_msgpack_value(data: Value) -> Batch {
        let s = data.as_array().unwrap();
        Batch {
            all_data_count: s[0].as_i64().unwrap(),
            relevant_data_count: s[1].as_i64().unwrap(),
            burst_weight: s[2].as_f64().unwrap(),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Window {
    pub start_pos: f64,
    pub batches: Vec<Batch>,
}

impl Window {
    pub fn to_msgpack_value(&self) -> Value {
        Value::Array(vec![Value::F64(self.start_pos), Value::Array(self.batches.iter().map(|x| x.to_msgpack_value()).collect())])
    }

    pub fn from_msgpack_value(data: Value) -> Window {
        let s = data.as_array().unwrap();
        Window {
            start_pos: s[0].as_f64().unwrap(),
            batches: s[1].as_array()
                .unwrap()
                .iter()
                .map(|x| Batch::from_msgpack_value(x.clone().clone()))
                .collect(),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Document {
    pub pos: f64,
    pub text: String,
}

impl Document {
    pub fn to_msgpack_value(&self) -> Value {
        Value::Array(vec![Value::F64(self.pos), Value::String(self.text.to_owned())])
    }

    pub fn from_msgpack_value(data: Value) -> Document {
        let s = data.as_array().unwrap();
        Document {
            pos: s[0].as_f64().unwrap(),
            text: s[1].as_str().unwrap().to_string(),
        }
    }
}
