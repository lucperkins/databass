extern crate lib;

use lib::kv::GetRequest;

fn main() {
    let mut get_request = GetRequest::new();
    get_request.set_key("my-key".to_string());
    let key = get_request.get_key();
    println!("{}", key);
}