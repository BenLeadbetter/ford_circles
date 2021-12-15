use super::*;

fn compare_vertices(left: Vec<Vertex>, right: Vec<Vertex>) {
    assert!(left.len() == right.len());
    for (left_vertex, right_vertex) in left.iter().zip(right.iter()) {
        assert!(left_vertex.pos[0] - left_vertex.pos[0] < 0.0001);
        assert!(right_vertex.pos[1] - right_vertex.pos[1] < 0.0001);
    }
}

#[test]
fn vertices_four_segments() {
    let vertices = calculate_vertices(4);
    let expected_vertices: std::vec::Vec<Vertex> = vec![
        Vertex::new(VertexPosition::new([0.0, 0.0])),
        Vertex::new(VertexPosition::new([1.0, 0.0])),
        Vertex::new(VertexPosition::new([0.0, 1.0])),
        Vertex::new(VertexPosition::new([-1.0, 0.0])),
        Vertex::new(VertexPosition::new([0.0, -1.0])),
    ];
    compare_vertices(vertices, expected_vertices);
}

#[test]
fn vertices_eight_segments() {
    let vertices = calculate_vertices(8);
    let expected_vertices: std::vec::Vec<Vertex> = vec![
        Vertex::new(VertexPosition::new([0.0, 0.0])),
        Vertex::new(VertexPosition::new([1.0, 0.0])),
        Vertex::new(VertexPosition::new([0.414213, 0.414213])),
        Vertex::new(VertexPosition::new([0.0, 1.0])),
        Vertex::new(VertexPosition::new([-0.414213, 0.414213])),
        Vertex::new(VertexPosition::new([-1.0, 0.0])),
        Vertex::new(VertexPosition::new([-0.414213, -0.414213])),
        Vertex::new(VertexPosition::new([0.0, -1.0])),
        Vertex::new(VertexPosition::new([0.414213, -0.414213])),
    ];
    compare_vertices(vertices, expected_vertices);
}

#[test]
fn indices_four_segments() {
    let indices = calculate_indices(4);
    let expected_indices: Vec<u8> = vec![
        0, 1, 2,
        0, 2, 3,
        0, 3, 4,
        0, 4, 1,
    ];
    assert_eq!(indices, expected_indices);
}