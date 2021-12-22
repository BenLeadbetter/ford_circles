use super::*;

fn compare_vertices(left: &Vec<Vertex>, right: &Vec<Vertex>) {
    assert!(left.len() == right.len());
    for (left_vertex, right_vertex) in left.iter().zip(right.iter()) {
        assert!((left_vertex.pos[0] - right_vertex.pos[0]).abs() < 0.0001);
        assert!((left_vertex.pos[1] - right_vertex.pos[1]).abs() < 0.0001);
    }
}

#[test]
fn vertices_four_segments() {
    let vertices = calculate_vertices(4);
    let expected_vertices: std::vec::Vec<Vertex> = vec![
        Vertex::new(VertexPosition::new([0.0, 0.0])),
        Vertex::new(VertexPosition::new([1.41421, 0.0])),
        Vertex::new(VertexPosition::new([0.0, 1.41421])),
        Vertex::new(VertexPosition::new([-1.41421, 0.0])),
        Vertex::new(VertexPosition::new([0.0, -1.41421])),
    ];
    compare_vertices(&vertices, &expected_vertices);
}

#[test]
fn vertices_eight_segments() {
    let vertices = calculate_vertices(8);
    let expected_vertices: std::vec::Vec<Vertex> = vec![
        Vertex::new(VertexPosition::new([0.0, 0.0])),
        Vertex::new(VertexPosition::new([1.082392, 0.0])),
        Vertex::new(VertexPosition::new([0.765366, 0.765366])),
        Vertex::new(VertexPosition::new([0.0, 1.082392])),
        Vertex::new(VertexPosition::new([-0.765366, 0.765366])),
        Vertex::new(VertexPosition::new([-1.082392, 0.0])),
        Vertex::new(VertexPosition::new([-0.765366, -0.765366])),
        Vertex::new(VertexPosition::new([0.0, -1.082392])),
        Vertex::new(VertexPosition::new([0.765366, -0.765366])),
    ];
    compare_vertices(&vertices, &expected_vertices);
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