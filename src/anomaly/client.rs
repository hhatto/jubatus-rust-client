// This file is auto-generated from anomaly.idl(0.9.0-26-g051b3019) with jenerator version 1.0.0-25-g26c97cda/support-rust-client-for-jenerator
// *** DO NOT EDIT ***


use std::collections::HashMap;
use msgpack::Value;
use msgpack::value::{Integer, Float};
use common::datum::Datum;
use common::client::Client;
use anomaly::types::*;
use rmp_serialize::Decoder;

pub struct AnomalyClient<'a> {
    client: Client<'a>,
}

impl<'a> AnomalyClient<'a> {
    pub fn new(host: &str, name: &'a str) -> Self {
        let c = Client::new(host, name);
        AnomalyClient { client: c }
    }

    pub fn clear_row(&mut self, id: String) -> bool {
        let args: Vec<Value> = vec![Value::String(id.to_owned())];
        let result = self.client.call("clear_row", args);
        result.as_bool().unwrap()
    }

    pub fn add(&mut self, row: Datum) -> IdWithScore {
        let args: Vec<Value> = vec![row.to_msgpack_value()];
        let result = self.client.call("add", args);
        IdWithScore::from_msgpack_value(result.clone())
    }

    pub fn update(&mut self, id: String, row: Datum) -> f64 {
        let args: Vec<Value> = vec![Value::String(id.to_owned()), row.to_msgpack_value()];
        let result = self.client.call("update", args);
        result.as_f64().unwrap()
    }

    pub fn overwrite(&mut self, id: String, row: Datum) -> f64 {
        let args: Vec<Value> = vec![Value::String(id.to_owned()), row.to_msgpack_value()];
        let result = self.client.call("overwrite", args);
        result.as_f64().unwrap()
    }

    pub fn clear(&mut self) -> bool {
        let args: Vec<Value> = vec![];
        let result = self.client.call("clear", args);
        result.as_bool().unwrap()
    }

    pub fn calc_score(&mut self, row: Datum) -> f64 {
        let args: Vec<Value> = vec![row.to_msgpack_value()];
        let result = self.client.call("calc_score", args);
        result.as_f64().unwrap()
    }

    pub fn get_all_rows(&mut self) -> Vec<String> {
        let args: Vec<Value> = vec![];
        let result = self.client.call("get_all_rows", args);
        result.as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect()
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
