pub struct Camera {
    pub position: [f32; 2],
    pub zoom: f32,
}

impl Camera {
    pub fn new(position: [f32; 2], zoom: f32) -> Self {
        Self {
            position,
            zoom
        }
    }
}