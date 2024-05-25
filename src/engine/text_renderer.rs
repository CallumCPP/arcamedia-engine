use crate::engine::vec2f::Vec2f;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/web.js")]
extern "C" {
    fn makeTextCanvas(text: String, width: f64, height: f64) -> web_sys::CanvasRenderingContext2d;
    fn changeTextFillStyle(style: String);
}

static mut TR: Option<Box<TextRenderer>> = None;

pub fn tr() -> &'static mut TextRenderer {
    unsafe {
        TR.as_deref_mut()
            .expect("TextRenderer should be initialized")
    }
}

pub struct TextRenderer {
    pub text_canvas: web_sys::CanvasRenderingContext2d,
}

impl TextRenderer {
    pub async fn init() {
        let text_canvas = makeTextCanvas("".into(), 1920.0, 1080.0);

        unsafe { TR = Some(Box::new(Self { text_canvas })) }
    }

    pub fn clear(&self) {
        self.text_canvas.clear_rect(0.0, 0.0, 1920.0, 1080.0);
    }

    pub fn draw_text(&self, text: &str, pos: Vec2f) {
        self.text_canvas
            .fill_text(text, pos.x, pos.y)
            .expect("Should be able to draw text.");
    }
    
    pub fn set_text_color(&self, color: String) {
        changeTextFillStyle(color);
    }
}
