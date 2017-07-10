extern crate futures;
extern crate grpc;
extern crate lib;

use lib::kv::{DeleteRequest,GetRequest,GetResponse,PutRequest,WriteResponse};
use lib::kv_grpc::{KV,KVClient};

use std::time;

fn main() {
    let client = KVClient::new_plain("localhost", 1111, Default::default()).unwrap();
    let mut put_req = PutRequest::new();
    put_req.set_bucket(String::from("my-bucket"));
    put_req.set_key(String::from("my-key"));
    put_req.set_value(String::from("Here is an initial value").into_bytes());

    let put_resp: grpc::SingleResponse<WriteResponse> = client.put(grpc::RequestOptions::new(), put_req);
    println!("{:?}", put_resp.wait());

    let mut get_req = GetRequest::new();
    get_req.set_bucket(String::from("my-bucket"));
    get_req.set_key(String::from("my-key"));
    let resp: grpc::SingleResponse<GetResponse> = client.get(grpc::RequestOptions::new(), get_req);
    println!("{:?}", resp.wait());

    let mut put_req2 = PutRequest::new();
    put_req2.set_bucket(String::from("my-bucket"));
    put_req2.set_key(String::from("my-key"));
    put_req2.set_value(String::from("Here is yet another new value").into_bytes());

    let put_resp2: grpc::SingleResponse<WriteResponse> = client.put(grpc::RequestOptions::new(), put_req2);
    println!("{:?}", put_resp2.wait());

    let mut get_req2 = GetRequest::new();
    get_req2.set_bucket(String::from("my-bucket"));
    get_req2.set_key(String::from("my-key"));
    let resp2: grpc::SingleResponse<GetResponse> = client.get(grpc::RequestOptions::new(), get_req2);
    println!("{:?}", resp2.wait());

    let mut delete_req = DeleteRequest::new();
    delete_req.set_bucket(String::from("my-bucket"));
    delete_req.set_key(String::from("my-key"));
    let resp3: grpc::SingleResponse<WriteResponse> = client.delete(grpc::RequestOptions::new(), delete_req);
    println!("{:?}", resp3.wait());

    let mut get_req3 = GetRequest::new();
    get_req3.set_bucket(String::from("my-bucket"));
    get_req3.set_key(String::from("my-key"));
    let resp4: grpc::SingleResponse<GetResponse> = client.get(grpc::RequestOptions::new(), get_req3);
    println!("{:?}", resp4.wait());
}