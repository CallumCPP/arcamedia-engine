use crate::mesh::dynamic_mesh_t::DynamicMeshT;
use crate::mesh::Mesh;
use crate::object::{Object, Transform};
use crate::shader::Shader;
use crate::shader_manager::sm;
use crate::texture::Texture;

pub struct TexturedRect<'a> {
    transform: Transform,
    mesh: DynamicMeshT,
    shader: Shader,
    color: [f32; 4],
    texture: &'a Texture,
}

impl<'a> TexturedRect<'a> {
    pub async fn new(
        position: [f32; 2],
        size: [f32; 2],
        rotation: f32,
        color: [f32; 4],
        texture: &'a Texture,
    ) -> Self {
        let shader = sm()
            .get_shader("textured_vert.glsl", "textured_frag.glsl")
            .await
            .clone();

        let transform = Transform::new(position, size, rotation);

        #[rustfmt::skip]
        let mesh = DynamicMeshT::new(vec![
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
        }
    }
}

impl<'a> Object for TexturedRect<'a> {
    fn draw(&mut self) {
        self.shader.bind();
        self.texture.bind();
        self.shader
            .uniform4fv_with_f32_array("fragColor", self.color);
        self.shader.uniform1i("image", 0);
        self.shader.uniform_transform(&self.transform);

        self.mesh.draw();
    }

    fn tick(&mut self, delta_time: f64) {}

    fn transform(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn shader(&self) -> &Shader {
        &self.shader
    }
}
