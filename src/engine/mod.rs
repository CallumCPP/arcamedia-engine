use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;
use crate::engine::input::Input;
use crate::engine::object_manager::ObjectManager;
use crate::engine::shader_manager::ShaderManager;
use crate::engine::texture_manager::TextureManager;

#[macro_use]
pub mod web;
pub mod camera;
mod gl_objects;
pub mod input;
mod line_seg;
mod matrix;
mod mesh;
pub mod object;
pub mod object_manager;
mod raycast;
mod shader;
pub mod shader_manager;
mod texture;
pub mod texture_manager;
mod transform;
mod vec2;
pub mod timer;

static mut GL: Option<Box<WebGl2RenderingContext>> = None;
pub fn gl() -> &'static WebGl2RenderingContext {
    unsafe { GL.as_deref().expect("WebGL2 Context not initialized") }
}

pub struct Engine {}

impl Engine {
    pub fn init() {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement =
            canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        let gl = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();

        gl.enable(WebGl2RenderingContext::BLEND);

        gl.blend_func(
            WebGl2RenderingContext::SRC_ALPHA,
            WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
        );

        unsafe {
            GL = Some(Box::from(gl));
        }

        ShaderManager::init();
        TextureManager::init();
        ObjectManager::init();
        Input::init();
    }
}
