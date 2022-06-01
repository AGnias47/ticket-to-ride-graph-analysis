mod route;
mod ticket;
mod validate;
mod matrix;
mod serializers;
mod graph;

use std::collections::HashMap;

fn main() {
    let tickets: Vec<ticket::Ticket> =
        ticket::ticket_file_to_vec("mattgawarecki-ticket-to-ride/usa.tickets.json");
    let routes: HashMap<String, HashMap<String, route::Route>> =
        route::route_file_to_hashmap("mattgawarecki-ticket-to-ride/usa.routes.json");
    route::demo();
    ticket::demo();
    matrix::demo();
    graph::demo();
}
