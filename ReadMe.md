# Ticket to Ride - Graph Analysis

Analyzing the board game Ticket to Ride as an undirected graph.

Run via `cargo run`.

## Data

Data is included in the mattgawarecki-ticket-to-ride folder. This data was pulled 
from https://data.world/mattgawarecki/ticket-to-ride under the terms of the MIT 
license.

## Current Functionality

Determines the most efficient tickets by creating a ratio of the number of trains required to complete the ticket to the number of points earned from completing the ticket. A lower number is better, as less trains are required to earn a point. For example:

* Vancouver to Montreal requires 20 trains to earn 20 points from the ticket, and the distance to points ratio is 1, as 1 train earns 1 point.
* Seattle to New York requires 20 trains to earn 22 points, and the distance to points ratio is .909, as 20 trains are used to earn 22 points
* Winnipeg to Houston requires 15 trains to earn 12 points, and the distance to points ratio is 1.25, as 15 trains are used to earn 12 points

While some inconsistencies exist, most tickets have a 1:1 ratio. It may be advantageous to exploit some of these differences, but the difference in points may also be due to factors not currently accounted for in this analysis, such as length of routes used to complete the ticket, routes with more than one path, and the likelihood that a ticket will be closed off, ex. single point of failure vs. tickets with multiple workarounds.

This is the default implementation of the library, and can be run via

```sh
cargo run
```

## Individual library demos

View how individual modules work. Args can be combined.

```sh
./target/debug/ticket-to-ride --route
./target/debug/ticket-to-ride --ticket
./target/debug/ticket-to-ride --matrix
./target/debug/ticket-to-ride --graph
```

## Future Work

### Most Efficient Routes

* Consider length of routes required for a ticket and not just number of trains required to build
* Consider additional points from building trains in addition to completing the ticket
* Consider the effect of routes that can be completed with more than 1 color

### Easiest Routes

Find the routes with the best point value per difficulty.

* Calculate difficulty of all routes on a ratio of 1-10
  * This is a more difficult problem because will need to consider colored vs. 
    grey routes as well as routes with double tracks
  * May also want to consider number of viable paths
* Find ratio of difficulty per points, not considering paths to routes or 
points gained from building route

### Rust refactoring

Many complexities of rust memory management have been worked around via the `.clone()` function. While this may be acceptable, and even preferable, in some of its uses in this project, there are likely opportunities to reduce its usasge and write more performant and idiomatic Rust code while still maintaining functionality.

## Acknowledgements

* Contributors to the following StackOverflow questions:
  * [Parsing a nested JSON object](https://stackoverflow.com/questions/72289549/parsing-a-nested-json-in-rust-with-serde-json)
  * [Reading an array of strings from a file in Rust](https://stackoverflow.com/questions/72416538/reading-an-array-of-strings-from-a-file-in-rust/72416571#72416571)
