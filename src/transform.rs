pub struct Transform {
    pub position: [f32; 2],
    pub scale: [f32; 2],
    pub rotation: f32,
}

impl Transform {
    pub fn new(position: [f32; 2], scale: [f32; 2], rotation: f32) -> Self {
        Self {
            position,
            scale,
            rotation,
        }
    }
}
