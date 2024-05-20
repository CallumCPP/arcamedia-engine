use crate::input::input;
use crate::object::textured_rect::TexturedRect;
use crate::object::{Object, Transform};
use crate::object_manager::om;
use crate::shader::Shader;
use crate::texture::Texture;
use crate::vec2::Vec2;

pub struct Player<'a> {
    speed: f64,
    textured_rect: TexturedRect<'a>,
}

impl<'a> Player<'a> {
    pub async fn new(
        position: Vec2,
        size: Vec2,
        rotation: f64,
        color: [f32; 4],
        texture: &'a Texture,
    ) -> Self {
        let textured_rect = TexturedRect::new(position, size, rotation, color, texture).await;

        Self {
            speed: 1000.0,
            textured_rect,
        }
    }
}

impl<'a> Object for Player<'a> {
    fn draw(&self) {
        self.textured_rect.draw();
    }

    fn tick(&mut self, delta_time: f64) {
        let mut delta_position = Vec2::new(0.0, 0.0);

        if input().get_key_down("KeyW") {
            delta_position.y += 1.0;
        }
        if input().get_key_down("KeyS") {
            delta_position.y -= 1.0;
        }
        if input().get_key_down("KeyA") {
            delta_position.x -= 1.0;
        }
        if input().get_key_down("KeyD") {
            delta_position.x += 1.0;
        }

        delta_position = &delta_position.normalize() * (self.speed * delta_time);

        let old_position = self.transform().position.clone();
        self.transform_mut().position += &delta_position;

        for object in &om().objects_on_screen[1..] {
            if object.borrow().transform().overlaps(self.transform()) {
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
