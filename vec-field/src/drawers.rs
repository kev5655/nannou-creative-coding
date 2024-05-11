use nannou::prelude::*;
use nannou::Draw;
use nannou::geom::{Point2, pt2};

pub fn arrow(draw: &Draw, x: u32, y: u32, radiant: f32, scalar: f32, window: Rect) {
    let start_p = convert_p(
        rotate(pt2(-scalar, 0.0), radiant)
            + pt2(x as f32, y as f32)
        , window);

    let end_p = convert_p(
        rotate(pt2(scalar, 0.0), radiant)
            + pt2(x as f32, y as f32)
        , window);

    draw.arrow().start(start_p).end(end_p).stroke_weight(1.0).color(WHITE);
    // draw.line().start(start_p).end(end_p).stroke_weight(2.0).color(WHITE);
}

pub fn circle(draw: &Draw, point2: Point2, diameter: f32){
    draw.ellipse().xy(point2).radius(diameter);
}

pub fn convert_p(point2: Point2, window: Rect) -> Point2 {
    let (c_x, c_y) = convert(point2.x, point2.y, window);
    pt2(c_x, c_y)
}

fn convert(x: f32, y: f32, window: Rect) -> (f32, f32) {
    let c_x: f32 = x - (window.w() / 2.0);
    let c_y: f32 = (window.h() / 2.0) - y;
    (c_x, c_y)
}
fn rotate(p: Point2, radiant: f32) -> Point2 {
    let x = p.x * radiant.cos() - p.y * radiant.sin();
    let y = p.x * radiant.sin() + p.y * radiant.cos();
    pt2(x, y)
}

// fn start_p(x: f32, y: f32, vec: Vec2) -> Point2 {
//     pt2(x - (vec.x / 2.0), y + (vec.y / 2.0))
// }
//
// fn end_p(x: f32, y: f32, vec: Vec2) -> Point2 {
//     pt2(x + (vec.x / 2.0), y - (vec.y / 2.0))
// }

