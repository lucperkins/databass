use std::thread;

use grpc::{RequestOptions, ServerBuilder, SingleResponse};
use kv::{DeleteRequest, GetRequest, GetResponse, PutRequest, WriteResponse};
use kv_grpc::{KV, KVServer};
use rocksdb::DB;

pub trait Runnable {
    fn start(self) -> ();
    fn stop(self)  -> ();
}

pub struct Node {
    server_builder: ServerBuilder
}

struct KVImpl;

impl KV for KVImpl {
    fn get(&self, _o: RequestOptions, req: GetRequest) -> SingleResponse<GetResponse> {
        let bucket = req.get_bucket();
        let key = req.get_key();
        let mut res = GetResponse::new();

        let path = format!("/tmp/buckets/{}", bucket);
        let db = DB::open_default(path).unwrap();

        match db.get(key.as_bytes()) {
            Ok(Some(vector)) => {
                res.set_payload(vector.to_vec());
            }
            Ok(None) => {
                res.set_payload(String::from("EMPTY").into_bytes());
            }
            Err(_) => {
                res.set_payload(String::from("ERROR").into_bytes());
            }
        }

        SingleResponse::completed(res)
    }

    fn put(&self, _o: RequestOptions, req: PutRequest) -> SingleResponse<WriteResponse> {
        let bucket = req.get_bucket();
        let key = req.get_key();
        let value = req.get_value();
        let mut res = WriteResponse::new();

        let path = format!("/tmp/buckets/{}", bucket);
        let db = DB::open_default(path).unwrap();
        match db.put(key.as_bytes(), value) {
            Ok(()) => res.set_message(String::from("SUCCESS")),
            Err(_) => res.set_message(String::from("FAILURE"))
        }

        SingleResponse::completed(res)
    }

    fn delete(&self, _o: RequestOptions, req: DeleteRequest) -> SingleResponse<WriteResponse> {
        let bucket = req.get_bucket();
        let key = req.get_key();
        let mut res = WriteResponse::new();

        let path = format!("/tmp/buckets/{}", bucket);
        let db = DB::open_default(path).unwrap();

        match db.delete(key.as_bytes()) {
            Ok(()) => res.set_message(String::from("SUCCESS")),
            Err(_) => res.set_message(String::from("FAILURE"))
        }

        SingleResponse::completed(res)
    }
}

impl Node {
    pub fn new(port: u16) -> Self {
        let mut builder = ServerBuilder::new_plain();
        builder.http.set_port(port);
        builder.add_service(KVServer::new_service_def(KVImpl));

        Node {
            server_builder: builder
        }
    }
}

impl Runnable for Node {
    fn start(self) -> () {
        let _server = self.server_builder.build().expect("server");

        loop {
            thread::park();
        }
    }

    fn stop(self) -> () {
       
    }
}