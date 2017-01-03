// This file is auto-generated from classifier.idl(0.8.9-17-gd4c007f7) with jenerator version 1.0.0-25-g26c97cda/support-rust-client-for-jenerator
// *** DO NOT EDIT ***


use std::collections::HashMap;
use msgpack::Value;
use msgpack::value::{Integer, Float};
use common::datum::Datum;
use common::client::Client;
use classifier::types::*;
use rmp_serialize::Decoder;

pub struct ClassifierClient<'a> {
    client: Client<'a>,
}

impl<'a> ClassifierClient<'a> {
    pub fn new(host: &str, name: &'a str) -> Self {
        let c = Client::new(host, name);
        ClassifierClient { client: c }
    }

    pub fn train(&mut self, data: Vec<LabeledDatum>) -> i64 {
        let args: Vec<Value> = vec![Value::Array(data.iter()
                                        .map(|x| x.to_msgpack_value())
                                        .collect())];
        let result = self.client.call("train", args);
        result.as_i64().unwrap()
    }

    pub fn classify(&mut self, data: Vec<Datum>) -> Vec<Vec<EstimateResult>> {
        let args: Vec<Value> = vec![Value::Array(data.iter()
                                        .map(|x| x.to_msgpack_value())
                                        .collect())];
        let result = self.client.call("classify", args);
        result.as_array()
            .unwrap()
            .iter()
            .map(|x| {
                x.as_array()
                    .unwrap()
                    .iter()
                    .map(|x| EstimateResult::from_msgpack_value(x.clone()))
                    .collect()
            })
            .collect()
    }

    pub fn get_labels(&mut self) -> HashMap<String, u64> {
        let args: Vec<Value> = vec![];
        let result = self.client.call("get_labels", args);
        result.as_map()
            .unwrap()
            .iter()
            .map(|m| {
                let (ref k, ref v): (Value, Value) = *m;
                (k.as_str().unwrap().to_string(), v.as_u64().unwrap())
            })
            .collect::<HashMap<String, u64>>()
    }

    pub fn set_label(&mut self, new_label: String) -> bool {
        let args: Vec<Value> = vec![Value::String(new_label.to_owned())];
        let result = self.client.call("set_label", args);
        result.as_bool().unwrap()
    }

    pub fn clear(&mut self) -> bool {
        let args: Vec<Value> = vec![];
        let result = self.client.call("clear", args);
        result.as_bool().unwrap()
    }

    pub fn delete_label(&mut self, target_label: String) -> bool {
        let args: Vec<Value> = vec![Value::String(target_label.to_owned())];
        let result = self.client.call("delete_label", args);
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
