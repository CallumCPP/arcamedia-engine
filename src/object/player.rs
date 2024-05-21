use crate::input::input;
use crate::object;
use crate::object::rect::Rect;
use crate::object::textured_rect::TexturedRect;
use crate::object::{Object, Transform};
use crate::object_manager::om;
use crate::shader::Shader;
use crate::texture::Texture;
use crate::vec2::Vec2;
use js_sys::Math::random;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Player<'a> {
    speed: f64,
    textured_rect: TexturedRect<'a>,
    dummy_rect: Rect,
}

impl<'a> Player<'a> {
    pub async fn new(
        position: Vec2,
        size: Vec2,
        rotation: f64,
        color: [f32; 4],
        texture: &'a Texture,
    ) -> Self {
        let textured_rect =
            TexturedRect::new(position, size, rotation, color, texture, false).await;

        let dummy_rect = Rect::new(
            [0.0, 0.0].into(),
            [100.0, 100.0].into(),
            0.0,
            [1.0, 1.0, 1.0, 1.0],
            false,
        )
        .await;

        Self {
            speed: 1000.0,
            textured_rect,
            dummy_rect,
        }
    }
}

impl<'a> Object for Player<'a> {
    fn draw(&self) {
        self.textured_rect.draw();
    }

    fn tick(&mut self, delta_time: f64) {
        let mut key_dir = Vec2::new(0.0, 0.0);

        if input().get_key_down("KeyW") {
            key_dir.y += 1.0;
        }
        if input().get_key_down("KeyS") {
            key_dir.y -= 1.0;
        }
        if input().get_key_down("KeyA") {
            key_dir.x -= 1.0;
        }
        if input().get_key_down("KeyD") {
            key_dir.x += 1.0;
        }

        if input().get_key_down("KeyF") {
            let mut rect = self.dummy_rect.clone();
            rect.transform_mut().position = self.transform().position.clone();
            rect.color[0] = random() as f32;
            rect.color[1] = random() as f32;
            rect.color[2] = random() as f32;

            object!(rect);
        }

        let speed = self.speed * delta_time;

        let delta_position = &key_dir.normalize() * speed;

        let old_position = self.transform().position.clone();
        self.transform_mut().position += &delta_position;

        for object in &om().objects_on_screen[1..] {
            let object = object.borrow();
            let transform = object.transform();

            if object.collides() && transform.overlaps(self.transform()) {
                self.transform_mut().position = old_position;
                return;
            }
        }
    }

    fn transform(&self) -> &Transform {
        self.textured_rect.transform()
    }

    fn transform_mut(&mut self) -> &mut Transform {
        self.textured_rect.transform_mut()
    }

    fn shader(&self) -> &Shader {
        self.textured_rect.shader()
    }
}
