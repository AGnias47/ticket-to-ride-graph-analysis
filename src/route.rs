use crate::city::City;
use std::fs;
use std::collections::HashMap;

pub struct Route {
    source: City,
    destination: City,
    distance: u8,
    connections: Vec<String>
}

pub fn routes_from_file(fpath: &str) -> HashMap<String, serde_json::Value> {
    let routes = fs::read_to_string(fpath)
    .expect("Unable to read file");
    let routes_map: HashMap<String, serde_json::Value> = serde_json::from_str(&routes)
    .expect("Unable to parse JSON");
    return routes_map;
}
