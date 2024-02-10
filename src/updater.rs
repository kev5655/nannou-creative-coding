use std::time::Instant;

use nannou::{event::Update, App};

use crate::models::Model;

pub fn update(_app: &App, _model: &mut Model, _update: Update) {
    if _model.last_update.elapsed().as_secs() >= 1 {
        _model.last_update = Instant::now();
    }
}
