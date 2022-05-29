use super::serializers::L1;
use super::validate::valid_cities;
use std::collections::HashMap;
use std::fs;

const CITIES: usize = 36;

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
        print!("     ");
        for (col_num, _) in self.rows.iter().enumerate() {
            print!("{} ", col_num);
            if (col_num < 10) {
                print!(" ");
            }
        }
        println!();
        for _ in 1..113 {
            print!("-")
        }
        println!();
        for (row_num, r) in self.rows.iter().enumerate() {
            print!("{}: ", row_num);
            if (row_num < 10) {
                print!(" ");
            }
            print!("[");
            for (i, c) in r.iter().enumerate() {
                print!("{}", c);
                if i < CITIES - 1 {
                    print!("  ");
                }
            }
            println!("]");
        }
    }
}

pub fn init_matrix() -> Matrix {
    return Matrix {
        rows: [[0; CITIES]; CITIES],
    };
}

pub fn route_file_to_adjacency_matrix(fpath: &str) -> Matrix {
    let mut point_matrix: Matrix = init_matrix();
    let mut color_matrix: Matrix = init_matrix();
    let route_file_as_string = fs::read_to_string(fpath).expect("Unable to read file");
    let data: HashMap<String, L1> = serde_json::from_str(&route_file_as_string).unwrap();
    let mut city_index_map: HashMap<String, usize> = HashMap::new();
    let mut i: usize = 0;
    for city in &valid_cities() {
        city_index_map.insert(city.clone(), i);
        i += 1;
    }
    for (starting_city, destination_cities) in &data {
        for (destination_city, route_data) in &destination_cities.destination_city {
            let mut conn: Vec<String> = Vec::new();
            for c in &route_data.connections {
                conn.push(c.color.clone());
            }
            println!(
                "{} to {}, {}",
                starting_city, destination_city, route_data.distance
            );
            println!(
                "{} to {}",
                *city_index_map.get(starting_city).unwrap_or(&(0 as usize)),
                *city_index_map
                    .get(destination_city)
                    .unwrap_or(&(0 as usize))
            );
            point_matrix.set(
                *city_index_map.get(starting_city).unwrap_or(&(0 as usize)),
                *city_index_map
                    .get(destination_city)
                    .unwrap_or(&(0 as usize)),
                route_data.distance,
            );
            color_matrix.set(
                *city_index_map.get(starting_city).unwrap_or(&(0 as usize)),
                *city_index_map
                    .get(destination_city)
                    .unwrap_or(&(0 as usize)),
                0,
            );
        }
    }
    return point_matrix;
}

pub fn demo() {
    let matrix = route_file_to_adjacency_matrix("mattgawarecki-ticket-to-ride/usa.routes.json");
    matrix.print();
}
