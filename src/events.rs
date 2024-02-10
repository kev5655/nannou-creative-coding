use nannou::{event::MouseButton, geom::Point2, App};

use crate::models::Model;

pub fn pressed(_app: &App, _model: &mut Model, _button: MouseButton) {
    _model.mouse.is_drag = true;
}

pub fn released(
    _app: &App, _model: &mut Model, _button: MouseButton,
) {
    _model.mouse.is_drag = false;
}

pub fn moved(_app: &App, _model: &mut Model, p: Point2) {
    if _model.mouse.is_drag {
        let win = _app.window_rect();

        let r_y: f32 = win.h() / 2.0 - p.y;
        let r_x: f32 = win.w() / 2.0 + p.x;

        if r_y < win.h() && r_y >= 0.0 && r_x < win.w() && r_x >= 0.0
        {
            let size = _model.grid_size;
            let g_y: usize = (r_y / size as f32).floor() as usize;
            let g_x: usize = (r_x / size as f32).floor() as usize;
            if !_model.sand[g_y][g_x] {
                _model.sand[g_y][g_x] = true;
                _model.dirty_cells.push((g_x, g_y))
            }
        }
    }
}
