use std::time::Instant;
use nannou::App;
use nannou::event::Update;
use nannou::geom::{Point2, Vec2};
use crate::models::{Model, Vector};

pub fn updater(_app: &App, _model: &mut Model, _update: Update) {
    if _model.ui.last_updated.elapsed().as_millis() >= 10 {
        _model.objects.iter_mut().for_each(|object| {
            let vec = find_next_vec(&_model.vectors, _model.ui.grid_size, object.position);

            object.update_position(vec, _app.window_rect());
        });
        _model.ui.last_updated = Instant::now();
    }
}

fn find_next_vec(vectors: &Vec<Vec<Vector>>, grid_size: Vec2, curr_pos: Point2) -> Vec2 {
    let (i, j) = ((curr_pos.x / grid_size.x) as usize, (curr_pos.y / grid_size.y) as usize);
    let vector = mish_vectors(get_vec_around(i, j, vectors));
    vector.direction
}

fn get_vec_around(i: usize, j: usize, vec: &Vec<Vec<Vector>>) -> Vec<Vector> {
    let mut around = Vec::new();
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), /* (0, 0), */ (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    for (di, dj) in directions.iter() {
        let new_i = i as isize + di;
        let new_j = j as isize + dj;

        // Check if the new indices are within the bounds of the vector
        if new_i >= 0 && new_j >= 0 && (new_i as usize) < vec.len() && (new_j as usize) < vec[new_i as usize].len() {
            around.push(vec[new_i as usize][new_j as usize].clone());
        }
    }

    around
}

fn mish_vectors(vectors: Vec<Vector>) -> Vector {
    let n = vectors.len() as f32; // Convert to f32 for division
    if n == 0.0 {
        return Vector {
            direction: Vec2::ZERO, // Use Vec2::ZERO for a vector with all components as 0
            radiant: 0.0,
        }; // Return a default Vector if vectors is empty
    }

    // Sum up all vectors' direction and radiant components
    let sum = vectors.iter().fold(Vector { direction: Vec2::ZERO, radiant: 0.0 }, |acc, vec| Vector {
        direction: acc.direction + vec.direction,
        radiant: acc.radiant + vec.radiant,
    });

    // Calculate average
    Vector {
        direction: sum.direction / n,
        radiant: sum.radiant / n,
    }
}
