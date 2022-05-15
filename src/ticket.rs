use crate::city::City;

pub struct Ticket {
    route: (City, City),
    points: u8,
}