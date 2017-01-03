extern crate rmp as msgpack;
extern crate msgpack_rpc as rpc;
extern crate rmp_serialize;

pub mod common;
pub mod anomaly;
pub mod bandit;
pub mod burst;
pub mod classifier;
pub mod clustering;
pub mod graph;
pub mod nearest_neighbor;
pub mod recommender;
pub mod regression;
pub mod stat;
pub mod weight;

#[cfg(test)]
mod tests {
    use common::Datum;
    use common::client::Client;
    use classi::client::ClassifierClient;

    #[test]
    fn test_common_smoke_datam() {
        let mut d = Datum::new();
        d.add_string("k", "v");
        assert!(d.string_values().len() == 1);

        d.add_number("knum", 1);
        assert!(d.num_values().len() == 1);

        d.add_binary("kbin", vec![1, 2]);
        assert!(d.binary_values().len() == 1);
    }

    #[test]
    fn test_common_smoke_client() {
        let client = Client::new("localhost:9199", "classifier");
        let result = client.do_mix();
        println!("{:?}", result);
    }

    #[test]
    fn test_common_smoke_client_get_config() {
        let client = Client::new("127.0.0.1:9199", "classifier");
        let result = client.get_config();
        println!("{:?}", result);
    }

    #[test]
    fn test_classify_smoke_client_classify() {
        let client = ClassifierClient::new("127.0.0.1:9199", "classi");
        let result = client.classify();
    }
}
