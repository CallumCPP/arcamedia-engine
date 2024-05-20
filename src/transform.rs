use crate::vec2::Vec2;
use js_sys::Math::{cos, sin, sqrt};

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
            let min_max1 = Self::get_min_max(&vertices1, &normal);
            let min_max2 = Self::get_min_max(&vertices2, &normal);

            if min_max1.y < min_max2.x || min_max2.y < min_max1.x {
                return false;
            }
        }

        true
    }

    pub fn vertices(&self) -> Vec<Vec2> {
        let mut vertices: Vec<Vec2> = Vec::new();
        let half_size = &self.size / 2.0;

        let point =
            &rotate_point(&Vec2::new(half_size.x, half_size.y), self.rotation) + &self.position; // Top right
        vertices.push(point);

        let point =
            &rotate_point(&Vec2::new(half_size.x, -half_size.y), self.rotation) + &self.position; // Bottom right
        vertices.push(point);

        let point =
            &rotate_point(&Vec2::new(-half_size.x, -half_size.y), self.rotation) + &self.position; // Bottom left
        vertices.push(point);

        let point =
            &rotate_point(&Vec2::new(-half_size.x, half_size.y), self.rotation) + &self.position; // Top left
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

    fn get_min_max(vertices: &Vec<Vec2>, normal: &Vec2) -> Vec2 {
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

fn rotate_point(point: &Vec2, rotation: f64) -> Vec2 {
    Vec2::new(
        point.x * cos(rotation) - point.y * sin(rotation),
        point.y * cos(rotation) + point.x * sin(rotation),
    )
}
