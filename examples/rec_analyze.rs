extern crate jubatus;

use jubatus::recommender::client::RecommenderClient;
use jubatus::common::datum::Datum;

fn complete(client: &mut RecommenderClient, id: &str) {
    let _ = client.complete_row_from_id(id.to_string());
}

fn main() {
    let host = "127.0.0.1:9199";
    let name = "test";

    let mut client = RecommenderClient::new(host, name);
    complete(&mut client, "yyy");
}
