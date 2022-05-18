use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::clone::Clone;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Hash)]
pub struct Route {
    source: String,
    destination: String,
    distance: u8,
    connections: Vec<String>,
}


#[derive(Debug, Deserialize)]
pub struct L1 {
    destination_city: Option<HashMap<String, Value>>,
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

fn route_file_to_L0(fpath: &str) -> HashMap<String, HashMap<String, Value>>  {
    let route_file_as_string = fs::read_to_string(fpath).expect("Unable to read file");
    let data: HashMap<String, HashMap<String, Value>> = serde_json::from_str(&route_file_as_string).unwrap();
    return data;
}

pub fn routes_from_file(fpath: &str) -> HashMap<String, HashMap<String, Value>>  {
    let route_file_as_map: HashMap<String, HashMap<String, Value>> = route_file_to_L0(fpath);
    return route_file_as_map;
    /*
    let mut final_routes_map: HashMap<String, Vec<Route>> = HashMap::new();
    for (starting_city, destinations) in &route_file_as_map.routes {
        println!("{}", starting_city);
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
    */
}
