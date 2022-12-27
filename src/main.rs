mod graph;
mod matrix;
mod route;
mod serializers;
mod ticket;
mod validate;

use clap::Parser;

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

fn most_efficient_routes() {
    let tickets: Vec<ticket::Ticket> =
        ticket::ticket_file_to_vec("mattgawarecki-ticket-to-ride/usa.tickets.json");
    let graph: graph::Graph = graph::Graph::new();
    for t in tickets {
        let ssp = graph.dijkstra_ssp(graph::Vertex { city: t.clone().source });
        let d: u8 = *ssp.get(&t.clone().destination).unwrap();
        println!("{}", t.to_string());
        println!("Distance: {}", d);
        let dtpr: f32 = d as f32 / t.clone().points as f32;
        println!("Distance to Points Ratio: {}", dtpr);
        println!("-----------------");
    }
}

fn main() {
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
        most_efficient_routes();
    }
}
