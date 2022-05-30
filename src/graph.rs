use super::matrix::{route_file_to_matrix, Matrix};
use super::serializers::L1;
use super::validate::valid_cities;
use std::cmp::{Eq, PartialEq};
use std::collections::{HashMap, HashSet};
use std::fs;

enum GraphColor {
    White,
    Grey,
    Black,
}

#[derive(Eq, PartialEq, Hash)]
struct Edge {
    origin: String,
    destination: String,
    weight: u8,
}

#[derive(Eq, PartialEq, Hash)]
struct Vertex {
    city: String,
}

struct Graph {
    adj_matrix: Matrix,
    vertices: HashSet<Vertex>,
    edges: HashSet<Edge>,
}

impl Graph {
    pub fn new() -> Graph {
        let mut point_matrix: Matrix = Matrix::new();
        let mut color_matrix: Matrix = Matrix::new();
        let route_file_as_string =
            fs::read_to_string("mattgawarecki-ticket-to-ride/usa.routes.json")
                .expect("Unable to read file");
        let data: HashMap<String, L1> = serde_json::from_str(&route_file_as_string).unwrap();
        let mut verticies: HashSet<Vertex> = HashSet::new();
        let mut edges: HashSet<Edge> = HashSet::new();
        for (starting_city, destination_cities) in &data {
            verticies.insert(Vertex {
                city: starting_city.clone(),
            });
            for (destination_city, route_data) in &destination_cities.destination_city {
                edges.insert(Edge {
                    origin: starting_city.clone(),
                    destination: destination_city.clone(),
                    weight: route_data.distance,
                });
                point_matrix.set(
                    starting_city.clone(),
                    destination_city.clone(),
                    route_data.distance,
                );
                let mut conn: Vec<String> = Vec::new();
                for c in &route_data.connections {
                    conn.push(c.color.clone());
                }
                color_matrix.set(starting_city.clone(), destination_city.clone(), 0);
            }
        }
        return Graph {
            adj_matrix: point_matrix,
            vertices: verticies,
            edges: edges,
        };
    }
}

pub fn demo() {
    let graph = Graph::new();
    for vertex in graph.vertices {
        println!("{}", vertex.city);
    }
    for edge in graph.edges {
        println!("{} to {}, {}", edge.origin, edge.destination, edge.weight);
    }
    graph.adj_matrix.print();
}
