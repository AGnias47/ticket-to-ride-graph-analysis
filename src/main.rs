mod city;
mod route;
mod ticket;
use std::collections::HashMap;

fn main() {
    let routes: HashMap<String, route::L1>  = route::routes_from_file("mattgawarecki-ticket-to-ride/usa.routes.json");
    println!("---Cities---");
    for (k, _) in &routes {
        println!("{}", k);
    }
    let chicago: &HashMap<String, route::L2> = &routes.get("Chicago").unwrap().destination_city;
    println!("---Destinations from Chicago---");
    for (k, _) in chicago {
        println!("{}", k);
    }
    let to_omaha: &route::L2 = chicago.get("Omaha").unwrap();
    println!("---Data on Route to Omaha---");
    println!("Distance: {}", to_omaha.distance);
    print!("Connections: ");
    for c in &to_omaha.connections {
        println!("{} ", c.color);
    }
}

