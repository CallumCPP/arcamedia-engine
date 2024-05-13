use crate::gl_objects::vertex_array::VertexArray;
use crate::gl_objects::vertex_buffer::VertexBuffer;
use crate::shader::Shader;
use web_sys::WebGl2RenderingContext;

pub struct Object<'gl, 'shader, 'vertices> {
    gl: &'gl WebGl2RenderingContext,
    vertices: &'vertices Vec<f32>,
    color: [f32; 4],
    vao: VertexArray<'gl>,
    vbo: VertexBuffer<'gl>,
    shader: &'shader Shader<'shader>,
    vert_count: i32,
}

impl<'gl, 'shader, 'vertices> Object<'gl, 'shader, 'vertices> {
    pub fn new(
        gl: &'gl WebGl2RenderingContext,
        vertices: &'vertices Vec<f32>,
        shader: &'shader Shader,
        color: [f32; 4],
    ) -> Object<'gl, 'shader, 'vertices> {
        let vbo = VertexBuffer::new(gl, vertices);
        let vao = VertexArray::new(gl);
        vao.attach_vertex_buffer(&vbo, 0, 2, false, 0, 0);

        let vert_count = (vertices.len() / 2) as i32;

        Self {
            gl,
            vertices,
            color,
            vao,
            vbo,
            shader,
            vert_count,
        }
    }

    pub fn draw(&self) {
        self.shader.bind();
        self.vao.bind();
        self.vbo.update(self.vertices);
        self.shader.uniform4fv_with_f32_array("fragColor", self.color);
        self.gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, self.vert_count);
    }
}
