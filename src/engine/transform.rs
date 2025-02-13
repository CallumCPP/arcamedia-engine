use crate::engine::line_seg::LineSeg;
use crate::engine::vec2f::Vec2f;
use js_sys::Math::{max, min};

pub struct Transform {
    pub position: Vec2f,
    pub size: Vec2f,
    pub rotation: f64,
}

impl Transform {
    pub fn new(position: Vec2f, size: Vec2f, rotation: f64) -> Self {
        Self {
            position,
            size,
            rotation,
        }
    }

    pub fn overlaps(&self, other: &Transform) -> bool {
        let vertices1 = self.vertices();
        let vertices2 = other.vertices();

        let mut normals: Vec<Vec2f> = self.normals(&vertices1);
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
        let self_half_size = &self.size.abs() / 2.0;
        let other_half_size = &other.size.abs() / 2.0;

        let rotated_corners = [
            Vec2f::new(self_half_size.x, self_half_size.y).rotated(self.rotation),
            Vec2f::new(self_half_size.x, -self_half_size.y).rotated(self.rotation),
            Vec2f::new(-self_half_size.x, self_half_size.y).rotated(self.rotation),
            Vec2f::new(-self_half_size.x, -self_half_size.y).rotated(self.rotation),
        ];

        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for corner in &rotated_corners {
            let corner_world = &self.position + corner;
            if corner_world.x < min_x {
                min_x = corner_world.x;
            }
            if corner_world.x > max_x {
                max_x = corner_world.x;
            }
            if corner_world.y < min_y {
                min_y = corner_world.y;
            }
            if corner_world.y > max_y {
                max_y = corner_world.y;
            }
        }

        min_x < other.position.x + other_half_size.x
            && max_x > other.position.x - other_half_size.x
            && min_y < other.position.y + other_half_size.y
            && max_y > other.position.y - other_half_size.y
    }

    pub fn vertices(&self) -> Vec<Vec2f> {
        let mut vertices: Vec<Vec2f> = Vec::new();
        let half_size = &self.size / 2.0;

        let point = &Vec2f::new(half_size.x, half_size.y).rotated(self.rotation) + &self.position; // Top right
        vertices.push(point);

        let point = &Vec2f::new(half_size.x, -half_size.y).rotated(self.rotation) + &self.position; // Bottom right
        vertices.push(point);

        let point = &Vec2f::new(-half_size.x, -half_size.y).rotated(self.rotation) + &self.position; // Bottom left
        vertices.push(point);

        let point = &Vec2f::new(-half_size.x, half_size.y).rotated(self.rotation) + &self.position; // Top left
        vertices.push(point);

        vertices
    }

    pub fn lines(&self) -> Vec<LineSeg> {
        let mut lines: Vec<LineSeg> = Vec::new();

        let vertices = self.vertices();
        for i in 0..vertices.len() {
            lines.push(LineSeg::new(
                vertices[i].clone(),
                vertices[(i + 1) % vertices.len()].clone(),
            ));
        }

        lines
    }

    pub fn normals(&self, vertices: &[Vec2f]) -> Vec<Vec2f> {
        let mut normals: Vec<Vec2f> = Vec::new();

        for i in 0..vertices.len() {
            let p1 = &vertices[i];
            let p2 = &vertices[(i + 1) % vertices.len()];

            let edge = Vec2f::new(p2.x - p1.x, p2.y - p1.y);
            let normal = Vec2f::new(edge.y, -edge.x).normalize();
            normals.push(normal);
        }

        normals
    }

    pub fn nearest_edge_to(&self, point: &Vec2f) -> (Vec2f, Vec2f) {
        let mut nearest_edge: (Vec2f, Vec2f) = ([0.0, 0.0].into(), [0.0, 0.0].into());
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

    fn point_segment_distance(point: &Vec2f, seg_point1: &Vec2f, seg_point2: &Vec2f) -> f64 {
        let a = seg_point1;
        let b = seg_point2;
        let p = point;

        let ab = b - a;
        let ap = p - a;

        let t = Vec2f::dot(&ap, &ab) / Vec2f::dot(&ab, &ab);
        let t = max(0.0, min(1.0, t));

        let c = a + &(&ab * t);

        (p - &c).len()
    }

    fn get_min_max_projection(vertices: &Vec<Vec2f>, normal: &Vec2f) -> Vec2f {
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

        Vec2f::new(min, max)
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
