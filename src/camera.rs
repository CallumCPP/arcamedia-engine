use crate::input::input;

pub struct Camera {
    pub position: [f32; 2],
    pub zoom: f32,
}

impl Camera {
    pub fn new(position: [f32; 2], zoom: f32) -> Self {
        Self { position, zoom }
    }

    pub fn tick(&mut self, delta_time: f64) {
        let speed = (1000.0 * delta_time) as f32;

        if input().get_key_down("KeyW") {
            self.position[1] += speed;
        }

        if input().get_key_down("KeyS") {
            self.position[1] -= speed;
        }

        if input().get_key_down("KeyA") {
            self.position[0] -= speed;
        }

        if input().get_key_down("KeyD") {
            self.position[0] += speed;
        }
    }
}
