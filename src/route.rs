use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::clone::Clone;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Hash)]
pub struct Route {
    pub source: String,
    pub destination: String,
    pub distance: u8,
    pub connections: Vec<String>,
}


#[derive(Debug, Deserialize)]
pub struct L1 {
    #[serde(flatten)]
    destination_city: HashMap<String, L2>,
}

#[derive(Debug, Deserialize)]
struct L2 {
    distance: u8,
    connections: Vec<L3>,
}

#[derive(Debug, Deserialize, Clone)]
struct L3 {
    color: String,
    locomotives: u8,
    tunnels: u8,
}

fn route_file_to_hashmap(fpath: &str) -> HashMap<String, L1>  {
    let route_file_as_string = fs::read_to_string(fpath).expect("Unable to read file");
    let data: HashMap<String, L1> = serde_json::from_str(&route_file_as_string).unwrap();
    return data;
}

pub fn routes_from_file(fpath: &str) -> HashMap<String, L1>  {
    let route_file_as_map: HashMap<String, L1> = route_file_to_hashmap(fpath);
    return route_file_as_map;
}


pub fn demo() {
    let routes: HashMap<String, L1>  = routes_from_file("mattgawarecki-ticket-to-ride/usa.routes.json");
    println!("---Cities---");
    for (k, _) in &routes {
        println!("{}", k);
    }
    let chicago: &HashMap<String, L2> = &routes.get("Chicago").unwrap().destination_city;
    println!("---Destinations from Chicago---");
    for (k, _) in chicago {
        println!("{}", k);
    }
    let to_omaha: &L2 = chicago.get("Omaha").unwrap();
    println!("---Data on Route to Omaha---");
    println!("Distance: {}", to_omaha.distance);
    print!("Connections: ");
    for c in &to_omaha.connections {
        println!("{} ", c.color);
    }
}
