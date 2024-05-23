use crate::input::input;
use crate::line_seg::LineSeg;
use crate::object;
use crate::object::rect::Rect;
use crate::object::textured_rect::TexturedRect;
use crate::object::{Object, Transform};
use crate::object_manager::om;
use crate::raycast::Raycast;
use crate::shader::Shader;
use crate::texture::Texture;
use crate::vec2::Vec2;
use js_sys::Math::{cos, random, sin};
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;

pub struct Player<'a> {
    speed: f64,
    textured_rect: TexturedRect<'a>,
    raycast_rect: Rect,
    raycast_angle: f64,
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
            TexturedRect::new(position.clone(), size, rotation, color, texture, false).await;

        let raycast_rect = Rect::new(
            position.clone(),
            [5.0, 5.0].into(),
            PI / 4.0,
            [0.7, 0.2, 0.4, 1.0],
            false,
        )
        .await;

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
            raycast_rect,
            raycast_angle: 0.0,
            dummy_rect,
        }
    }
}

impl<'a> Object for Player<'a> {
    fn draw(&self) {
        self.textured_rect.shader().bind();
        self.textured_rect.draw();

        self.raycast_rect.shader().bind();
        self.raycast_rect.draw();
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

        if input().get_key_down("KeyM") {
            self.raycast_angle -= 1.0 * delta_time;
        }

        if input().get_key_down("KeyN") {
            self.raycast_angle += 1.0 * delta_time;
        }

        let raycast_length = 500.0;
        self.raycast_rect.transform_mut().position = &self.transform().position
            + &Vec2::new(
                raycast_length * cos(self.raycast_angle),
                raycast_length * sin(self.raycast_angle),
            );

        if input().get_key_down("KeyR") {
            let ray = LineSeg::new(
                self.transform().position.clone(),
                self.raycast_rect.transform().position.clone(),
            );

            let raycast = Raycast::new(ray);

            match raycast.hit {
                None => {}
                Some(hit) => {
                    hit.object.borrow_mut().transform_mut().rotation += 8.0 * delta_time;
                }
            }
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

    fn set_transform(&mut self, transform: Transform) {
        self.textured_rect.set_transform(transform);
    }

    fn shader(&self) -> &Shader {
        self.textured_rect.shader()
    }

    fn color_mut(&mut self) -> Option<&mut [f32; 4]> {
        self.textured_rect.color_mut()
    }
}
