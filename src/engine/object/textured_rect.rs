use crate::engine::mesh::static_mesh_t::StaticMeshT;
use crate::engine::mesh::Mesh;
use crate::engine::shader::Shader;
use crate::engine::texture::Texture;
use crate::engine::vec2f::Vec2f;
use crate::object::{Object, Transform};
use crate::shader_manager::sm;

pub struct TexturedRect<'a> {
    transform: Transform,
    mesh: StaticMeshT,
    shader: Shader,
    texture: &'a Texture,
    collides: bool,
    pub color: [f32; 4],
    pub tags: Vec<String>,
}

impl<'a> TexturedRect<'a> {
    pub async fn new(
        position: Vec2f,
        size: Vec2f,
        rotation: f64,
        color: [f32; 4],
        texture: &'a Texture,
        collides: bool,
    ) -> Self {
        let shader = sm()
            .get_shader("textured_vert.glsl", "textured_frag.glsl")
            .await
            .clone();

        let transform = Transform::new(position, size, rotation);

        #[rustfmt::skip]
        let mesh = StaticMeshT::new(vec![
            -0.5, -0.5,  0.0, 0.0,
             0.5, -0.5,  1.0, 0.0,
            -0.5,  0.5,  0.0, 1.0,
             0.5, -0.5,  1.0, 0.0,
             0.5,  0.5,  1.0, 1.0,
            -0.5,  0.5,  0.0, 1.0,
        ]);

        Self {
            transform,
            mesh,
            shader,
            color,
            texture,
            collides,
            tags: Vec::new(),
        }
    }
}

impl<'a> Object for TexturedRect<'a> {
    fn draw(&self) {
        self.texture.bind();
        self.shader
            .uniform4fv_with_f32_array("fragColor", &self.color);
        self.shader.uniform1i("image", 0);
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
