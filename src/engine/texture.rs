use crate::engine::{gl, web};
use image::ImageFormat;
use web_sys::{WebGl2RenderingContext, WebGlTexture};

pub struct Texture {
    web_gl_texture: WebGlTexture,
}

impl Texture {
    pub async fn new(path: &str) -> Self {
        let img_file = web::get_bytes((String::from("/textures/") + &*String::from(path)).as_str())
            .await
            .expect("Should get texture.");
        let img = image::load_from_memory_with_format(img_file.as_slice(), ImageFormat::Png)
            .expect("Should be able to load image");

        let web_gl_texture = gl()
            .create_texture()
            .expect("Should be able to create texture");
        gl().bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&web_gl_texture));

        gl().tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            WebGl2RenderingContext::REPEAT as i32,
        );
        gl().tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            WebGl2RenderingContext::REPEAT as i32,
        );
        gl().tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::NEAREST as i32,
        );
        gl().tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::NEAREST as i32,
        );

        log!("{}", img.width());
        log!("{}", img.height());

        let (format, data_type) = match img.color() {
            image::ColorType::L8 => (
                WebGl2RenderingContext::LUMINANCE,
                WebGl2RenderingContext::UNSIGNED_BYTE,
            ),
            image::ColorType::La8 => (
                WebGl2RenderingContext::LUMINANCE_ALPHA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
            ),
            image::ColorType::Rgb8 => (
                WebGl2RenderingContext::RGB,
                WebGl2RenderingContext::UNSIGNED_BYTE,
            ),
            image::ColorType::Rgba8 => (
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
            ),
            _ => panic!("Unsupported image color type"),
        };

        gl().tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            format as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            format,
            data_type,
            Some(img.as_bytes()),
        )
        .expect("Should be able to upload image");

        Self { web_gl_texture }
    }

    pub fn bind(&self) {
        gl().active_texture(WebGl2RenderingContext::TEXTURE0);
        gl().bind_texture(
            WebGl2RenderingContext::TEXTURE_2D,
            Some(&self.web_gl_texture),
        );
    }
}
