use crate::engine::texture::Texture;
use crate::gl;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use web_sys::WebGl2RenderingContext;

static mut TM: Option<Box<TextureManager>> = None;

pub fn tm() -> &'static mut TextureManager {
    unsafe {
        TM.as_deref_mut()
            .expect("Texture manager should be initialized")
    }
}

pub struct TextureManager {
    textures: HashMap<String, Texture>,
}

impl TextureManager {
    pub fn init() {
        let textures: HashMap<String, Texture> = HashMap::new();
        gl().pixel_storei(WebGl2RenderingContext::UNPACK_PREMULTIPLY_ALPHA_WEBGL, 0);

        unsafe { TM = Some(Box::new(Self { textures })) }
    }

    pub async fn get_texture(&mut self, path: &str) -> &Texture {
        match self.textures.entry(path.into()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let texture = Texture::new(path).await;
                entry.insert(texture)
            }
        }
    }
}
