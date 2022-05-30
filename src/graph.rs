enum GraphColor {
    White,
    Grey,
    Black
}

struct Graph {
    adj_matrix: Matrix,
    vertices: Vec<Vertex>,
    edges: Vec<Edge>
}

struct Edge {
    origin: Vertex,
    destination: Vertex,
    weight: u8
}

struct Vertex {
    city: String
}
