mod models;
mod updater;
mod view;
mod drawers;
mod events;
mod vector_field;

use std::ops::Div;
use std::time::Instant;
use nannou::App;
use nannou::color::GRAY;
use nannou::geom::Vec2;
use nannou::glam::vec2;
use models::Model;
use crate::models::UI;
use crate::vector_field::for_vec_field;

fn main() {
    nannou::app(model).update(updater::updater).run();
}


fn model(app: &App) -> Model {
    let win_size = vec2(1200.0, 1200.0);
    let grid_elements: u32 = 50;
    let arrow_length = 10.;
    let grid_size: Vec2 = win_size / grid_elements as f32;

    println!("window_size: {} grid_size: {} grid_elements: {}", win_size, grid_size, grid_elements);

    app.new_window()
        .size(win_size.x as u32, win_size.y as u32)
        .view(view::view)
        .mouse_pressed(events::pressed)
        .build()
        .unwrap();

    let vectors = for_vec_field(grid_elements);

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




