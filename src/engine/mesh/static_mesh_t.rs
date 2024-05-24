use crate::gl;
use crate::engine::gl_objects::vertex_array::VertexArray;
use crate::engine::gl_objects::vertex_buffer::VertexBuffer;
use crate::engine::mesh::Mesh;
use web_sys::WebGl2RenderingContext;

pub struct StaticMeshT {
    vert_count: i32,
    va: VertexArray,
}

impl Mesh for StaticMeshT {
    fn new(vertices: Vec<f32>) -> Self {
        let va = VertexArray::new();
        let vb = VertexBuffer::new(&vertices);

        va.attach_vertex_buffer(&vb, 0, 2, false, 16, 0);
        va.attach_vertex_buffer(&vb, 1, 2, false, 16, 8);

        let vert_count = (vertices.len() / 4) as i32;

        Self { vert_count, va }
    }

    fn draw(&self) {
        self.va.bind();
        gl().draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, self.vert_count);
    }
}
