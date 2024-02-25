mod models;
mod updater;
mod view;
mod drawers;
mod events;

use std::f32::consts::PI;
use std::time::Instant;
use nannou::App;
use nannou::color::GRAY;
use nannou::glam::vec2;
use nannou::rand::random_range;
use models::Model;
use updater::updater;
use crate::events::pressed;
use crate::models::{UI, Vector};
use crate::view::view;

fn main() {
    nannou::app(model).update(updater).run();
}


fn model(app: &App) -> Model {
    let win_size = 1200;
    let grid_elements = 50;
    let arrow_length = 10.;
    let grid_size = win_size / grid_elements;

    println!("window_size: {} grid_size: {} grid_elements: {}", win_size, grid_size, grid_elements);

    app.new_window()
        .size(win_size, win_size)
        .view(view)
        .mouse_pressed(pressed)
        .build()
        .unwrap();

    let step = PI * 2.0 / grid_elements.pow(2) as f32;
    let mut curr = 0.0;

    let vectors: Vec<Vec<Vector>> = (0..grid_elements).map(|_| {
        (0..grid_elements).map(|_| {
            let v = Vector {
                vector: vec2(random_range(-1.0, 1.0), random_range(-1.0, 1.0)),
                radiant: curr,
            };
            curr += step;
            v
        }).collect()
    }).collect();

    println!("vector {}", vectors.len());

    Model {
        ui: UI {
            bg_color: GRAY,
            win_size,
            grid_size,
            grid_elements,
            arrow_length,
            last_updated: Instant::now()
        },
        vectors,
        objects: Vec::new()
    }
}


