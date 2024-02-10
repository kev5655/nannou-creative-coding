use std::time::Instant;

pub struct Mouse {
    pub is_drag: bool,
}

pub struct Model {
    pub bg_color: String,
    pub grid_size: u32,
    pub mouse: Mouse,
    pub grid: Vec<Vec<bool>>,
    pub sand_blocks: Vec<(usize, usize)>,
    pub last_update: Instant,

}
