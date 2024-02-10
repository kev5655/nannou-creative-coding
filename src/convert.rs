pub fn convert_grid_to_00win(x: usize, y: usize, win_x: f32, win_y: f32, size: f32) -> (f32, f32) {
    let pos_x = x as f32 * size - (win_x / 2.0) + (size / 2.0);
    let pos_y = (win_y / 2.0) - (size / 2.0) - y as f32 * size;
    (pos_x, pos_y)
}
