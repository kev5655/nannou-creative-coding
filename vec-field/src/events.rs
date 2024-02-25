use nannou::App;
use nannou::event::MouseButton;
use nannou::geom::vec2;
use nannou::rand::random_range;
use crate::models::{Model, Object, Vector};

pub fn pressed(_app: &App, _model: &mut Model, _button: MouseButton) {

    let position = _app.mouse.position();
     _model.objects.push(Object {
         position,
         velocity: 1.0,
         vector: Vector {
             vector: vec2(random_range(-1.0, 1.0), random_range(-1.0, 1.0)),
             radiant: Default::default(),
         }
     })

}