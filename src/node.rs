use std::thread;

use grpc::{RequestOptions, Server, ServerBuilder, SingleResponse};
use kv::{GetRequest, GetResponse};
use kv_grpc::{KV, KVServer};

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
        let mut res = GetResponse::new();
        res.set_payload(String::from("here is a message").into_bytes());
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