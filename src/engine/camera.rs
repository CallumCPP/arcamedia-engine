use crate::engine::vec2f::Vec2f;

pub struct Camera {
    pub position: Vec2f,
    pub zoom: f64,
}

impl Camera {
    pub fn new(position: Vec2f, zoom: f64) -> Self {
        Self { position, zoom }
    }

    pub fn tick(&mut self) {
        // self.position = om().objects[0].borrow().transform().position.clone();
    }
}
