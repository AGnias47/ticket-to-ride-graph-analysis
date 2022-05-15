use crate::city::City;

pub struct Route {
    source: City,
    destination: City,
    distance: u8,
    connections: Vec<String>
}
