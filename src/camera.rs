use crate::input::input;
use crate::object_manager::om;
use crate::vec2::Vec2;

pub struct Camera {
    pub position: Vec2,
    pub zoom: f64,
}

impl Camera {
    pub fn new(position: Vec2, zoom: f64) -> Self {
        Self { position, zoom }
    }

    pub fn tick(&mut self) {
        self.position = om().objects[0].borrow().transform().position.clone();

        if input().key_was_pressed("KeyO") {
            self.zoom -= 0.02;
        }

        if input().key_was_pressed("KeyP") {
            self.zoom -= 0.2;
        }

        if input().key_was_pressed("KeyI") {
            self.zoom += 0.2;
        }
    }
}
