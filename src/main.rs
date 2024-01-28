extern crate nannou;
use nannou::{event, prelude::*};
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
    pos: Point2,
    is_drag: bool,
}
struct Model {
    bg_color: String,
    x: f32,
    y: f32,
    radius: f32,
    grid_size: f32,
    mouse: Mouse,
    sand: Vec<Point2>,
    last_update: Instant,
}

fn model(_app: &App) -> Model {
    let initial_mouse_pos = pt2(0.0, 0.0);
    let mouse = Mouse {
        pos: initial_mouse_pos,
        is_drag: false,
    };
    Model {
        bg_color: "honeydew".to_string(),
        x: 0.0,
        y: 0.0,
        radius: 10.0,
        grid_size: 10.0,
        mouse: mouse,
        sand: Vec::new(),
        last_update: Instant::now(),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    if _model.last_update.elapsed().as_secs() >= 1 {
        if _model.radius < 500.0 {
            _model.radius += 10.0;
        }
        _model.last_update = Instant::now();
    }
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let draw = _app.draw();
    draw.background()
        .color(from_str(&_model.bg_color).unwrap_or(BLACK));
    draw.ellipse()
        .color(STEELBLUE)
        .w(_model.radius)
        .h(_model.radius)
        .x_y(_model.x, _model.y);

    draw.rect().x_y(0.0, 100.0).w_h(30.0, 50.0).color(YELLOW);

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

    for pos in &_model.sand {
        rect(&draw, pos.x, pos.y, _model.grid_size, _model.grid_size);
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
                    // Update mouse position only if dragging
                    _model.mouse.pos = position.into();
                    _model.sand.push(position.into());
                }
            }
            _ => {}
        },
        _ => {}
    }
}
