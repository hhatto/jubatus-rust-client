// This file is auto-generated from bandit.idl(0.7.2-79-g2db27d79) with jenerator version 1.0.0-76-g95eed232/support-rust-client-for-jenerator
// *** DO NOT EDIT ***

use std::collections::HashMap;
use common::datum::Datum;
use rmpv::Value;

#[derive(Default, Debug, Clone)]
pub struct ArmInfo {
    pub trial_count: i64,
    pub weight: f64,
}

impl ArmInfo {
    pub fn to_msgpack_value(&self) -> Value {
        Value::Array(vec![Value::from(self.trial_count), Value::from(self.weight)])
    }

    pub fn from_msgpack_value(data: Value) -> ArmInfo {
        let s = data.as_array().unwrap();
        ArmInfo { trial_count: s[0].as_i64().unwrap(),
                  weight: s[1].as_f64().unwrap(), }
    }
}
