mod city;
mod route;
mod ticket;
use serde_json::Value;
use std::collections::HashMap;

fn main() {
    let jawn: HashMap<String, route::L1>  = route::routes_from_file("mattgawarecki-ticket-to-ride/usa.routes.json");
    let elone: &route::L1 = jawn.get("Chicago").unwrap();
    println!("{:?}", elone);
    for (k, v) in jawn {
        println!("{}", k);
        // println!("{:?}", v);
    }
}

