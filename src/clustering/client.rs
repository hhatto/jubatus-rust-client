// This file is auto-generated from clustering.idl(0.9.4-18-g4935b2bd) with jenerator version 1.0.0-27-ge6a9293f/support-rust-client-for-jenerator
// *** DO NOT EDIT ***

use std::collections::HashMap;
use rmpv::Value;
use common::datum::Datum;
use common::client::Client;
use clustering::types::*;

pub struct ClusteringClient<'a> {
    client: Client<'a>,
}

impl<'a> ClusteringClient<'a> {
    pub fn new(host: &str, name: &'a str) -> Self {
        let c = Client::new(host, name);
        ClusteringClient { client: c }
    }

    pub fn push(&mut self, points: Vec<IndexedPoint>) -> bool {
        let args: Vec<Value> = vec![Value::Array(points.iter().map(|x| x.to_msgpack_value()).collect())];
        let result = self.client.call("push", args);
        result.as_bool().unwrap()
    }

    pub fn get_revision(&mut self) -> u64 {
        let args: Vec<Value> = vec![];
        let result = self.client.call("get_revision", args);
        result.as_u64().unwrap()
    }

    pub fn get_core_members(&mut self) -> Vec<Vec<WeightedDatum>> {
        let args: Vec<Value> = vec![];
        let result = self.client.call("get_core_members", args);
        result.as_array()
              .unwrap()
              .iter()
              .map(|x| {
                       x.as_array().unwrap()
                        .iter()
                        .map(|x| WeightedDatum::from_msgpack_value(x.clone()))
                        .collect()
                   })
              .collect()
    }

    pub fn get_core_members_light(&mut self) -> Vec<Vec<WeightedIndex>> {
        let args: Vec<Value> = vec![];
        let result = self.client.call("get_core_members_light", args);
        result.as_array()
              .unwrap()
              .iter()
              .map(|x| {
                       x.as_array().unwrap()
                        .iter()
                        .map(|x| WeightedIndex::from_msgpack_value(x.clone()))
                        .collect()
                   })
              .collect()
    }

    pub fn get_k_center(&mut self) -> Vec<Datum> {
        let args: Vec<Value> = vec![];
        let result = self.client.call("get_k_center", args);
        result.as_array()
              .unwrap()
              .iter()
              .map(|x| Datum::from_msgpack_value(x.clone()))
              .collect()
    }

    pub fn get_nearest_center(&mut self, point: Datum) -> Datum {
        let args: Vec<Value> = vec![point.to_msgpack_value()];
        let result = self.client.call("get_nearest_center", args);
        Datum::from_msgpack_value(result.clone())
    }

    pub fn get_nearest_members(&mut self, point: Datum) -> Vec<WeightedDatum> {
        let args: Vec<Value> = vec![point.to_msgpack_value()];
        let result = self.client.call("get_nearest_members", args);
        result.as_array()
              .unwrap()
              .iter()
              .map(|x| WeightedDatum::from_msgpack_value(x.clone()))
              .collect()
    }

    pub fn get_nearest_members_light(&mut self, point: Datum) -> Vec<WeightedIndex> {
        let args: Vec<Value> = vec![point.to_msgpack_value()];
        let result = self.client.call("get_nearest_members_light", args);
        result.as_array()
              .unwrap()
              .iter()
              .map(|x| WeightedIndex::from_msgpack_value(x.clone()))
              .collect()
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
            ret.insert(k.as_str().unwrap().to_string(),
                       v.as_str().unwrap().to_string());
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
        return self.client.name;
    }

    pub fn set_name(&mut self, new_name: &'a str) {
        self.client.name = new_name;
    }
}
