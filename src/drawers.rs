use nannou::prelude::*;

pub fn rect(
    draw: &Draw, x: f32, y: f32, w: f32, h: f32, color: Rgb<u8>,
) {
    draw.rect().x_y(x, y).width(w).height(h).color(color);
}

