use rmpv::Value;

#[derive(Debug, Default, Clone)]
pub struct Datum {
    string_values: Vec<(String, String)>,
    num_values: Vec<(String, f64)>,
    binary_values: Vec<(String, Vec<u8>)>,
}

impl Datum {
    pub fn new() -> Self {
        Datum { string_values: vec![],
                num_values: vec![],
                binary_values: vec![], }
    }

    pub fn from_msgpack_value(data: Value) -> Datum {
        let mut s: Vec<(String, String)> = vec![];
        let mut n: Vec<(String, f64)> = vec![];
        let mut b: Vec<(String, Vec<u8>)> = vec![];
        let datum = data.as_array().unwrap();
        for d in datum[0].as_array().unwrap().iter() {
            let sv = d.as_array().unwrap();
            s.push((sv[0].as_str().unwrap().to_string(), sv[1].to_string()));
        }
        for d in datum[1].as_array().unwrap().iter() {
            let sv = d.as_array().unwrap();
            n.push((sv[0].as_str().unwrap().to_string(), sv[1].as_f64().unwrap()));
        }
        for d in datum[2].as_array().unwrap().iter() {
            let sv = d.as_array().unwrap();
            let bin = sv[1].as_slice().unwrap();
            b.push((sv[0].as_str().unwrap().to_string(), Vec::from(bin)));
        }
        Datum { string_values: s,
                num_values: n,
                binary_values: b, }
    }

    pub fn to_msgpack_value(&self) -> Value {
        let mut datum: Vec<Value> = vec![];
        let mut s: Vec<Value> = vec![];
        let mut n: Vec<Value> = vec![];
        let mut b: Vec<Value> = vec![];
        for v in self.string_values.iter() {
            s.push(Value::Array(vec![Value::from(v.0.to_owned()), Value::from(v.1.to_owned())]));
        }
        for v in self.num_values.iter() {
            n.push(Value::Array(vec![Value::from(v.0.to_owned()), Value::from(v.1)]));
        }
        for v in self.binary_values.iter() {
            b.push(Value::Array(vec![Value::from(v.0.to_owned()), Value::Binary(v.1.to_owned())]));
        }
        datum.push(Value::Array(s));
        datum.push(Value::Array(n));
        datum.push(Value::Array(b));
        Value::Array(datum)
    }

    pub fn string_values(&self) -> &Vec<(String, String)> {
        &self.string_values
    }
    pub fn num_values(&self) -> &Vec<(String, f64)> {
        &self.num_values
    }
    pub fn binary_values(&self) -> &Vec<(String, Vec<u8>)> {
        &self.binary_values
    }

    pub fn add_string(&mut self, key: &str, value: &str) {
        self.string_values.push((key.to_string(), value.to_string()));
    }

    pub fn add_number(&mut self, key: &str, value: f64) {
        self.num_values.push((key.to_string(), value));
    }

    pub fn add_binary(&mut self, key: &str, value: Vec<u8>) {
        self.binary_values.push((key.to_string(), value.clone()));
    }
}
