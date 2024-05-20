use crate::mesh::static_mesh::StaticMesh;
use crate::mesh::Mesh;
use crate::object::{Object, Transform};
use crate::shader::Shader;
use crate::shader_manager::sm;
use crate::vec2::Vec2;

pub struct Rect {
    transform: Transform,
    mesh: StaticMesh,
    shader: Shader,
    color: [f32; 4],
}

impl Rect {
    pub async fn new(position: Vec2, size: Vec2, rotation: f64, color: [f32; 4]) -> Self {
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
        }
    }
}

impl Object for Rect {
    fn draw(&self) {
        self.shader.bind();
        self.shader
            .uniform4fv_with_f32_array("fragColor", &self.color);
        self.shader.uniform_transform(&self.transform);

        self.mesh.draw();
    }

    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn shader(&self) -> &Shader {
        &self.shader
    }
}
