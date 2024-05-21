use crate::gl;
use crate::gl_objects::vertex_array::VertexArray;
use crate::gl_objects::vertex_buffer::VertexBuffer;
use crate::mesh::Mesh;
use web_sys::WebGl2RenderingContext;

pub struct StaticMesh {
    vert_count: i32,
    va: VertexArray,
}

impl Mesh for StaticMesh {
    fn new(vertices: Vec<f32>) -> Self {
        let va = VertexArray::new();
        let vb = VertexBuffer::new(&vertices);

        va.attach_vertex_buffer(&vb, 0, 2, false, 0, 0);

        let vert_count = (vertices.len() / 2) as i32;

        Self { vert_count, va }
    }

    fn draw(&self) {
        self.va.bind();
        gl().draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, self.vert_count);
    }
}

impl Clone for StaticMesh {
    fn clone(&self) -> Self {
        Self {
            vert_count: self.vert_count,
            va: self.va.clone(),
        }
    }
}
