pub struct Transform {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub rotation: f32,
}

impl Transform {
    pub fn new(position: [f32; 2], size: [f32; 2], rotation: f32) -> Self {
        Self {
            position,
            size,
            rotation,
        }
    }

    pub fn overlaps(&self, other: Transform) -> bool {
        self.position[0] - self.size[0]/2.0 <= other.position[0] + other.size[0]/2.0 &&
        self.position[0] + self.size[0]/2.0 >= other.position[0] - other.size[0]/2.0 &&
        self.position[1] - self.size[1]/2.0 >= other.position[1] + other.size[1]/2.0 &&
        self.position[1] + self.size[1]/2.0 >= other.position[1] - other.size[1]/2.0
    }
}
