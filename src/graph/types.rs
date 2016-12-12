// This file is auto-generated from graph.idl(0.6.4-33-gcc8d7ca9) with jenerator version 0.9.4-42-g70f75391/master
// *** DO NOT EDIT ***

use std::collections::HashMap;
use rustc_serialize::{Encodable, Decodable};
use common::datum::Datum;
use msgpack::Value;
use msgpack::value::Float;
use msgpack::value::Integer;

#[derive(RustcEncodable, RustcDecodable, Default, Debug, Clone)]
pub struct Node {
    pub property: HashMap<String, String>,
    pub in_edges: Vec<u64>,
    pub out_edges: Vec<u64>,
}

impl Node {
    pub fn to_msgpack_value(&self) -> Value {
        Value::Array(vec![Value::Map(self.property
                              .iter()
                              .map(|k, v| (Value::String(k.to_owned()), Value::String(v.to_owned())))
                              .collect()),
                          Value::Array(self.in_edges
                              .iter()
                              .map(|x| Value::Integer(Integer::U64(x)))
                              .collect()),
                          Value::Array(self.out_edges
                              .iter()
                              .map(|x| Value::Integer(Integer::U64(x)))
                              .collect())])
    }

    pub fn from_msgpack_value(data: Value) -> Node {
        let s = data.as_array().unwrap();
        Node {
            property: s[0].as_map().unwrap().iter().map(|k, v| {
                h.insert(k.as_str()
                             .unwrap()
                             .to_string(),
                         v.as_str().unwrap().to_string())
            }),
            in_edges: s[1].as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_u64().unwrap())
                .collect(),
            out_edges: s[2].as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_u64().unwrap())
                .collect(),
        }
    }
}

#[derive(RustcEncodable, RustcDecodable, Default, Debug, Clone)]
pub struct Query {
    pub from_id: String,
    pub to_id: String,
}

impl Query {
    pub fn to_msgpack_value(&self) -> Value {
        Value::Array(vec![Value::String(self.from_id.to_owned()), Value::String(self.to_id.to_owned())])
    }

    pub fn from_msgpack_value(data: Value) -> Query {
        let s = data.as_array().unwrap();
        Query {
            from_id: s[0].as_str().unwrap().to_string(),
            to_id: s[1].as_str().unwrap().to_string(),
        }
    }
}

#[derive(RustcEncodable, RustcDecodable, Default, Debug, Clone)]
pub struct PresetQuery {
    pub edge_query: Vec<Query>,
    pub node_query: Vec<Query>,
}

impl PresetQuery {
    pub fn to_msgpack_value(&self) -> Value {
        Value::Array(vec![Value::Array(self.edge_query
                              .iter()
                              .map(|x| x.to_msgpack_value())
                              .collect()),
                          Value::Array(self.node_query.iter().map(|x| x.to_msgpack_value()).collect())])
    }

    pub fn from_msgpack_value(data: Value) -> PresetQuery {
        let s = data.as_array().unwrap();
        PresetQuery {
            edge_query: s[0].as_array()
                .unwrap()
                .iter()
                .map(|x| Query::from_msgpack_value(x))
                .collect(),
            node_query: s[1].as_array()
                .unwrap()
                .iter()
                .map(|x| Query::from_msgpack_value(x))
                .collect(),
        }
    }
}

#[derive(RustcEncodable, RustcDecodable, Default, Debug, Clone)]
pub struct Edge {
    pub property: HashMap<String, String>,
    pub source: String,
    pub target: String,
}

impl Edge {
    pub fn to_msgpack_value(&self) -> Value {
        Value::Array(vec![Value::Map(self.property
                              .iter()
                              .map(|k, v| (Value::String(k.to_owned()), Value::String(v.to_owned())))
                              .collect()),
                          Value::String(self.source.to_owned()),
                          Value::String(self.target.to_owned())])
    }

    pub fn from_msgpack_value(data: Value) -> Edge {
        let s = data.as_array().unwrap();
        let h: HashMap<String, String> = HashMap::new();
        let _ = s[0].as_map().unwrap().iter().map(|m| {
            let (ref k, ref v) = *m;
            h.insert(k.as_str()
                         .unwrap()
                         .to_string(),
                     v.as_str().unwrap().to_string())
        });
        Edge {
            property: h,
            source: s[1].as_str().unwrap().to_string(),
            target: s[2].as_str().unwrap().to_string(),
        }
    }
}

#[derive(RustcEncodable, RustcDecodable, Default, Debug, Clone)]
pub struct ShortestPathQuery {
    pub source: String,
    pub target: String,
    pub max_hop: u64,
    pub query: PresetQuery,
}

impl ShortestPathQuery {
    pub fn to_msgpack_value(&self) -> Value {
        Value::Array(vec![Value::String(self.source.to_owned()),
                          Value::String(self.target.to_owned()),
                          Value::Integer(Integer::U64(self.max_hop)),
                          self.query.to_msgpack_value()])
    }

    pub fn from_msgpack_value(data: Value) -> ShortestPathQuery {
        let s = data.as_array().unwrap();
        ShortestPathQuery {
            source: s[0].as_str().unwrap().to_string(),
            target: s[1].as_str().unwrap().to_string(),
            max_hop: s[2].as_u64().unwrap(),
            query: PresetQuery::from_msgpack_value(s[3]),
        }
    }
}
