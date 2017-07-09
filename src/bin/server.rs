extern crate lib;

use lib::node::{Node,Runnable};

fn main() {
    let node1 = Node::new(1111);
    node1.start();
}