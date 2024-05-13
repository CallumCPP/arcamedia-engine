use web_sys::{WebGl2RenderingContext, WebGlBuffer};

pub struct VertexBuffer<'a> {
    gl: &'a WebGl2RenderingContext,
    buffer: WebGlBuffer,
}

impl<'a> VertexBuffer<'a> {
    pub fn new(gl: &'a WebGl2RenderingContext, vertices: &[f32]) -> VertexBuffer<'a> {
        let buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

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

            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vertices_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        Self { gl, buffer }
    }

    pub fn update(&self, vertices: &[f32]) {
        self.bind();

        unsafe {
            let vertices_view = js_sys::Float32Array::view(vertices);

            self.gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vertices_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
    }

    pub fn bind(&self) {
        self.gl
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffer));
    }
}
