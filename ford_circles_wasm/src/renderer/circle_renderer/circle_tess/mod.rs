use luminance::{
    Semantics, 
    Vertex
};

#[cfg(test)]
mod tests;

pub struct CircleTessData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u8>
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum VertexSemantics {
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
    pos: VertexPosition,
}

impl CircleTessData {
    pub fn new(segments: u8) -> CircleTessData {
        CircleTessData {
            vertices: calculate_vertices(segments),
            indices: calculate_indices(segments,)
        }
    }
}

pub fn calculate_vertices(segments: u8) -> Vec<Vertex> {
    let mut ret = Vec::new();
    ret.push(Vertex::new(VertexPosition::new([0.0, 0.0])));

    let angle_delta = 2.0 * std::f32::consts::PI / segments as f32;
    for i in 0..segments {
        ret.push(Vertex::new(VertexPosition::new([
            (angle_delta * i as f32).cos(), 
            (angle_delta * i as f32).sin()
        ])));
    }

    ret
}

pub fn calculate_indices(segments: u8) -> Vec<u8> {
    let mut ret = Vec::new();
    for i in 0..(segments - 1) {
        ret.push(0);
        ret.push(i + 1);
        ret.push(i + 2);
    }

    // last triangle
    ret.push(0);
    ret.push(segments);
    ret.push(1);

    ret
}