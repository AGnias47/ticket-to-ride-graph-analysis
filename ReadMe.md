# Ticket to Ride - Graph Analysis

Analyzing the board game Ticket to Ride as an undirected graph for various problems. This 
is currently a work in progress.

## Data

Data is included in the mattgawarecki-ticket-to-ride folder. This data was pulled 
from https://data.world/mattgawarecki/ticket-to-ride under the terms of the MIT 
license.

## Most Efficient Routes

Find out which routes gain the most points for the least amount of trains

### Process

* Find route with best ratio of points per train. Calculate this using points 
per each route as well as points of finishing the route.

## Easiest Routes

Find the routes with the best point value per difficulty.

* Calculate difficulty of all routes on a ratio of 1-10
  * This is a more difficult problem because will need to consider colored vs. 
    grey routes as well as routes with double tracks
  * May also want to consider number of viable paths
* Find ratio of difficulty per points, not considering paths to routes or 
points gained from building route

## Acknowledgements

* Contributors to [this StackOverflow question](https://stackoverflow.com/questions/72289549/parsing-a-nested-json-in-rust-with-serde-json)
