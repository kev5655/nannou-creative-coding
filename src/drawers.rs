use nannou::prelude::*;

// pub fn line(draw: &Draw, start_x: f32, start_y: f32, end_x: f32, end_y: f32) {
//     draw.line()
//         .start(pt2(start_x, start_y))
//         .end(pt2(end_x, end_y))
//         .weight(1.0)
//         .color(BLACK);
// }

pub fn rect(
    draw: &Draw, x: f32, y: f32, w: f32, h: f32, color: Rgb<u8>,
) {
    draw.rect().x_y(x, y).width(w).height(h).color(color);
}

// pub fn text(draw: &Draw, text: &str, position: Point2, font_size: u32, color: Rgb<u8>) {
//     draw.text(text)
//         .font_size(font_size)
//         .xy(position)
//         .color(color);
// }

// pub fn grid_init(
//     grid: &Vec<Vec<bool>>, grid_size: f32, draw: &Draw, win_x: f32,
//     win_y: f32,
// ) {
//     for (y, row) in grid.iter().enumerate() {
//         for (x, &_should_draw) in row.iter().enumerate() {
//             let (pos_x, pos_y) =
//                 convert_grid_to_00win(x, y, win_x, win_y, grid_size);

//             rect(
//                 draw,
//                 pos_x,
//                 pos_y,
//                 grid_size as f32 - 1.0,
//                 grid_size as f32 - 1.0,
//                 CADETBLUE,
//             );
//         }
//     }
// }
