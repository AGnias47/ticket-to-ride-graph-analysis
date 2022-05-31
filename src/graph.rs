use super::matrix::{route_file_to_matrix, Matrix};
use super::serializers::L1;
use super::validate::valid_cities;
use std::cmp::{Eq, PartialEq};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::{Index, IndexMut};
extern crate queues;
use queues::*;

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum GraphColor {
    White,
    Grey,
    Black,
}

#[derive(Eq, PartialEq, Hash)]
pub struct Edge {
    origin: String,
    destination: String,
    weight: u8,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Vertex {
    city: String,
}

pub struct Graph {
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

    pub fn get_first_vertex(&self) -> Vertex {
        return self.vertices.iter().next().unwrap().clone();
    }

    pub fn BFS(
        &self,
        s: Vertex,
    ) -> (
        Vec<Vertex>,
        HashMap<Vertex, GraphColor>,
        HashMap<Vertex, Option<Vertex>>,
        HashMap<Vertex, i8>,
    ) {
        let mut colors: HashMap<Vertex, GraphColor> = HashMap::new();
        let mut predecessor: HashMap<Vertex, Option<Vertex>> = HashMap::new();
        let mut distance: HashMap<Vertex, i8> = HashMap::new();
        let mut order: Vec<Vertex> = Vec::new();
        for vertex in &self.vertices {
            colors.insert(vertex.clone(), GraphColor::White);
            distance.insert(vertex.clone(), -1);
        }
        colors.insert(s.clone(), GraphColor::Grey);
        distance.insert(s.clone(), 0);
        let mut q: Queue<Vertex> = queue![];
        q.add(s.clone()).expect("Could not add vertex to queue");
        while q.size() > 0 {
            let u: Vertex = q.remove().expect("Could not pop vertex from queue");
            for v in &self.vertices {
                let weight: u8 = self.adj_matrix.get(u.city.clone(), v.city.clone()).unwrap();
                if weight != 0 {
                    if colors.get(v).unwrap().clone() == GraphColor::White {
                        colors.insert(v.clone(), GraphColor::Grey);
                        distance.insert(v.clone(), distance.get(&u).unwrap() + weight as i8);
                        predecessor.insert(v.clone(), Some(u.clone()));
                        q.add(v.clone()).expect("Could not add vertex to queue");
                    }
                }
            }
            colors.insert(u.clone(), GraphColor::Black);
            order.push(u.clone());
        }
        return (order, colors, predecessor, distance);
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
