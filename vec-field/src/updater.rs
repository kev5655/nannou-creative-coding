use std::time::Instant;
use nannou::App;
use nannou::event::Update;
use nannou::geom::Vec2;
use nannou::geom::Rect;
use crate::models::{Model, Object, Vector};

pub fn updater(_app: &App, _model: &mut Model, _update: Update) {
    if _model.ui.last_updated.elapsed().as_millis() >= 10 {
        let updated_objects: Vec<Object> = _model.objects.iter().map(|object| {
            let new_pos =
                wrap_position(object.position + (object.velocity * object.vector.vector),
                              _app.window_rect());
            println!("old pos: {:?}, new pos: {:?}", object.position, new_pos);
            Object {
                position: new_pos, // Directly use new_pos, no need for additional calculation
                velocity: object.velocity,
                vector: Vector {
                    vector: object.vector.vector,
                    radiant: object.vector.radiant,
                },
            }
        }).collect();

        _model.objects = updated_objects;
        _model.ui.last_updated = Instant::now();
    }
}

fn wrap_position(pos: Vec2, win: Rect<f32>) -> Vec2 {
    let mut new_pos = pos;

    // Handle X coordinate
    if pos.x < win.left() {
        new_pos.x = win.right();
    } else if pos.x > win.right() {
        new_pos.x = win.left();
    }

    // Handle Y coordinate
    if pos.y < win.bottom() {
        new_pos.y = win.top();
    } else if pos.y > win.top() {
        new_pos.y = win.bottom();
    }

    new_pos
}