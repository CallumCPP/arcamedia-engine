use crate::gl_objects::vertex_buffer::VertexBuffer;
use web_sys::{WebGl2RenderingContext, WebGlVertexArrayObject};

pub struct VertexArray<'a> {
    gl: &'a WebGl2RenderingContext,
    gl_vao: WebGlVertexArrayObject,
}

impl<'a> VertexArray<'a> {
    pub fn new(gl: &'a WebGl2RenderingContext) -> VertexArray<'a> {
        let gl_vao = gl.create_vertex_array().unwrap();

        Self { gl, gl_vao }
    }

    pub fn bind(&self) {
        self.gl.bind_vertex_array(Some(&self.gl_vao));
    }

    pub fn attach_vertex_buffer(
        &self,
        buffer: &VertexBuffer,
        location: i32,
        num_components: i32,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        self.bind();
        buffer.bind();

        self.gl.vertex_attrib_pointer_with_i32(
            location as u32,
            num_components,
            WebGl2RenderingContext::FLOAT,
            normalized,
            stride,
            offset,
        );
        self.gl.enable_vertex_attrib_array(location as u32);
    }
}
