use crate::input::input;
use crate::engine::mesh::static_mesh_t::StaticMeshT;
use crate::engine::mesh::Mesh;
use crate::object::{Object, Transform};
use crate::engine::shader::Shader;
use crate::shader_manager::sm;
use crate::engine::texture::Texture;
use crate::engine::vec2::Vec2;

pub struct TexturedRect<'a> {
    transform: Transform,
    mesh: StaticMeshT,
    shader: Shader,
    pub color: [f32; 4],
    texture: &'a Texture,
    collides: bool,
    pub tags: Vec<String>,
}

impl<'a> TexturedRect<'a> {
    pub async fn new(
        position: Vec2,
        size: Vec2,
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

    fn tick(&mut self, _delta_time: f64) {
        if input().key_was_pressed("KeyT") {
            self.transform.rotation += 0.2;
        }

        if input().key_was_pressed("KeyY") {
            self.transform.rotation -= 0.2;
        }
    }

    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    fn shader(&self) -> &Shader {
        &self.shader
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
