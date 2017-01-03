// This file is auto-generated from stat.idl(0.6.4-33-gcc8d7ca9) with jenerator version 1.0.0-25-g26c97cda/support-rust-client-for-jenerator
// *** DO NOT EDIT ***


use std::collections::HashMap;
use msgpack::Value;
use msgpack::value::{Integer, Float};
use common::datum::Datum;
use common::client::Client;
use stat::types::*;
use rmp_serialize::Decoder;

pub struct StatClient<'a> {
    client: Client<'a>,
}

impl<'a> StatClient<'a> {
    pub fn new(host: &str, name: &'a str) -> Self {
        let c = Client::new(host, name);
        StatClient { client: c }
    }

    pub fn push(&mut self, key: String, value: f64) -> bool {
        let args: Vec<Value> = vec![Value::String(key.to_owned()), Value::Float(Float::F64(value))];
        let result = self.client.call("push", args);
        result.as_bool().unwrap()
    }

    pub fn sum(&mut self, key: String) -> f64 {
        let args: Vec<Value> = vec![Value::String(key.to_owned())];
        let result = self.client.call("sum", args);
        result.as_f64().unwrap()
    }

    pub fn stddev(&mut self, key: String) -> f64 {
        let args: Vec<Value> = vec![Value::String(key.to_owned())];
        let result = self.client.call("stddev", args);
        result.as_f64().unwrap()
    }

    pub fn max(&mut self, key: String) -> f64 {
        let args: Vec<Value> = vec![Value::String(key.to_owned())];
        let result = self.client.call("max", args);
        result.as_f64().unwrap()
    }

    pub fn min(&mut self, key: String) -> f64 {
        let args: Vec<Value> = vec![Value::String(key.to_owned())];
        let result = self.client.call("min", args);
        result.as_f64().unwrap()
    }

    pub fn entropy(&mut self, key: String) -> f64 {
        let args: Vec<Value> = vec![Value::String(key.to_owned())];
        let result = self.client.call("entropy", args);
        result.as_f64().unwrap()
    }

    pub fn moment(&mut self, key: String, degree: i64, center: f64) -> f64 {
        let args: Vec<Value> = vec![Value::String(key.to_owned()), Value::Integer(Integer::I64(degree)), Value::Float(Float::F64(center))];
        let result = self.client.call("moment", args);
        result.as_f64().unwrap()
    }

    pub fn clear(&mut self) -> bool {
        let args: Vec<Value> = vec![];
        let result = self.client.call("clear", args);
        result.as_bool().unwrap()
    }

    pub fn save(&mut self, id: String) -> HashMap<String, String> {
        let args: Vec<Value> = vec![Value::String(id)];
        let result = self.client.call("save", args);
        let mut ret: HashMap<String, String> = HashMap::new();
        for r in result.as_map().unwrap().iter() {
            let (ref k, ref v): (Value, Value) = *r;
            ret.insert(k.as_str().unwrap().to_string(), v.as_str().unwrap().to_string());
        }
        ret
    }

    pub fn load(&mut self, id: String) -> bool {
        let args: Vec<Value> = vec![Value::String(id)];
        let result = self.client.call("load", args);
        result.as_bool().unwrap()
    }

    pub fn get_config(&mut self) -> String {
        let args: Vec<Value> = vec![];
        let result = self.client.call("get_config", args);
        result.as_str().unwrap().to_string()
    }

    pub fn get_status(&mut self) -> HashMap<String, HashMap<String, String>> {
        let args: Vec<Value> = vec![];
        let result = self.client.call("get_status", args);
        let mut ret: HashMap<String, HashMap<String, String>> = HashMap::new();
        for r in result.as_map().unwrap().iter() {
            let (ref kk, ref vv): (Value, Value) = *r;
            let mut hh: HashMap<String, String> = HashMap::new();
            for rr in vv.as_map().unwrap().iter() {
                let (ref kkk, ref vvv): (Value, Value) = *rr;
                hh.insert(kkk.as_str().unwrap().to_string(), vvv.as_str().unwrap().to_string());
            }
            ret.insert(kk.as_str().unwrap().to_string(), hh);
        }
        ret
    }

    pub fn do_mix(&mut self) -> bool {
        let args: Vec<Value> = vec![];
        let result = self.client.call("do_mix", args);
        result.as_bool().unwrap()
    }

    pub fn get_proxy_status(&mut self) -> HashMap<String, HashMap<String, String>> {
        let args: Vec<Value> = vec![];
        let result = self.client.call("get_proxy_status", args);
        let mut ret: HashMap<String, HashMap<String, String>> = HashMap::new();
        for r in result.as_map().unwrap().iter() {
            let (ref kk, ref vv): (Value, Value) = *r;
            let mut hh: HashMap<String, String> = HashMap::new();
            for rr in vv.as_map().unwrap().iter() {
                let (ref kkk, ref vvv): (Value, Value) = *rr;
                hh.insert(kkk.as_str().unwrap().to_string(), vvv.as_str().unwrap().to_string());
            }
            ret.insert(kk.as_str().unwrap().to_string(), hh);
        }
        ret
    }

    pub fn get_name(&self) -> &str {
        return self.client.name;
    }

    pub fn set_name(&mut self, new_name: &'a str) {
        self.client.name = new_name;
    }
}
