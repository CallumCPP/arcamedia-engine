use crate::engine::mesh::static_mesh::StaticMesh;
use crate::engine::mesh::Mesh;
use crate::engine::shader::Shader;
use crate::engine::vec2::Vec2;
use crate::object::{Object, Transform};
use crate::shader_manager::sm;

pub struct Rect {
    transform: Transform,
    shader: Shader,
    mesh: StaticMesh,
    collides: bool,
    pub color: [f32; 4],
    pub tags: Vec<String>,
}

impl Rect {
    pub async fn new(
        position: Vec2,
        size: Vec2,
        rotation: f64,
        color: [f32; 4],
        collides: bool,
    ) -> Self {
        let shader = sm()
            .get_shader("colored_vert.glsl", "colored_frag.glsl")
            .await
            .clone();

        let transform = Transform::new(position, size, rotation);

        let mesh = StaticMesh::new(vec![
            -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5,
        ]);

        Self {
            transform,
            mesh,
            shader,
            color,
            collides,
            tags: Vec::new(),
        }
    }
}

impl Object for Rect {
    fn draw(&self) {
        self.shader
            .uniform4fv_with_f32_array("fragColor", &self.color);
        self.shader.uniform_transform(&self.transform);

        self.mesh.draw();
    }

    fn transform(&self) -> Option<&Transform> {
        Some(&self.transform)
    }

    fn transform_mut(&mut self) -> Option<&mut Transform> {
        Some(&mut self.transform)
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    fn shader(&self) -> Option<&Shader> {
        Some(&self.shader)
    }

    fn collides(&self) -> bool {
        self.collides
    }

    fn color_mut(&mut self) -> Option<&mut [f32; 4]> {
        Some(&mut self.color)
    }

    fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    fn tags_mut(&mut self) -> &mut Vec<String> {
        &mut self.tags
    }
}

impl Clone for Rect {
    fn clone(&self) -> Self {
        Self {
            shader: self.shader.clone(),
            transform: self.transform().unwrap().clone(),
            mesh: self.mesh.clone(),
            color: self.color.clone(),
            collides: self.collides,
            tags: self.tags.clone(),
        }
    }
}
