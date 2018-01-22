// This file is auto-generated from recommender.idl(1.0.0-112-g051a909c) with jenerator version 1.0.0-76-g95eed232/support-rust-client-for-jenerator
// *** DO NOT EDIT ***

use std::collections::HashMap;
use common::datum::Datum;
use rmpv::Value;

#[derive(Default, Debug, Clone)]
pub struct IdWithScore {
    pub id: String,
    pub score: f64,
}

impl IdWithScore {
    pub fn to_msgpack_value(&self) -> Value {
        Value::Array(vec![Value::from(self.id.to_owned()), Value::from(self.score)])
    }

    pub fn from_msgpack_value(data: Value) -> IdWithScore {
        let s = data.as_array().unwrap();
        IdWithScore { id: s[0].as_str().unwrap().to_string(),
                      score: s[1].as_f64().unwrap(), }
    }
}
