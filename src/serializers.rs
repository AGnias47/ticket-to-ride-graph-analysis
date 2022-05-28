extern crate serde;
extern crate serde_json;

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct L1 {
    #[serde(flatten)]
    pub destination_city: HashMap<String, L2>,
}

#[derive(Debug, Deserialize)]
pub struct L2 {
    pub distance: u8,
    pub connections: Vec<L3>,
}

#[derive(Debug, Deserialize)]
pub struct L3 {
    pub color: String,
    locomotives: u8,
    tunnels: u8,
}
