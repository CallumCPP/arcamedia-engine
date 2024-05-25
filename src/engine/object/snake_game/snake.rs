use super::*;
use crate::engine::input::input;
use crate::engine::object::snake_game::segment::Segment;
use crate::engine::object::snake_game::snake::Direction::*;
use crate::engine::object::textured_rect::TexturedRect;
use crate::engine::object::Object;
use crate::engine::shader::Shader;
use crate::engine::texture::Texture;
use crate::engine::timer::Timer;
use crate::engine::transform::Transform;
use crate::engine::vec2f::Vec2f;
use std::cmp::PartialEq;
use std::f64::consts::PI;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Still,
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake<'a> {
    pub speed: f64,
    start_speed: f64,
    pub head_rect: TexturedRect<'a>,
    movement_timer: Timer,
    movement_carry: f64,
    direction: Direction,
    movement_queue: Vec<Direction>,
    pub dead: bool,
    pub prev_head_position: Vec2f,
    dummy_segment: Segment,
    pub tail: Vec<Segment>,
    tags: Vec<String>,
    color: [f32; 4],
}

impl<'a> Snake<'a> {
    pub async fn new(speed: f64, position: Vec2f, texture: &'a Texture) -> Self {
        let head_rect = TexturedRect::new(
            position.clone(),
            [TILE_SIZE, TILE_SIZE].into(),
            0.0,
            [1.0, 1.0, 1.0, 1.0],
            texture,
            false,
        )
        .await;

        let movement_timer = Timer::new();

        let dummy_segment = Segment::new().await;

        let tail: Vec<Segment> = Vec::new();
        let tags: Vec<String> = Vec::new();
        let movement_queue: Vec<Direction> = Vec::new();

        Self {
            speed,
            start_speed: speed,
            head_rect,
            movement_timer,
            movement_carry: 0.0,
            direction: Still,
            movement_queue,
            dead: false,
            prev_head_position: [0.0, 0.0].into(),
            dummy_segment,
            tail,
            tags,
            color: [40.0 / 255.0, 220.0 / 255.0, 40.0 / 255.0, 1.0],
        }
    }

    pub fn add_segment(&mut self) {
        match self.tail.last() {
            None => {
                let mut rect = self.dummy_segment.clone();
                rect.transform_mut().unwrap().position = self.prev_head_position.clone();
                self.tail.push(rect);
            }
            Some(segment) => {
                let mut rect = self.dummy_segment.clone();
                rect.transform_mut().unwrap().position =
                    segment.transform().unwrap().position.clone();
                self.tail.push(rect);
            }
        }
    }

    pub fn kill(&mut self) {
        self.transform_mut().unwrap().position = self.prev_head_position.clone();

        for seg in &mut self.tail {
            seg.move_back();
        }

        self.dead = true;
    }

    pub fn reset(&mut self) {
        self.dead = false;
        self.tail.clear();
        self.transform_mut().unwrap().position = [0.0, 0.0].into();
        self.speed = self.start_speed;
        self.movement_timer.elapsed_reset();
        self.direction = Still;
    }
}

impl Object for Snake<'_> {
    fn draw(&self) {
        match self.tail.first() {
            None => {}
            Some(rect) => {
                rect.shader().unwrap().bind();
            }
        }

        for rect in &self.tail {
            rect.draw();
        }

        self.head_rect.shader().unwrap().bind();
        self.head_rect.draw();
    }

    fn tick(&mut self, _delta_time: f64) {
        if self.dead {
            return;
        }

        if input().key_was_pressed("KeyS") {
            self.add_segment();
        }

        let distance_to_move =
            (self.movement_timer.elapsed() / 1000.0) * self.speed + self.movement_carry;

        for key in input().key_pressed_map.keys() {
            let last_move = match self.movement_queue.last() {
                None => self.direction,
                Some(dir) => *dir,
            };

            if key == "ArrowUp" && last_move != Down {
                self.movement_queue.push(Up);
            } else if key == "ArrowDown" && last_move != Up {
                self.movement_queue.push(Down);
            } else if key == "ArrowLeft" && last_move != Right {
                self.movement_queue.push(Left);
            } else if key == "ArrowRight" && last_move != Left {
                self.movement_queue.push(Right);
            }
        }
        
        if distance_to_move >= TILE_SIZE {
            self.movement_timer.elapsed_reset();
            self.movement_carry = distance_to_move - TILE_SIZE;
            self.prev_head_position = self.transform().unwrap().position.clone();

            if !self.tail.is_empty() {
                let self_position = self.transform().unwrap().position.clone();
                for i in (1..self.tail.len()).rev() {
                    let position = self.tail[i - 1].transform().unwrap().position.clone();
                    self.tail[i].update_pos(position);
                }

                self.tail[0].update_pos(self_position);
            }

            let mut next_direction = if !self.movement_queue.is_empty() {
                self.movement_queue.remove(0)
            } else {
                Still
            };

            while next_direction != Still && self.direction == next_direction {
                next_direction = if !self.movement_queue.is_empty() {
                    self.movement_queue.remove(0)
                } else {
                    Still
                };
            }

            if next_direction != Still {
                self.direction = next_direction;
            }

            match self.direction {
                Up => {
                    self.transform_mut().unwrap().position.y += TILE_SIZE;
                    self.head_rect.transform_mut().unwrap().rotation = 0.0;
                }
                Down => {
                    self.transform_mut().unwrap().position.y -= TILE_SIZE;
                    self.head_rect.transform_mut().unwrap().rotation = PI;
                }
                Left => {
                    self.transform_mut().unwrap().position.x -= TILE_SIZE;
                    self.head_rect.transform_mut().unwrap().rotation = PI / 2.0;
                }
                Right => {
                    self.transform_mut().unwrap().position.x += TILE_SIZE;
                    self.head_rect.transform_mut().unwrap().rotation = -PI / 2.0;
                }
                Still => {}
            }
        }

        self.color.clone_into(self.head_rect.color_mut().unwrap());

        for seg in &mut self.tail {
            self.color.clone_into(seg.color_mut().unwrap());

            if self.head_rect.transform().unwrap().position == seg.transform().unwrap().position {
                self.kill();
                return;
            }
        }
    }

    fn transform(&self) -> Option<&Transform> {
        self.head_rect.transform()
    }

    fn transform_mut(&mut self) -> Option<&mut Transform> {
        self.head_rect.transform_mut()
    }

    fn set_transform(&mut self, transform: Transform) {
        self.head_rect.set_transform(transform)
    }

    fn shader(&self) -> Option<&Shader> {
        self.head_rect.shader()
    }

    fn color_mut(&mut self) -> Option<&mut [f32; 4]> {
        Some(&mut self.color)
    }

    fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    fn tags_mut(&mut self) -> &mut Vec<String> {
        &mut self.tags
    }
}
