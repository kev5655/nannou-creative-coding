extern crate nannou;
use nannou::prelude::*;
use std::time::Instant;

mod drawers;
use drawers::{line, rect};

fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .simple_window(view)
        .run();
}

struct Mouse {
    is_drag: bool,
}
struct Model {
    bg_color: String,
    grid_size: f32,
    mouse: Mouse,
    sand: Vec<Vec<bool>>,
    last_update: Instant,
}

fn model(_app: &App) -> Model {
    let win = _app.window_rect();
    let mouse = Mouse { is_drag: false };
    Model {
        bg_color: "honeydew".to_string(),
        grid_size: 10.0,
        mouse,
        sand: vec![vec![false; (win.w() * 2.0 / 10.0) as usize]; (win.h() * 2.0 / 10.0) as usize],
        last_update: Instant::now(),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    if _model.last_update.elapsed().as_secs() >= 1 {
        _model.last_update = Instant::now();
    }
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let draw = _app.draw();
    draw.background()
        .color(from_str(&_model.bg_color).unwrap_or(BLACK));

    let win = _app.window_rect();
    let mut curr_y = -win.h();
    while curr_y <= win.h() {
        line(&draw, curr_y, -win.h(), curr_y, win.h());
        curr_y += _model.grid_size;
    }
    let mut curr_x = -win.w();
    while curr_x <= win.w() {
        line(&draw, -win.w(), curr_x, win.w(), curr_x);
        curr_x += _model.grid_size;
    }

    for (y, row) in _model.sand.iter().enumerate() {
        for (x, &should_draw) in row.iter().enumerate() {
            if should_draw {
                // Calculate the position based on grid coordinates
                let pos_x = x as f32 * _model.grid_size - (win.w() / 2.0);
                let pos_y = y as f32 * _model.grid_size - (win.h() / 2.0);
                // Draw the rectangle
                rect(&draw, pos_x, pos_y, _model.grid_size, _model.grid_size);
            }
        }
    }

    draw.to_frame(_app, &frame).unwrap();
}

fn event(_app: &App, _model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(window_event),
            ..
        } => match window_event {
            WindowEvent::MousePressed(_button) => {
                _model.mouse.is_drag = true;
            }
            WindowEvent::MouseReleased(_button) => {
                _model.mouse.is_drag = false;
            }
            WindowEvent::MouseMoved(position) => {
                if _model.mouse.is_drag {
                    let win = _app.window_rect();
                    let grid_x = (position.x + win.w() / 2.0) / _model.grid_size;
                    let grid_y = (position.y + win.h() / 2.0) / _model.grid_size;
                    if grid_x >= 0.0
                        && grid_x < (win.w() * 2.0) / _model.grid_size as f32
                        && grid_y >= 0.0
                        && grid_y < (win.h() * 2.0) / _model.grid_size as f32
                    {
                        _model.sand[grid_y as usize][grid_x as usize] = true;
                    }
                }
            }
            _ => {}
        },
        _ => {}
    }
}
