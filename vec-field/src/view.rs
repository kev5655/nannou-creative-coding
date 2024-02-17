use nannou::{App, Frame};
use crate::drawers::arrow;
use crate::models::Model;

pub fn view(_app: &App, _model: &Model, _frame: Frame){

    let draw = _app.draw();
    let win = _app.window_rect();
    draw.background().color(_model.ui.bg_color);

    for (y, line) in _model.vectors.iter().enumerate() {
        for (x, element) in line.iter().enumerate() {
            let (c_x, c_y) = convert(x, y, _model.ui.grid_size);
            arrow(&draw, c_x, c_y, element.radiant, _model.ui.arrow_length, win)
        }
    }

    draw.to_frame(_app, &_frame).unwrap()

}

fn convert(x: usize, y: usize, size: u32) -> (u32, u32) {
    (convert_array_index_to_cord(x, size), convert_array_index_to_cord(y, size))
}
fn convert_array_index_to_cord(i: usize, size: u32) -> u32 {
    i as u32 * size + (size / 2)
}