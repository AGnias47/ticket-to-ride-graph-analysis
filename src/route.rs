use crate::city::City;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::clone::Clone;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Hash)]
pub struct Route {
    source: City,
    destination: City,
    distance: u8,
    connections: Vec<String>,
}

#[derive(Deserialize)]
struct L1 {
    destination_city: HashMap<City, L2>,
}

#[derive(Deserialize)]
struct L2 {
    distance: u8,
    connections: Vec<L3>,
}

#[derive(Deserialize, Clone)]
struct L3 {
    color: String,
    locomotives: u8,
    tunnels: u8,
}

pub fn routes_from_file(fpath: &str) -> HashMap<City, Vec<Route>> {
    let route_file_as_string = fs::read_to_string(fpath).expect("Unable to read file");
    let route_file_as_map: HashMap<City, L1> = serde_json::from_str(&route_file_as_string).unwrap();
    let mut final_routes_map: HashMap<City, Vec<Route>> = HashMap::new();
    for (starting_city, destinations) in &route_file_as_map {
        let mut routes_for_city: Vec<Route> = Vec::new();
        for (destination, l2) in &destinations.destination_city {
            let mut colors: Vec<String> = Vec::new();
            for c in l2.connections.clone() {
                colors.push(c.color);
            }
            let route: Route = Route {
                source: (*starting_city).clone(),
                destination: (*destination).clone(),
                distance: l2.distance,
                connections: colors.clone()
            };
            routes_for_city.push(route);
        }
        final_routes_map.insert((*starting_city).clone(), routes_for_city);
    }
    return final_routes_map;
}
