// This file is auto-generated from bandit.idl(0.7.2-79-g2db27d79) with jenerator version 0.9.4-42-g70f75391/master
// *** DO NOT EDIT ***


use std::collections::HashMap;
use msgpack::Value;
use msgpack::value::{Integer, Float};
use common::datum::Datum;
use common::client::Client;
use bandit::types::*;
use rustc_serialize::Decodable;
use rmp_serialize::Decoder;

pub struct BanditClient<'a> {
    client: Client<'a>,
}

impl<'a> BanditClient<'a> {
    pub fn new(host: &str, name: &'a str) -> Self {
        let c = Client::new(host, name);
        BanditClient { client: c }
    }

    pub fn register_arm(&mut self, arm_id: String) -> bool {
        let args: Vec<Value> = vec![Value::String(arm_id.to_owned())];
        let result = self.client.call("register_arm", args);
        let ret = result.as_bool().unwrap();
        return ret;
    }

    pub fn delete_arm(&mut self, arm_id: String) -> bool {
        let args: Vec<Value> = vec![Value::String(arm_id.to_owned())];
        let result = self.client.call("delete_arm", args);
        let ret = result.as_bool().unwrap();
        return ret;
    }

    pub fn select_arm(&mut self, player_id: String) -> String {
        let args: Vec<Value> = vec![Value::String(player_id.to_owned())];
        let result = self.client.call("select_arm", args);
        let ret = result.as_str().unwrap().to_string();
        return ret;
    }

    pub fn register_reward(&mut self, player_id: String, arm_id: String, reward: f64) -> bool {
        let args: Vec<Value> =
            vec![Value::String(player_id.to_owned()), Value::String(arm_id.to_owned()), Value::Float(Float::F64(reward))];
        let result = self.client.call("register_reward", args);
        let ret = result.as_bool().unwrap();
        return ret;
    }

    pub fn get_arm_info(&mut self, player_id: String) -> HashMap<String, ArmInfo> {
        let args: Vec<Value> = vec![Value::String(player_id.to_owned())];
        let result = self.client.call("get_arm_info", args);
        let ret = result.as_map().unwrap().iter().map(|m| {
            let (ref k, ref v): (Value, Value) = *m;
            h.insert(k.as_str().unwrap().to_string(), ArmInfo::from_msgpack_value(v.clone()))
        });
        return ret;
    }

    pub fn reset(&mut self, player_id: String) -> bool {
        let args: Vec<Value> = vec![Value::String(player_id.to_owned())];
        let result = self.client.call("reset", args);
        let ret = result.as_bool().unwrap();
        return ret;
    }

    pub fn clear(&mut self) -> bool {
        let args: Vec<Value> = vec![];
        let result = self.client.call("clear", args);
        let ret = result.as_bool().unwrap();
        return ret;
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
