extern crate lib;

use lib::Person;

fn main() {
    let luc = Person::new("Luc", 35);
    println!("Name of person: {}", luc.name);
}