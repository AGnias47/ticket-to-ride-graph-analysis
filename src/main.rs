mod city;
mod route;
mod ticket;
use serde_json::Value;
use std::collections::HashMap;

fn main() {
    let jawn: HashMap<String, Value> = route::routes_from_file("mattgawarecki-ticket-to-ride/usa.routes.json");
    println!("{:#?}", jawn.get("Winnipeg"));
    println!("Hello, world!");
}

