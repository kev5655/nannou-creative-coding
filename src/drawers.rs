use nannou::prelude::*;

pub fn line(draw: &Draw, start_x: f32, start_y: f32, end_x: f32, end_y: f32) {
    draw.line()
        .start(pt2(start_x, start_y))
        .end(pt2(end_x, end_y))
        .weight(1.0)
        .color(BLACK);
}

pub fn rect(draw: &Draw, x: f32, y: f32, w: f32, h: f32) {
    draw.rect().x_y(x, y).width(w).height(h).color(STEELBLUE);
}
