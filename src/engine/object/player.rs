use crate::engine::line_seg::LineSeg;
use crate::engine::raycast::{FilterType, Raycast};
use crate::engine::shader::Shader;
use crate::engine::texture::Texture;
use crate::engine::vec2f::Vec2f;
use crate::input::input;
use crate::object;
use crate::object::rect::Rect;
use crate::object::textured_rect::TexturedRect;
use crate::object::{Object, Transform};
use crate::object_manager::om;
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
    pub tags: Vec<String>,
}

impl<'a> Player<'a> {
    pub async fn new(
        position: Vec2f,
        size: Vec2f,
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
            tags: ["player".into()].into(),
        }
    }
}

impl Object for Player<'_> {
    fn draw(&self) {
        self.textured_rect.shader().unwrap().bind();
        self.textured_rect.draw();

        self.raycast_rect.shader().unwrap().bind();
        self.raycast_rect.draw();
    }

    fn tick(&mut self, delta_time: f64) {
        let mut key_dir = Vec2f::new(0.0, 0.0);

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

        let speed = self.speed * delta_time;

        let delta_position = &key_dir.normalize() * speed;

        let old_position = self.transform().unwrap().position.clone();
        self.transform_mut().unwrap().position += &delta_position;

        for object in &om().objects_on_screen[1..] {
            let object = object.borrow();
            let transform = object.transform().unwrap();

            if object.collides() && transform.overlaps(self.transform().unwrap()) {
                self.transform_mut().unwrap().position = old_position;
                break;
            }
        }

        if input().get_key_down("KeyM") {
            self.raycast_angle -= 2.0 * delta_time;
        }

        if input().get_key_down("KeyN") {
            self.raycast_angle += 2.0 * delta_time;
        }

        let raycast_length = 500.0;
        let mut raycast_p2 = Vec2f::new(
            raycast_length * cos(self.raycast_angle),
            raycast_length * sin(self.raycast_angle),
        );

        self.raycast_rect.transform_mut().unwrap().position =
            &self.transform().unwrap().position + &(&raycast_p2 / 2.0);
        self.raycast_rect.transform_mut().unwrap().rotation = self.raycast_angle;
        self.raycast_rect.transform_mut().unwrap().size.x = raycast_length;

        raycast_p2 += &self.transform().unwrap().position.clone();

        if input().get_key_down("KeyF") {
            let mut rect = self.dummy_rect.clone();
            rect.transform_mut().unwrap().position = raycast_p2.clone();
            rect.color[0] = random() as f32;
            rect.color[1] = random() as f32;
            rect.color[2] = random() as f32;

            object!(rect);
        }

        if input().get_key_down("KeyR") {
            let ray = LineSeg::new(
                self.transform().unwrap().position.clone(),
                raycast_p2.clone(),
            );

            let mut raycast = Raycast::new(ray, ["player".into()].into());
            raycast.fire(FilterType::Blacklist);

            match raycast.hit {
                None => {}
                Some(hit) => {
                    hit.object.borrow_mut().transform_mut().unwrap().rotation += 8.0 * delta_time;
                }
            }
        }

        if input().get_key_down("KeyG") {
            let ray = LineSeg::new(
                self.transform().unwrap().position.clone(),
                raycast_p2.clone(),
            );

            let mut raycast = Raycast::new(ray, ["player".into()].into());
            raycast.fire(FilterType::Blacklist);

            match raycast.hit {
                None => {}
                Some(hit) => {
                    om().remove_object(hit.object.clone());
                }
            }
        }
    }

    fn transform(&self) -> Option<&Transform> {
        self.textured_rect.transform()
    }

    fn transform_mut(&mut self) -> Option<&mut Transform> {
        self.textured_rect.transform_mut()
    }

    fn set_transform(&mut self, transform: Transform) {
        self.textured_rect.set_transform(transform);
    }

    fn shader(&self) -> Option<&Shader> {
        self.textured_rect.shader()
    }

    fn color_mut(&mut self) -> Option<&mut [f32; 4]> {
        self.textured_rect.color_mut()
    }

    fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    fn tags_mut(&mut self) -> &mut Vec<String> {
        &mut self.tags
    }
}
