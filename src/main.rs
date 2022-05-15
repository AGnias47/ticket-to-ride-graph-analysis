mod city;
mod route;
mod ticket;

fn main() {
    route::routes_from_file("mattgawarecki-ticket-to-ride/usa.routes.json");
    println!("Hello, world!");
}

