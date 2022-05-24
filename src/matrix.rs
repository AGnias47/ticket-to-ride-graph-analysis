use super::serializers::L1;
use std::collections::HashMap;
use std::fs;
use std::convert::TryFrom;

const CITIES: usize = 36;
const CITIES_U8: u8 = 36;

/// Matrix implementation from https://www.reddit.com/r/rust/comments/icpdvh/rust_matrix_structure/
pub struct Matrix {
    rows: [[u8; CITIES]; CITIES],
}

impl Matrix {
    pub fn get(&self, row: usize, col: usize) -> Option<u8> {
        let val = self.rows.get(row)?.get(col)?;
        Some(*val)
    }

    pub fn set(&mut self, row: usize, col: usize, value: u8) {
        self.rows[row][col] = value;
    }

    pub fn print(&self) {
        for r in self.rows {
            print!("[");
            for (i, c) in r.iter().enumerate() {
                print!("{}", c);
                if c < &(CITIES as u8 - i as u8 - 1) {
                    print!(" ");
                }
            }
            println!("]");
        }
    }
}

pub fn init_matrix() -> Matrix {
    return Matrix{rows: [[0; CITIES]; CITIES]};
}

pub fn route_file_to_adjacency_matrix(fpath: &str) -> Matrix {
    let mut point_matrix: Matrix = init_matrix();
    let mut color_matrix: Matrix = init_matrix();
    let route_file_as_string = fs::read_to_string(fpath).expect("Unable to read file");
    let data: HashMap<String, L1> = serde_json::from_str(&route_file_as_string).unwrap();
    let mut i: usize = 0;
    // Need to correlate cities with an index; right now just throws them in starting from 0
    for (starting_city, destination_cities) in &data {
        let mut j: usize = 0;
        for (destination_city, route_data) in &destination_cities.destination_city {
            let mut conn: Vec<String> = Vec::new();
            for c in &route_data.connections {
                conn.push(c.color.clone());
            }
            point_matrix.set(i, j, route_data.distance);
            color_matrix.set(i, j, 0);
            j += 1;
        }
        i += 1;
    }
    return point_matrix;
}

pub fn demo() {
    let matrix: Matrix = route_file_to_adjacency_matrix("mattgawarecki-ticket-to-ride/usa.routes.json");
    matrix.print();
}