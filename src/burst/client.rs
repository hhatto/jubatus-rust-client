// This file is auto-generated from burst.idl(0.6.4-96-g66ed74d5) with jenerator version 1.0.0-76-g95eed232/support-rust-client-for-jenerator
// *** DO NOT EDIT ***

use std::collections::HashMap;
use rmpv::Value;
use common::datum::Datum;
use common::client::Client;
use burst::types::*;

pub struct BurstClient<'a> {
    client: Client<'a>,
}

impl<'a> BurstClient<'a> {
    pub fn new(host: &str, name: &'a str) -> Self {
        let c = Client::new(host, name);
        BurstClient { client: c }
    }

    pub fn add_documents(&mut self, data: Vec<Document>) -> i64 {
        let args: Vec<Value> = vec![Value::Array(data.iter().map(|x| x.to_msgpack_value()).collect())];
        let result = self.client.call("add_documents", args);
        result.as_i64().unwrap()
    }

    pub fn get_result(&mut self, keyword: String) -> Window {
        let args: Vec<Value> = vec![Value::from(keyword.to_owned())];
        let result = self.client.call("get_result", args);
        Window::from_msgpack_value(result.clone())
    }

    pub fn get_result_at(&mut self, keyword: String, pos: f64) -> Window {
        let args: Vec<Value> = vec![Value::from(keyword.to_owned()), Value::from(pos)];
        let result = self.client.call("get_result_at", args);
        Window::from_msgpack_value(result.clone())
    }

    pub fn get_all_bursted_results(&mut self) -> HashMap<String, Window> {
        let args: Vec<Value> = vec![];
        let result = self.client.call("get_all_bursted_results", args);
        result.as_map()
              .unwrap()
              .iter()
              .map(|m| {
                       let (ref k, ref v): (Value, Value) = *m;
                       (k.as_str().unwrap().to_string(), Window::from_msgpack_value(v.clone()))
                   })
              .collect::<HashMap<String, Window>>()
    }

    pub fn get_all_bursted_results_at(&mut self, pos: f64) -> HashMap<String, Window> {
        let args: Vec<Value> = vec![Value::from(pos)];
        let result = self.client.call("get_all_bursted_results_at", args);
        result.as_map()
              .unwrap()
              .iter()
              .map(|m| {
                       let (ref k, ref v): (Value, Value) = *m;
                       (k.as_str().unwrap().to_string(), Window::from_msgpack_value(v.clone()))
                   })
              .collect::<HashMap<String, Window>>()
    }

    pub fn get_all_keywords(&mut self) -> Vec<KeywordWithParams> {
        let args: Vec<Value> = vec![];
        let result = self.client.call("get_all_keywords", args);
        result.as_array()
              .unwrap()
              .iter()
              .map(|x| KeywordWithParams::from_msgpack_value(x.clone()))
              .collect()
    }

    pub fn add_keyword(&mut self, keyword: KeywordWithParams) -> bool {
        let args: Vec<Value> = vec![keyword.to_msgpack_value()];
        let result = self.client.call("add_keyword", args);
        result.as_bool().unwrap()
    }

    pub fn remove_keyword(&mut self, keyword: String) -> bool {
        let args: Vec<Value> = vec![Value::from(keyword.to_owned())];
        let result = self.client.call("remove_keyword", args);
        result.as_bool().unwrap()
    }

    pub fn remove_all_keywords(&mut self) -> bool {
        let args: Vec<Value> = vec![];
        let result = self.client.call("remove_all_keywords", args);
        result.as_bool().unwrap()
    }

    pub fn clear(&mut self) -> bool {
        let args: Vec<Value> = vec![];
        let result = self.client.call("clear", args);
        result.as_bool().unwrap()
    }

    pub fn save(&mut self, id: String) -> HashMap<String, String> {
        let args: Vec<Value> = vec![Value::from(id)];
        let result = self.client.call("save", args);
        let mut ret: HashMap<String, String> = HashMap::new();
        for r in result.as_map().unwrap().iter() {
            let (ref k, ref v): (Value, Value) = *r;
            ret.insert(k.as_str().unwrap().to_string(),
                       v.as_str().unwrap().to_string());
        }
        ret
    }

    pub fn load(&mut self, id: String) -> bool {
        let args: Vec<Value> = vec![Value::from(id)];
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
                hh.insert(kkk.as_str().unwrap().to_string(),
                          vvv.as_str().unwrap().to_string());
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
                hh.insert(kkk.as_str().unwrap().to_string(),
                          vvv.as_str().unwrap().to_string());
            }
            ret.insert(kk.as_str().unwrap().to_string(), hh);
        }
        ret
    }

    pub fn get_name(&self) -> &str {
        self.client.name
    }

    pub fn set_name(&mut self, new_name: &'a str) {
        self.client.name = new_name;
    }
}
