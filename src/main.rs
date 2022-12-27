mod graph;
mod matrix;
mod route;
mod serializers;
mod ticket;
mod validate;

use clap::Parser;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(long, short, action)]
    route: bool,

    #[clap(long, short, action)]
    ticket: bool,

    #[clap(long, short, action)]
    matrix: bool,

    #[clap(long, short, action)]
    graph: bool,
}

fn main() {
    let tickets: Vec<ticket::Ticket> =
        ticket::ticket_file_to_vec("mattgawarecki-ticket-to-ride/usa.tickets.json");
    let routes: HashMap<String, HashMap<String, route::Route>> =
        route::route_file_to_hashmap("mattgawarecki-ticket-to-ride/usa.routes.json");
    let args = Args::parse();
    if args.route {
        route::demo();
    }
    if args.ticket {
        ticket::demo();
    }
    if args.matrix {
        matrix::demo();
    }
    if args.graph {
        graph::demo();
    }
    if !args.route && !args.ticket && !args.matrix && !args.graph {
        route::demo();
        ticket::demo();
        matrix::demo();
        graph::demo();
    }
}
