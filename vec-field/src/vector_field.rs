use std::f32::consts::PI;
use nannou::geom::vec2;
use nannou::rand::random_range;
use crate::models::Vector;

pub fn for_vec_field(grid_elements: u32) -> Vec<Vec<Vector>> {
    let step = PI * 2.0 / grid_elements.pow(2) as f32;
    let mut curr = 0.0;

    let vectors: Vec<Vec<Vector>> = (0..grid_elements).map(|_| {
        (0..grid_elements).map(|_| {
            let v = Vector {
                direction: vec2(random_range(-1.0, 1.0), random_range(-1.0, 1.0)),
                radiant: curr,
            };
            curr += step;
            v
        }).collect()
    }).collect();
    vectors
}