use crate::engine::input::Input;
use crate::engine::object_manager::ObjectManager;
use crate::engine::shader_manager::ShaderManager;
use crate::engine::text_renderer::TextRenderer;
use crate::engine::texture_manager::TextureManager;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

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
mod text_renderer;
mod texture;
pub mod texture_manager;
pub mod timer;
mod transform;
mod vec2f;
mod vec2i;

static mut GL: Option<Box<WebGl2RenderingContext>> = None;
pub fn gl() -> &'static WebGl2RenderingContext {
    unsafe { GL.as_deref().expect("WebGL2 Context not initialized") }
}

pub fn exit() {
    web_sys::window()
        .unwrap()
        .open_with_url_and_target("https://lncn.ac/rarcade", "_parent")
        .expect("Should be able to open arcade menu.");
}

pub struct Engine {}

impl Engine {
    pub async fn init() {
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
        TextRenderer::init().await;
    }
}
