use std::time::Instant;

use nannou::{event::Update, App};
use nannou::rand::{Rng, thread_rng};

use crate::models::Model;

pub fn update(_app: &App, _model: &mut Model, _update: Update) {
    if _model.last_update.elapsed().as_millis() >= 200 {
        let max_x_index = _model.grid.len();
        let max_y_index = _model.grid[0].len();
        animate_sand(&mut _model.sand_blocks, max_x_index, max_y_index, &mut _model.grid);
        _model.last_update = Instant::now();
    }
}

enum MoveDirection {
    Down,
    LeftDown,
    RightDown,
    None
}

fn animate_sand(sand_blocks: &mut Vec<(usize, usize)>, max_x_index: usize, max_y_index: usize, grid: &mut Vec<Vec<bool>>) {
    let mut new_positions: Vec<(usize, usize)> = Vec::new();
    println!("animate_sand");
    for (x, y) in sand_blocks.iter() {
        match check_direction(x, y, max_x_index, max_y_index, grid) {
            MoveDirection::Down => {
                grid[*y][*x] = false;
                grid[y + 1][*x] = true;
                println!("Add Down Element on[{}, {}]",y + 1, x - 1);
                new_positions.push((*x, y + 1));
            },
            MoveDirection::LeftDown => {
                grid[*y][*x] = false;
                grid[y + 1][x - 1] = true;
                println!("Add Left Element on[{}, {}]",y + 1, x - 1);
                new_positions.push((x - 1, y + 1));
            },
            MoveDirection::RightDown => {
                grid[*y][*x] = false;
                grid[y + 1][x + 1] = true;
                println!("Add Right Element on[{}, {}]",y + 1, x + 1);
                new_positions.push((x + 1, y + 1));
            },
            MoveDirection::None => {
                println!("do not change Element on[{}, {}]",y + 1, x - 1);
                new_positions.push((*x, *y)); // Keep the block in its current position
            },
        }
    }

    *sand_blocks = new_positions;
}

fn check_direction(x: &usize, y: &usize, x_max: usize, y_max: usize, grid: &Vec<Vec<bool>>) -> MoveDirection {
    if can_move_down(x, y, y_max, grid) {
        return MoveDirection::Down;
    }
    let mut rng = thread_rng();
    // let try_left_first = rng.gen_bool(0.5); // 50% chance to try left down first

    if true {
        if can_move_left_down(x, y, x_max, y_max, grid) {
            return MoveDirection::LeftDown;
        } else if can_move_right_down(x, y, x_max, y_max, grid) {
            return MoveDirection::RightDown;
        }
    } else {
        if can_move_right_down(x, y, x_max, y_max, grid) {
            return MoveDirection::RightDown;
        } else if can_move_left_down(x, y, x_max, y_max, grid) {
            return MoveDirection::LeftDown;
        }
    }

    MoveDirection::None
}

fn can_move_down(x: &usize, y: &usize, y_max: usize, grid: &Vec<Vec<bool>>) -> bool {
    y + 1 < y_max && !grid[y + 1][*x]
}

fn can_move_left_down(x: &usize, y: &usize, x_max: usize, y_max: usize, grid: &Vec<Vec<bool>>) -> bool {
    return if is_between(x - 1, 0, x_max) && y + 1 < y_max {
        !grid[y + 1][x - 1]
    } else { false }
}

fn can_move_right_down(x: &usize, y: &usize, x_max: usize, y_max: usize, grid: &Vec<Vec<bool>>) -> bool {
    return if is_between(x + 1, 0, x_max) && y + 1 < y_max {
        !grid[y + 1][x + 1]
    }else { false }

}

fn is_between(value: usize, min: usize, max: usize) -> bool{
    value >= min && value < max
}

