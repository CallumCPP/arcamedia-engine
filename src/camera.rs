use crate::input::input;
use crate::vec2::Vec2;

pub struct Camera {
    pub position: Vec2,
    pub zoom: f64,
}

impl Camera {
    pub fn new(position: Vec2, zoom: f64) -> Self {
        Self { position, zoom }
    }

    pub fn tick(&mut self, delta_time: f64) {
        let speed = 1000.0 * delta_time / self.zoom;

        if input().get_key_down("KeyW") {
            self.position.y += speed;
        }

        if input().get_key_down("KeyS") {
            self.position.y -= speed;
        }

        if input().get_key_down("KeyA") {
            self.position.x -= speed;
        }

        if input().get_key_down("KeyD") {
            self.position.x += speed;
        }

        if input().key_was_pressed("KeyI") {
            self.zoom += 0.2;
        }

        if input().key_was_pressed("KeyO") {
            self.zoom -= 0.2;
        }
    }
}
