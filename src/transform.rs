use crate::vec2::Vec2;
use js_sys::Math::{max, min};

pub struct Transform {
    pub position: Vec2,
    pub size: Vec2,
    pub rotation: f64,
}

impl Transform {
    pub fn new(position: Vec2, size: Vec2, rotation: f64) -> Self {
        Self {
            position,
            size,
            rotation,
        }
    }

    pub fn overlaps(&self, other: &Transform) -> bool {
        let vertices1 = self.vertices();
        let vertices2 = other.vertices();

        let mut normals: Vec<Vec2> = self.normals(&vertices1);
        normals.extend(other.normals(&vertices2));

        for normal in normals {
            let min_max1 = Self::get_min_max_projection(&vertices1, &normal);
            let min_max2 = Self::get_min_max_projection(&vertices2, &normal);

            if min_max1.y < min_max2.x || min_max2.y < min_max1.x {
                return false;
            }
        }

        true
    }

    pub fn overlaps_lazy(&self, other: &Transform) -> bool {
        let half_size = &self.size / 2.0;
        let other_half_size = &other.size / 2.0;

        let corner = &Vec2::new(half_size.x, half_size.y).rotated(self.rotation);

        let half_size = corner.len();

        self.position.x - half_size < other.position.x + other_half_size.x
            && self.position.y - half_size < other.position.y + other_half_size.y
            && self.position.x + half_size > other.position.x - other_half_size.x
            && self.position.y + half_size > other.position.y - other_half_size.y
    }

    pub fn vertices(&self) -> Vec<Vec2> {
        let mut vertices: Vec<Vec2> = Vec::new();
        let half_size = &self.size / 2.0;

        let point = &Vec2::new(half_size.x, half_size.y).rotated(self.rotation) + &self.position; // Top right
        vertices.push(point);

        let point = &Vec2::new(half_size.x, -half_size.y).rotated(self.rotation) + &self.position; // Bottom right
        vertices.push(point);

        let point = &Vec2::new(-half_size.x, -half_size.y).rotated(self.rotation) + &self.position; // Bottom left
        vertices.push(point);

        let point = &Vec2::new(-half_size.x, half_size.y).rotated(self.rotation) + &self.position; // Top left
        vertices.push(point);

        vertices
    }

    pub fn normals(&self, vertices: &[Vec2]) -> Vec<Vec2> {
        let mut normals: Vec<Vec2> = Vec::new();

        for i in 0..vertices.len() {
            let p1 = &vertices[i];
            let p2 = &vertices[(i + 1) % vertices.len()];

            let edge = Vec2::new(p2.x - p1.x, p2.y - p1.y);
            let normal = Vec2::new(edge.y, -edge.x).normalize();
            normals.push(normal);
        }

        normals
    }

    pub fn nearest_edge_to(&self, point: &Vec2) -> (Vec2, Vec2) {
        let mut nearest_edge: (Vec2, Vec2) = ([0.0, 0.0].into(), [0.0, 0.0].into());
        let mut nearest_distance = f64::MAX;

        for i in 0..self.vertices().len() {
            let vertex1 = &self.vertices()[i];
            let vertex2 = &self.vertices()[(i + 1) % self.vertices().len()];
            let distance = Self::point_segment_distance(point, vertex1, vertex2);

            if distance < nearest_distance {
                nearest_edge = (vertex1.clone(), vertex2.clone());
                nearest_distance = distance;
            }
        }

        nearest_edge
    }

    fn point_segment_distance(point: &Vec2, seg_point1: &Vec2, seg_point2: &Vec2) -> f64 {
        let a = seg_point1;
        let b = seg_point2;
        let p = point;

        let ab = b - a;
        let ap = p - a;

        let t = Vec2::dot(&ap, &ab) / Vec2::dot(&ab, &ab);
        let t = max(0.0, min(1.0, t));

        let c = a + &(&ab * t);

        (p - &c).len()
    }

    fn get_min_max_projection(vertices: &Vec<Vec2>, normal: &Vec2) -> Vec2 {
        let mut min = f64::MAX;
        let mut max = f64::MIN;

        for vertex in vertices {
            let projection = vertex.dot(normal);

            if projection < min {
                min = projection;
            }
            if projection > max {
                max = projection
            }
        }

        Vec2::new(min, max)
    }
}

impl Clone for Transform {
    fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            size: self.size.clone(),
            rotation: self.rotation,
        }
    }
}
