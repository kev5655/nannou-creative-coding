use nannou::color::Srgb;
use nannou::geom::Vec2;

pub struct Model {
    pub ui: UI,
    pub vectors: Vec<Vec<Vector>>
}

pub struct UI {
    pub bg_color: Srgb<u8>,
    pub win_size: u32,
    pub grid_size: u32,
    pub grid_elements: u32,
    pub arrow_length: f32,
}


pub struct Vector {
    pub vector: Vec2,
    pub radiant: f32
}