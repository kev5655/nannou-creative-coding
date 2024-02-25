use nannou::{App, Draw, Frame};
use nannou::geom::{Rect, Vec2};
use crate::drawers::{arrow, circle};
use crate::models::Model;

pub fn view(_app: &App, _model: &Model, _frame: Frame){

    let draw = _app.draw();
    let win = _app.window_rect();
    draw.background().color(_model.ui.bg_color);

    draw_vec_field(_model, &draw, win);
    draw_objects(_model, &draw);

    draw.to_frame(_app, &_frame).unwrap()

}

fn draw_objects(_model: &Model, draw: &Draw) {
    for object in _model.objects.iter() {
        circle(&draw, object.position, 10.0)
    }
}

fn draw_vec_field(_model: &Model, draw: &Draw, win: Rect) {
    for (y, line) in _model.vectors.iter().enumerate() {
        for (x, element) in line.iter().enumerate() {
            let (c_x, c_y) = convert(x, y, _model.ui.grid_size);
            arrow(&draw, c_x, c_y, element.radiant, _model.ui.arrow_length, win)
        }
    }
}

fn convert(x: usize, y: usize, size: Vec2) -> (u32, u32) {
    (convert_array_index_to_cord(x, size.x as u32), convert_array_index_to_cord(y, size.y as u32))
}
fn convert_array_index_to_cord(i: usize, size: u32) -> u32 {
    i as u32 * size + (size / 2)
}