use std::time::Instant;
use nannou::color::Srgb;
use nannou::geom::{Point2, Rect, Vec2};

pub struct Model {
    pub ui: UI,
    pub vectors: Vec<Vec<Vector>>,
    pub objects: Vec<Object>
}

pub struct UI {
    pub bg_color: Srgb<u8>,
    pub win_size: Vec2,
    pub grid_size: Vec2,
    pub grid_elements: u32,
    pub arrow_length: f32,
    pub last_updated: Instant,
}

#[derive(Clone)]
pub struct Vector {
    pub direction: Vec2,
    pub radiant: f32
}


pub struct Object {
    pub position: Point2,
    pub velocity: f32,
    pub vector: Vector
}

impl Object {
    pub fn update_position(&mut self, direction: Vec2, window_rect: Rect<f32>) {
        let new_pos = self.position + (self.velocity * direction);

        // Correctly call the associated function without passing `self`
        self.position = Object::wrap_position(new_pos, window_rect);
    }

    fn wrap_position(pos: Vec2, win: Rect<f32>) -> Vec2 {
        Vec2::new(
            if pos.x < win.left() { win.right() } else if pos.x > win.right() { win.left() } else { pos.x },
            if pos.y < win.bottom() { win.top() } else if pos.y > win.top() { win.bottom() } else { pos.y },
        )
    }
}