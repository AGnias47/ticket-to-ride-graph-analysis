use serde::Deserialize;
use std::fs;

#[derive(Clone)]
pub struct Ticket {
    pub source: String,
    pub destination: String,
    pub points: u8,
}

impl Ticket {
    pub fn to_string(&self) -> String {
        return format!(
            "Source: {}\nDestination: {}\nPoints: {}",
            &self.source, &self.destination, &self.points
        );
    }
}

pub fn ticket_file_to_vec(fpath: &str) -> Vec<Ticket> {
    #[derive(Debug, Deserialize)]
    pub struct TicketSerializer {
        cities: Vec<String>,
        points: u8,
    }

    let ticket_file_as_string = fs::read_to_string(fpath).expect("Unable to read file");
    let data: Vec<TicketSerializer> =
        serde_json::from_str(&ticket_file_as_string).expect("Nothing");
    let mut tickets: Vec<Ticket> = Vec::new();
    for d in data {
        let ticket = Ticket {
            source: d.cities[0].clone(),
            destination: d.cities[1].clone(),
            points: d.points,
        };
        tickets.push(ticket);
    }
    return tickets;
}

pub fn demo() {
    let tickets = ticket_file_to_vec("mattgawarecki-ticket-to-ride/usa.tickets.json");
    for t in tickets {
        println!("{}", t.to_string());
        println!("-----------------");
    }
}
