extern crate futures;
extern crate grpc;
extern crate lib;

use lib::kv::{GetRequest,GetResponse};
use lib::kv_grpc::{KV,KVClient};

fn main() {
    let client = KVClient::new_plain("localhost", 1111, Default::default()).unwrap();
    let mut req = GetRequest::new();
    req.set_key(String::from("key"));
    let resp: grpc::SingleResponse<GetResponse> = client.get(grpc::RequestOptions::new(), req);
    println!("{:?}", resp.wait());
}