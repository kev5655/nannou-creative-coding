extern crate nannou;
use nannou::prelude::*;
use std::time::Instant;

mod drawers;
use drawers::rect;

mod convert;
use convert::convert_grid_to_00win;

mod models;
use models::{Model, Mouse};

mod events;
use events::{moved, pressed, released};

mod updater;
use updater::update;

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    // Create the window first
    let window_size: u32 = 800.0 as u32; // This matches the size specified in the window creation
    let grid_size: u32 = 50.0 as u32;
    app.new_window()
        .size(window_size, window_size)
        .view(view)
        .mouse_pressed(pressed)
        .mouse_released(released)
        .mouse_moved(moved)
        .build()
        .unwrap();

    let mouse = Mouse { is_drag: false };
    let sand_size = (window_size / grid_size) as usize;

    Model {
        bg_color: "black".to_string(),
        grid_size,
        mouse,
        sand: vec![vec![false; sand_size]; sand_size],
        dirty_cells: Vec::new(),
        last_update: Instant::now(),
    }
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let draw = _app.draw();
    draw.background()
        .color(from_str(&_model.bg_color).unwrap_or(BLACK));

    let win = _app.window_rect();

    for &(x, y) in &_model.dirty_cells {
        let size = _model.grid_size as f32;
        let (pos_x, pos_y) =
            convert_grid_to_00win(x, y, win.w(), win.h(), size);
        rect(&draw, pos_x, pos_y, size - 1.0, size - 1.0, STEELBLUE);
    }

    draw.to_frame(_app, &frame).unwrap();
}
