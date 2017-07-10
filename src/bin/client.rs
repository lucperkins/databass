extern crate futures;
extern crate grpc;
extern crate lib;

use grpc::{RequestOptions,SingleResponse};
use lib::kv::{DeleteRequest,GetRequest,GetResponse,PutRequest,WriteResponse};
use lib::kv_grpc::{KV,KVClient};

fn get(client: &KVClient, bucket: &str, key: &str) -> () {
    let mut get_req: GetRequest = GetRequest::new();
    get_req.set_bucket(String::from(bucket));
    get_req.set_key(String::from(key));
    let get_resp: SingleResponse<GetResponse> = client.get(RequestOptions::new(), get_req);
    println!("{:?}", get_resp.wait());
}

fn put(client: &KVClient, bucket: &str, key: &str, value: &[u8]) -> () {
    let mut put_req: PutRequest = PutRequest::new();
    put_req.set_bucket(String::from(bucket));
    put_req.set_key(String::from(key));
    put_req.set_value(value.to_vec());
    let put_resp: SingleResponse<WriteResponse> = client.put(RequestOptions::new(), put_req);
    println!("{:?}", put_resp.wait());
}

fn delete(client: &KVClient, bucket: &str, key: &str) -> () {
    let mut delete_req: DeleteRequest = DeleteRequest::new();
    delete_req.set_bucket(String::from(bucket));
    delete_req.set_key(String::from(key));
    let delete_resp: SingleResponse<WriteResponse> = client.delete(RequestOptions::new(), delete_req);
    println!("{:?}", delete_resp.wait());
}

fn main() {
    let bucket = "my-bucket";
    let key = "my-key";
    let client = KVClient::new_plain("localhost", 1111, Default::default()).unwrap();
    println!("Deleting...");
    delete(&client, bucket, key);
    println!("Failed get....");
    get(&client, bucket, key);
    println!("Putting...");
    put(&client, bucket, key, "First value".as_bytes());
    println!("Successful get...");
    get(&client, bucket, key);
    println!("Failed get...");
    get(&client, "other-bucket", "other-key");
    println!("Another put...");
    put(&client, "other-bucket", "other-key", "Other value".as_bytes());
    println!("Another successful get...");
    get(&client, "other-bucket", "other-key");
    println!("Cleaning up...");
    delete(&client, bucket, key);
    delete(&client, "other-bucket", "other-key");
}