use crate::route::Route;
use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Hash)]
pub struct City {
    name: String,
    routes: Vec<Route>
}