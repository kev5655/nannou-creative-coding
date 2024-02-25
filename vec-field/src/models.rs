use std::time::Instant;
use nannou::color::Srgb;
use nannou::geom::{Point2, Vec2};

pub struct Model {
    pub ui: UI,
    pub vectors: Vec<Vec<Vector>>,
    pub objects: Vec<Object>
}

pub struct UI {
    pub bg_color: Srgb<u8>,
    pub win_size: u32,
    pub grid_size: u32,
    pub grid_elements: u32,
    pub arrow_length: f32,
    pub last_updated: Instant,
}


pub struct Vector {
    pub vector: Vec2,
    pub radiant: f32
}


pub struct Object {
    pub position: Point2,
    pub velocity: f32,
    pub vector: Vector
}