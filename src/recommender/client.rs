// This file is auto-generated from recommender.idl(1.0.0-112-g051a909c) with jenerator version 1.0.0-76-g95eed232/support-rust-client-for-jenerator
// *** DO NOT EDIT ***

use std::collections::HashMap;
use rmpv::Value;
use common::datum::Datum;
use common::client::Client;
use recommender::types::*;

pub struct RecommenderClient<'a> {
    client: Client<'a>,
}

impl<'a> RecommenderClient<'a> {
    pub fn new(host: &str, name: &'a str) -> Self {
        let c = Client::new(host, name);
        RecommenderClient { client: c }
    }

    pub fn clear_row(&mut self, id: String) -> bool {
        let args: Vec<Value> = vec![Value::from(id.to_owned())];
        let result = self.client.call("clear_row", args);
        result.as_bool().unwrap()
    }

    pub fn update_row(&mut self, id: String, row: Datum) -> bool {
        let args: Vec<Value> = vec![Value::from(id.to_owned()), row.to_msgpack_value()];
        let result = self.client.call("update_row", args);
        result.as_bool().unwrap()
    }

    pub fn clear(&mut self) -> bool {
        let args: Vec<Value> = vec![];
        let result = self.client.call("clear", args);
        result.as_bool().unwrap()
    }

    pub fn complete_row_from_id(&mut self, id: String) -> Datum {
        let args: Vec<Value> = vec![Value::from(id.to_owned())];
        let result = self.client.call("complete_row_from_id", args);
        Datum::from_msgpack_value(result.clone())
    }

    pub fn complete_row_from_datum(&mut self, row: Datum) -> Datum {
        let args: Vec<Value> = vec![row.to_msgpack_value()];
        let result = self.client.call("complete_row_from_datum", args);
        Datum::from_msgpack_value(result.clone())
    }

    pub fn similar_row_from_id(&mut self, id: String, size: u64) -> Vec<IdWithScore> {
        let args: Vec<Value> = vec![Value::from(id.to_owned()), Value::from(size)];
        let result = self.client.call("similar_row_from_id", args);
        result.as_array()
              .unwrap()
              .iter()
              .map(|x| IdWithScore::from_msgpack_value(x.clone()))
              .collect()
    }

    pub fn similar_row_from_id_and_score(&mut self, id: String, score: f64) -> Vec<IdWithScore> {
        let args: Vec<Value> = vec![Value::from(id.to_owned()), Value::from(score)];
        let result = self.client.call("similar_row_from_id_and_score", args);
        result.as_array()
              .unwrap()
              .iter()
              .map(|x| IdWithScore::from_msgpack_value(x.clone()))
              .collect()
    }

    pub fn similar_row_from_id_and_rate(&mut self, id: String, rate: f64) -> Vec<IdWithScore> {
        let args: Vec<Value> = vec![Value::from(id.to_owned()), Value::from(rate)];
        let result = self.client.call("similar_row_from_id_and_rate", args);
        result.as_array()
              .unwrap()
              .iter()
              .map(|x| IdWithScore::from_msgpack_value(x.clone()))
              .collect()
    }

    pub fn similar_row_from_datum(&mut self, row: Datum, size: u64) -> Vec<IdWithScore> {
        let args: Vec<Value> = vec![row.to_msgpack_value(), Value::from(size)];
        let result = self.client.call("similar_row_from_datum", args);
        result.as_array()
              .unwrap()
              .iter()
              .map(|x| IdWithScore::from_msgpack_value(x.clone()))
              .collect()
    }

    pub fn similar_row_from_datum_and_score(&mut self, row: Datum, score: f64) -> Vec<IdWithScore> {
        let args: Vec<Value> = vec![row.to_msgpack_value(), Value::from(score)];
        let result = self.client.call("similar_row_from_datum_and_score", args);
        result.as_array()
              .unwrap()
              .iter()
              .map(|x| IdWithScore::from_msgpack_value(x.clone()))
              .collect()
    }

    pub fn similar_row_from_datum_and_rate(&mut self, row: Datum, rate: f64) -> Vec<IdWithScore> {
        let args: Vec<Value> = vec![row.to_msgpack_value(), Value::from(rate)];
        let result = self.client.call("similar_row_from_datum_and_rate", args);
        result.as_array()
              .unwrap()
              .iter()
              .map(|x| IdWithScore::from_msgpack_value(x.clone()))
              .collect()
    }

    pub fn decode_row(&mut self, id: String) -> Datum {
        let args: Vec<Value> = vec![Value::from(id.to_owned())];
        let result = self.client.call("decode_row", args);
        Datum::from_msgpack_value(result.clone())
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

    pub fn calc_similarity(&mut self, lhs: Datum, rhs: Datum) -> f64 {
        let args: Vec<Value> = vec![lhs.to_msgpack_value(), rhs.to_msgpack_value()];
        let result = self.client.call("calc_similarity", args);
        result.as_f64().unwrap()
    }

    pub fn calc_l2norm(&mut self, row: Datum) -> f64 {
        let args: Vec<Value> = vec![row.to_msgpack_value()];
        let result = self.client.call("calc_l2norm", args);
        result.as_f64().unwrap()
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
