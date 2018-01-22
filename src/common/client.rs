use rpc;
use rmpv::Value;

pub struct Client<'a> {
    client: rpc::Client,
    pub name: &'a str,
}

impl<'a> Client<'a> {
    pub fn new(host: &str, name: &'a str) -> Self {
        Client { name: name,
                 client: rpc::Client::connect_socket(host), }
    }

    pub fn call(&mut self, method: &str, arg: Vec<Value>) -> Value {
        let mut v: Vec<Value> = vec![Value::from(self.name.to_string())];
        for x in arg {
            v.push(x);
        }
        let result = self.client.call(method, v);
        match result {
            Ok(r) => r,
            Err(e) => {
                println!("result error: {:?}", e);
                Value::Nil
            }
        }
    }
}
