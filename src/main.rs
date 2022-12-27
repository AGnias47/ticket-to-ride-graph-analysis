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
    let bfs_graph: graph::Graph = graph::Graph::new();
    for t in tickets {
        let (predecessor, distance) = bfs_graph.bfs(graph::Vertex { city: t.clone().source });
        let d: u8 = graph::distance_from_bfs_origin(
            graph::Vertex {
                city: t.clone().destination,
            },
            predecessor,
            distance,
        );
        println!("{}", t.to_string());
        println!("Distance: {}", d);
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
