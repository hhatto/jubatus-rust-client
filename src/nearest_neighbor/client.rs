// This file is auto-generated from nearest_neighbor.idl(0.8.2-20-g8e4dc3b5) with jenerator version 1.0.0-6-gebf1c263/support-rust-client-for-jenerator
// *** DO NOT EDIT ***


use std::collections::HashMap;
use msgpack::Value;
use msgpack::value::{Integer, Float};
use common::datum::Datum;
use common::client::Client;
use nearest_neighbor::types::*;
use rustc_serialize::Decodable;
use rmp_serialize::Decoder;

pub struct NearestNeighborClient<'a> {
    client: Client<'a>,
}

impl<'a> NearestNeighborClient<'a> {
    pub fn new(host: &str, name: &'a str) -> Self {
        let c = Client::new(host, name);
        NearestNeighborClient { client: c }
    }

    pub fn clear(&mut self) -> bool {
        let args: Vec<Value> = vec![];
        let result = self.client.call("clear", args);

        let ret = result.as_bool().unwrap();
        ret
    }

    pub fn set_row(&mut self, id: String, d: Datum) -> bool {
        let args: Vec<Value> = vec![Value::String(id.to_owned()), d.to_msgpack_value()];
        let result = self.client.call("set_row", args);

        let ret = result.as_bool().unwrap();
        ret
    }

    pub fn neighbor_row_from_id(&mut self, id: String, size: u64) -> Vec<IdWithScore> {
        let args: Vec<Value> = vec![Value::String(id.to_owned()), Value::Integer(Integer::U64(size))];
        let result = self.client.call("neighbor_row_from_id", args);

        let ret = result.as_array()
            .unwrap()
            .iter()
            .map(|x| IdWithScore::from_msgpack_value(x.clone()))
            .collect();
        ret
    }

    pub fn neighbor_row_from_datum(&mut self, query: Datum, size: u64) -> Vec<IdWithScore> {
        let args: Vec<Value> = vec![query.to_msgpack_value(), Value::Integer(Integer::U64(size))];
        let result = self.client.call("neighbor_row_from_datum", args);

        let ret = result.as_array()
            .unwrap()
            .iter()
            .map(|x| IdWithScore::from_msgpack_value(x.clone()))
            .collect();
        ret
    }

    pub fn similar_row_from_id(&mut self, id: String, ret_num: u64) -> Vec<IdWithScore> {
        let args: Vec<Value> = vec![Value::String(id.to_owned()), Value::Integer(Integer::U64(ret_num))];
        let result = self.client.call("similar_row_from_id", args);

        let ret = result.as_array()
            .unwrap()
            .iter()
            .map(|x| IdWithScore::from_msgpack_value(x.clone()))
            .collect();
        ret
    }

    pub fn similar_row_from_datum(&mut self, query: Datum, ret_num: u64) -> Vec<IdWithScore> {
        let args: Vec<Value> = vec![query.to_msgpack_value(), Value::Integer(Integer::U64(ret_num))];
        let result = self.client.call("similar_row_from_datum", args);

        let ret = result.as_array()
            .unwrap()
            .iter()
            .map(|x| IdWithScore::from_msgpack_value(x.clone()))
            .collect();
        ret
    }

    pub fn get_all_rows(&mut self) -> Vec<String> {
        let args: Vec<Value> = vec![];
        let result = self.client.call("get_all_rows", args);

        let ret = result.as_array()
            .unwrap()
            .iter()
            .map(|x| {
                x.as_str()
                    .unwrap()
                    .to_string()
            })
            .collect();
        ret
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
