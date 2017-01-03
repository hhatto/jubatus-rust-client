// This file is auto-generated from graph.idl(0.6.4-33-gcc8d7ca9) with jenerator version 1.0.0-6-gebf1c263/support-rust-client-for-jenerator
// *** DO NOT EDIT ***


use std::collections::HashMap;
use msgpack::Value;
use msgpack::value::{Integer, Float};
use common::datum::Datum;
use common::client::Client;
use graph::types::*;
use rustc_serialize::Decodable;
use rmp_serialize::Decoder;

pub struct GraphClient<'a> {
    client: Client<'a>,
}

impl<'a> GraphClient<'a> {
    pub fn new(host: &str, name: &'a str) -> Self {
        let c = Client::new(host, name);
        GraphClient { client: c }
    }

    pub fn create_node(&mut self) -> String {
        let args: Vec<Value> = vec![];
        let result = self.client.call("create_node", args);

        let ret = result.as_str().unwrap().to_string();
        ret
    }

    pub fn remove_node(&mut self, node_id: String) -> bool {
        let args: Vec<Value> = vec![Value::String(node_id.to_owned())];
        let result = self.client.call("remove_node", args);

        let ret = result.as_bool().unwrap();
        ret
    }

    pub fn update_node(&mut self, node_id: String, property: HashMap<String, String>) -> bool {
        let args: Vec<Value> = vec![Value::String(node_id.to_owned()),
                                    Value::Map(property.iter()
                                        .map(|(k, v)| (Value::String(k.to_owned()), Value::String(v.to_owned())))
                                        .collect())];
        let result = self.client.call("update_node", args);

        let ret = result.as_bool().unwrap();
        ret
    }

    pub fn create_edge(&mut self, node_id: String, e: Edge) -> u64 {
        let args: Vec<Value> = vec![Value::String(node_id.to_owned()), e.to_msgpack_value()];
        let result = self.client.call("create_edge", args);

        let ret = result.as_u64().unwrap();
        ret
    }

    pub fn update_edge(&mut self, node_id: String, edge_id: u64, e: Edge) -> bool {
        let args: Vec<Value> = vec![Value::String(node_id.to_owned()), Value::Integer(Integer::U64(edge_id)), e.to_msgpack_value()];
        let result = self.client.call("update_edge", args);

        let ret = result.as_bool().unwrap();
        ret
    }

    pub fn remove_edge(&mut self, node_id: String, edge_id: u64) -> bool {
        let args: Vec<Value> = vec![Value::String(node_id.to_owned()), Value::Integer(Integer::U64(edge_id))];
        let result = self.client.call("remove_edge", args);

        let ret = result.as_bool().unwrap();
        ret
    }

    pub fn get_centrality(&mut self, node_id: String, centrality_type: i64, query: PresetQuery) -> f64 {
        let args: Vec<Value> =
            vec![Value::String(node_id.to_owned()), Value::Integer(Integer::I64(centrality_type)), query.to_msgpack_value()];
        let result = self.client.call("get_centrality", args);

        let ret = result.as_f64().unwrap();
        ret
    }

    pub fn add_centrality_query(&mut self, query: PresetQuery) -> bool {
        let args: Vec<Value> = vec![query.to_msgpack_value()];
        let result = self.client.call("add_centrality_query", args);

        let ret = result.as_bool().unwrap();
        ret
    }

    pub fn add_shortest_path_query(&mut self, query: PresetQuery) -> bool {
        let args: Vec<Value> = vec![query.to_msgpack_value()];
        let result = self.client.call("add_shortest_path_query", args);

        let ret = result.as_bool().unwrap();
        ret
    }

    pub fn remove_centrality_query(&mut self, query: PresetQuery) -> bool {
        let args: Vec<Value> = vec![query.to_msgpack_value()];
        let result = self.client.call("remove_centrality_query", args);

        let ret = result.as_bool().unwrap();
        ret
    }

    pub fn remove_shortest_path_query(&mut self, query: PresetQuery) -> bool {
        let args: Vec<Value> = vec![query.to_msgpack_value()];
        let result = self.client.call("remove_shortest_path_query", args);

        let ret = result.as_bool().unwrap();
        ret
    }

    pub fn get_shortest_path(&mut self, query: ShortestPathQuery) -> Vec<String> {
        let args: Vec<Value> = vec![query.to_msgpack_value()];
        let result = self.client.call("get_shortest_path", args);

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

    pub fn update_index(&mut self) -> bool {
        let args: Vec<Value> = vec![];
        let result = self.client.call("update_index", args);

        let ret = result.as_bool().unwrap();
        ret
    }

    pub fn clear(&mut self) -> bool {
        let args: Vec<Value> = vec![];
        let result = self.client.call("clear", args);

        let ret = result.as_bool().unwrap();
        ret
    }

    pub fn get_node(&mut self, node_id: String) -> Node {
        let args: Vec<Value> = vec![Value::String(node_id.to_owned())];
        let result = self.client.call("get_node", args);

        let ret = Node::from_msgpack_value(result.clone());
        ret
    }

    pub fn get_edge(&mut self, node_id: String, edge_id: u64) -> Edge {
        let args: Vec<Value> = vec![Value::String(node_id.to_owned()), Value::Integer(Integer::U64(edge_id))];
        let result = self.client.call("get_edge", args);

        let ret = Edge::from_msgpack_value(result.clone());
        ret
    }

    pub fn create_node_here(&mut self, node_id: String) -> bool {
        let args: Vec<Value> = vec![Value::String(node_id.to_owned())];
        let result = self.client.call("create_node_here", args);

        let ret = result.as_bool().unwrap();
        ret
    }

    pub fn remove_global_node(&mut self, node_id: String) -> bool {
        let args: Vec<Value> = vec![Value::String(node_id.to_owned())];
        let result = self.client.call("remove_global_node", args);

        let ret = result.as_bool().unwrap();
        ret
    }

    pub fn create_edge_here(&mut self, edge_id: u64, e: Edge) -> bool {
        let args: Vec<Value> = vec![Value::Integer(Integer::U64(edge_id)), e.to_msgpack_value()];
        let result = self.client.call("create_edge_here", args);

        let ret = result.as_bool().unwrap();
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
