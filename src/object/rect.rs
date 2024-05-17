use crate::object::{Mesh, Object, Transform};
use crate::shader::Shader;
use crate::shader_manager::sm;

pub struct Rect {
    transform: Transform,
    mesh: Mesh,
    shader: Shader,
    color: [f32; 4],
}

impl Rect {
    pub fn new(position: [f32; 2], scale: [f32; 2], rotation: f32, color: [f32; 4]) -> Self {
        let shader =
            sm().get_shader("colored_vert.glsl", "colored_frag.glsl").clone();

        let transform = Transform::new(position, scale, rotation);

        let mesh = Mesh::new(vec![
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
    fn draw(&mut self) {
        self.shader.bind();
        self.shader
            .uniform4fv_with_f32_array("fragColor", self.color);
        self.shader.uniform_transform(&self.transform);

        self.mesh.draw();
    }

    fn tick(&mut self) {

    }

    fn mesh(&mut self) -> &mut Mesh {
        &mut self.mesh
    }

    fn transform(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn shader(&self) -> &Shader {
        &self.shader
    }
}
