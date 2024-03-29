use super::matrix::Matrix;
use super::serializers::L1;
use std::cmp::{Eq, PartialEq};
use std::collections::{HashMap, HashSet};
use std::fs;
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
    pub city: String,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct DijkstraVertex {
    pub city: String,
    pub weight: u8,
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

    pub fn size(&self) -> usize {
        return self.vertices.len();
    }

    pub fn bfs(&self, s: Vertex) -> (HashMap<Vertex, Option<Vertex>>, HashMap<Vertex, i8>) {
        let mut colors: HashMap<Vertex, GraphColor> = HashMap::new();
        let mut predecessor: HashMap<Vertex, Option<Vertex>> = HashMap::new();
        let mut distance: HashMap<Vertex, i8> = HashMap::new();
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
        }
        return (predecessor, distance);
    }

    pub fn dijkstra_ssp(&self, s: Vertex) -> HashMap<String, u8> {
        let mut q: Queue<DijkstraVertex> = queue![];
        let mut dijkstra_vertices: Vec<DijkstraVertex> = Vec::new();
        for v in &self.vertices {
            let mut v_struct: DijkstraVertex = DijkstraVertex {
                city: String::from(v.clone().city),
                weight: u8::MAX,
            };
            if v_struct.city == s.clone().city {
                v_struct.weight = 0;
                q.add(v_struct.clone())
                    .expect("Could not add vertex to queue");
            }
            dijkstra_vertices.push(v_struct);
        }
        while q.size() > 0 {
            let u: DijkstraVertex = q.remove().expect("Could not pop vertex from queue");
            for v in &mut dijkstra_vertices {
                let weight: u8 = self.adj_matrix.get(u.city.clone(), v.city.clone()).unwrap();
                if weight != 0 {
                    if v.weight == u8::MAX {
                        v.weight = u.weight + weight;
                        q.add(v.clone()).expect("Could not add vertex to queue");
                    } else {
                        let new_weight: u8 = u.weight + weight;
                        if new_weight < v.weight {
                            v.weight = new_weight;
                        }
                    }
                }
            }
        }
        let mut ssp_map: HashMap<String, u8> = HashMap::new();
        for v in dijkstra_vertices {
            ssp_map.insert(v.clone().city, v.clone().weight);
        }
        return ssp_map;
    }
}

pub fn distance_from_bfs_origin(
    from: Vertex,
    predecessor: HashMap<Vertex, Option<Vertex>>,
    distance: HashMap<Vertex, i8>,
) -> u8 {
    let mut distance_to_origin: i8 = 0;
    let mut predecessor_option: Option<Vertex> = match predecessor.get(&from) {
        Some(val) => val.clone(),
        None => None,
    };
    let mut destination: Vertex = from;
    while predecessor_option.is_some() {
        distance_to_origin += distance.get(&destination).unwrap().clone();
        predecessor_option = match predecessor.get(&predecessor_option.unwrap()) {
            Some(val) => val.clone(),
            None => None,
        };
        if predecessor_option.is_some() {
            destination = predecessor_option.clone().unwrap();
        }
    }
    return distance_to_origin as u8;
}

pub fn demo() {
    let graph = Graph::new();
    let v: Vertex = Vertex {
        city: "New York".to_string(),
    };
    let (predecessor, distance) = graph.bfs(v.clone());
    let destination: Vertex = Vertex {
        city: "Nashville".to_string(),
    };
    println!(
        "Distance from New York to Nashville (from BFS): {}",
        distance_from_bfs_origin(destination.clone(), predecessor, distance)
    );

    let ssp_map = graph.dijkstra_ssp(v.clone());

    println!(
        "Distance from New York to Nashville (from Dijkstra SSP): {}",
        ssp_map.get(&destination.clone().city).unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bfs() {
        let graph = Graph::new();
        let origin: Vertex = Vertex {
            city: "New York".to_string(),
        };
        let (predecessor, distance) = graph.bfs(origin);
        let boston: Vertex = Vertex {
            city: "Boston".to_string(),
        };
        let nashville: Vertex = Vertex {
            city: "Nashville".to_string(),
        };
        let ssm: Vertex = Vertex {
            city: "Sault Ste. Marie".to_string(),
        };
        assert_eq!(
            2,
            distance_from_bfs_origin(boston, predecessor.clone(), distance.clone())
        );
        assert_eq!(
            6,
            distance_from_bfs_origin(nashville, predecessor.clone(), distance.clone())
        );
        assert_eq!(
            8,
            distance_from_bfs_origin(ssm, predecessor.clone(), distance.clone())
        );
    }
}
