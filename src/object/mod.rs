use crate::gl;
use crate::gl_objects::vertex_array::VertexArray;
use crate::gl_objects::vertex_buffer::VertexBuffer;
use crate::shader::Shader;
use web_sys::WebGl2RenderingContext;

pub mod rect;

pub trait Object {
    fn draw(&mut self);
    fn tick(&mut self);
    fn mesh(&mut self) -> &mut Mesh;
    fn transform(&mut self) -> &mut Transform;
    fn shader(&self) -> &Shader;
}

pub struct Transform {
    pub position: [f32; 2],
    pub scale: [f32; 2],
    pub rotation: f32,
}

pub struct Mesh {
    vertices: Vec<f32>,
    vert_count: i32,
    vao: VertexArray,
    vbo: VertexBuffer,
}

impl Transform {
    pub fn new(position: [f32; 2], scale: [f32; 2], rotation: f32) -> Self {
        Self {
            position,
            scale,
            rotation,
        }
    }
}

impl Mesh {
    pub fn new(vertices: Vec<f32>) -> Self {
        let vao = VertexArray::new();
        let vbo = VertexBuffer::new(&vertices);

        vao.attach_vertex_buffer(&vbo, 0, 2, false, 0, 0);

        let vert_count = (vertices.len() / 2) as i32;

        Self {
            vertices,
            vert_count,
            vao,
            vbo,
        }
    }

    pub fn draw(&self) {
        self.vao.bind();
        self.vbo.update(self.vertices.as_slice());
        gl().draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, self.vert_count);
    }
}
