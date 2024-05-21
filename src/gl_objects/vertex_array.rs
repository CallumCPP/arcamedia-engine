use crate::gl;
use crate::gl_objects::vertex_buffer::VertexBuffer;
use web_sys::{WebGl2RenderingContext, WebGlVertexArrayObject};

pub struct VertexArray {
    gl_vao: WebGlVertexArrayObject,
}

impl VertexArray {
    pub fn new() -> VertexArray {
        let gl_vao = gl().create_vertex_array().unwrap();

        Self { gl_vao }
    }

    pub fn bind(&self) {
        gl().bind_vertex_array(Some(&self.gl_vao));
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

        gl().vertex_attrib_pointer_with_i32(
            location as u32,
            num_components,
            WebGl2RenderingContext::FLOAT,
            normalized,
            stride,
            offset,
        );
        gl().enable_vertex_attrib_array(location as u32);
    }
}

impl Clone for VertexArray {
    fn clone(&self) -> Self {
        Self {
            gl_vao: self.gl_vao.clone(),
        }
    }
}
