use crate::engine::input::input;
use crate::engine::object_manager::om;
use crate::engine::vec2::Vec2;

pub struct Camera {
    pub position: Vec2,
    pub zoom: f64,
}

impl Camera {
    pub fn new(position: Vec2, zoom: f64) -> Self {
        Self { position, zoom }
    }

    pub fn tick(&mut self) {
        // self.position = om().objects[0].borrow().transform().position.clone();
    }
}
