use crate::gl;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

pub struct VertexBuffer {
    buffer: WebGlBuffer,
}

impl VertexBuffer {
    pub fn new(vertices: &[f32]) -> VertexBuffer {
        let buffer = gl().create_buffer().unwrap();
        gl().bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        // Note that `Float32Array::view` is somewhat dangerous (hence the
        // `unsafe`!). This is creating a raw view into our module's
        // `WebAssembly.Memory` buffer, but if we allocate more pages for ourselves
        // (aka do a memory allocation in Rust) it'll cause the buffer to change,
        // causing the `Float32Array` to be invalid.
        //
        // As a result, after `Float32Array::view` we have to be very careful not to
        // do any memory allocations before it's dropped.
        unsafe {
            let vertices_view = js_sys::Float32Array::view(vertices);

            gl().buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vertices_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        Self { buffer }
    }

    pub fn update(&self, vertices: &[f32]) {
        self.bind();

        unsafe {
            let vertices_view = js_sys::Float32Array::view(vertices);

            gl().buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vertices_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
    }

    pub fn bind(&self) {
        gl().bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffer));
    }
}
