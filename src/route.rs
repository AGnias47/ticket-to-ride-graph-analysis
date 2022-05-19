use serde::Deserialize;
use std::clone::Clone;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Hash)]
pub struct Route {
    pub source: String,
    pub destination: String,
    pub distance: u8,
    pub connections: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct L1 {
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

/// Converts the Routes JSON into a hashmap structure
///
/// # Arguments
/// `fpath` - File path to JSON file
///
/// # Returns
/// Hash map where
/// * key: String corresponding to starting city
/// * value: Hash map where
///   * key: String corresponding to destination city
///   * value: Route object storing route info
///
pub fn route_file_to_hashmap(fpath: &str) -> HashMap<String, HashMap<String, Route>> {
    let route_file_as_string = fs::read_to_string(fpath).expect("Unable to read file");
    let data: HashMap<String, L1> = serde_json::from_str(&route_file_as_string).unwrap();
    let mut top_level_hashmap: HashMap<String, HashMap<String, Route>> = HashMap::new();
    for (starting_city, destination_cities) in &data {
        let mut individual_city_hashmap: HashMap<String, Route> = HashMap::new();
        for (destination_city, route_data) in &destination_cities.destination_city {
            let mut conn: Vec<String> = Vec::new();
            for c in &route_data.connections {
                conn.push(c.color.clone());
            }
            let r: Route = Route {
                source: starting_city.clone(),
                destination: destination_city.clone(),
                distance: route_data.distance,
                connections: conn,
            };
            individual_city_hashmap.insert(destination_city.clone(), r);
        }
        top_level_hashmap.insert(starting_city.clone(), individual_city_hashmap);
    }
    return top_level_hashmap;
}

/// Demos route JSON parsing
pub fn demo() {
    let map: HashMap<String, HashMap<String, Route>> =
        route_file_to_hashmap("mattgawarecki-ticket-to-ride/usa.routes.json");
    println!("---Cities---");
    for (k, _) in &map {
        println!("{}", k);
    }
    let chicago: &HashMap<String, Route> = &map.get("Chicago").unwrap();
    println!("---Destinations from Chicago---");
    for (k, _) in chicago {
        println!("{}", k);
    }
    let to_omaha: &Route = chicago.get("Omaha").unwrap();
    println!("---Data on Route to Omaha---");
    println!("Distance: {}", to_omaha.distance);
    print!("Connections: ");
    for c in &to_omaha.connections {
        println!("{} ", c);
    }
}
