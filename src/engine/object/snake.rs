use std::cell::RefCell;
use std::cmp::PartialEq;
use crate::engine::input::input;
use crate::engine::object::rect::Rect;
use crate::engine::object::snake::Direction::*;
use crate::engine::object::textured_rect::TexturedRect;
use crate::engine::object::Object;
use crate::engine::object_manager::om;
use crate::engine::shader::Shader;
use crate::engine::texture::Texture;
use crate::engine::timer::Timer;
use crate::engine::transform::Transform;
use crate::engine::vec2::Vec2;
use std::f64::consts::PI;
use std::rc::Rc;
use js_sys::Math::random;
use crate::engine::exit;
use crate::object;

const TILE_SIZE: f64 = 50.0;

#[derive(PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake<'a> {
    speed: f64,
    head_rect: TexturedRect<'a>,
    movement_timer: Timer,
    movement_carry: f64,
    next_direction: Direction,
    real_direction: Direction,
    dead: bool,
    prev_head_position: Vec2,
    dummy_segment: Rect,
    dummy_apple: Rect,
    dummy_wall: Rect,
    tail: Vec<Rect>,
    apples: Vec<Rect>,
    tags: Vec<String>,
    bounds: Vec2,
    color: [f32; 4],
    playing: bool,
}

impl<'a> Snake<'a> {
    pub async fn new(speed: f64, position: Vec2, texture: &'a Texture, bounds: Vec2) -> Self {
        let head_rect = TexturedRect::new(
            position.clone(),
            [TILE_SIZE, TILE_SIZE].into(),
            0.0,
            [1.0, 1.0, 1.0, 1.0].into(),
            &texture,
            false,
        )
        .await;

        let movement_timer = Timer::new();

        let dummy_segment = Rect::new(
            [0.0, 0.0].into(),
            [TILE_SIZE-5.0, TILE_SIZE-5.0].into(),
            0.0,
            [40.0/255.0, 220.0/255.0, 40.0/255.0, 1.0],
            true,
        )
        .await;

        let mut dummy_apple = Rect::new(
            [0.0, 0.0].into(),
            [TILE_SIZE-5.0, TILE_SIZE-5.0].into(),
            0.0,
            [1.0, 0.2, 0.2, 1.0],
            true,
        )
        .await;
        dummy_apple.tags_mut().push("apple".into());

        let mut dummy_wall = Rect::new(
            [0.0, 0.0].into(),
            [0.0, 0.0].into(),
            0.0,
            [1.0, 1.0, 1.0, 1.0],
            true,
        )
        .await;
        dummy_wall.tags_mut().push("wall".into());

        let tail: Vec<Rect> = Vec::new();
        let apples: Vec<Rect> = Vec::new();
        let tags: Vec<String> = Vec::new();

        Self {
            speed,
            head_rect,
            movement_timer,
            movement_carry: 0.0,
            next_direction: Up,
            real_direction: Up,
            dead: false,
            prev_head_position: [0.0, 0.0].into(),
            dummy_segment,
            dummy_apple,
            dummy_wall,
            tail,
            apples,
            tags,
            bounds,
            color: [40.0/255.0, 220.0/255.0, 40.0/255.0, 1.0],
            playing: false,
        }
    }

    fn add_segment(&mut self) {
        match self.tail.last() {
            None => {
                let mut rect = self.dummy_segment.clone();
                rect.transform_mut().position = self.prev_head_position.clone();
                self.tail.push(rect);
            }
            Some(segment) => {
                let mut rect = self.dummy_segment.clone();
                rect.transform_mut().position = segment.transform().position.clone();
                self.tail.push(rect);
            }
        }
    }

    fn add_apple(&mut self) {
        let mut possible_spawns: Vec<Vec2> = Vec::new();
        let mut x = -self.bounds.x/2.0 + TILE_SIZE;
        let mut y = -self.bounds.y/2.0 + TILE_SIZE;

        while x < self.bounds.x/2.0 {
            while y < self.bounds.y/2.0 {
                possible_spawns.push([x, y].into());
                y += TILE_SIZE;
            }

            x += TILE_SIZE;
            y = -self.bounds.y/2.0 + TILE_SIZE;
        }

        possible_spawns.retain(|pos| {
            for segment in &self.tail {
                if *pos == segment.transform().position {
                    return false;
                }
            }

            *pos != self.head_rect.transform().position
        });

        let pos = possible_spawns[(random() * possible_spawns.len() as f64).round() as usize].clone();
        let mut apple = self.dummy_apple.clone();
        apple.transform_mut().position = pos;
        object!(apple);
    }

    fn kill() {

    }
}

impl Object for Snake<'_> {
    fn draw(&self) {
        match self.tail.first() {
            None => {}
            Some(rect) => {
                rect.shader().bind();
            }
        }

        for rect in &self.tail {
            rect.draw();
        }

        self.head_rect.shader().bind();
        self.head_rect.draw();
    }

    fn init(&mut self) {
        let bounds = &self.bounds;

        let mut wall = self.dummy_wall.clone();
        let transform = wall.transform_mut();
        transform.position = [0.0, -bounds.y/2.0].into();
        transform.size = [bounds.x+10.0, 10.0].into();
        object!(wall);

        let mut wall = self.dummy_wall.clone();
        let transform = wall.transform_mut();
        transform.position = [0.0, bounds.y/2.0].into();
        transform.size = [bounds.x+10.0, 10.0].into();
        object!(wall);

        let mut wall = self.dummy_wall.clone();
        let transform = wall.transform_mut();
        transform.position = [-bounds.x/2.0, 0.0].into();
        transform.size = [10.0, bounds.y+10.0].into();
        object!(wall);

        let mut wall = self.dummy_wall.clone();
        let transform = wall.transform_mut();
        transform.position = [bounds.x/2.0, 0.0].into();
        transform.size = [10.0, bounds.y+10.0].into();
        object!(wall);

        let mut button_size = Vec2::from([bounds.x/4.0-1.0, bounds.y-TILE_SIZE]);
        // button_size.x -= button_size.x % (TILE_SIZE/2.0);

        let mut exit = self.dummy_wall.clone();
        let transform = exit.transform_mut();
        transform.position = [-bounds.x/2.0 + button_size.x/2.0 + TILE_SIZE/2.0, 0.0].into();
        transform.size = button_size.clone();
        [1.0, 0.0, 0.0, 1.0].clone_into(exit.color_mut().unwrap());
        exit.tags_mut().push("exit".into());
        object!(exit);

        let mut start = self.dummy_wall.clone();
        let transform = start.transform_mut();
        transform.position = [bounds.x/2.0 - button_size.x/2.0 - TILE_SIZE/2.0, 0.0].into();
        transform.size = button_size;
        [0.0, 1.0, 0.0, 1.0].clone_into(start.color_mut().unwrap());
        start.tags_mut().push("start".into());
        object!(start);
    }

    fn tick(&mut self, _delta_time: f64) {
        if self.dead {
            return;
        }

        self.color.clone_into(self.head_rect.color_mut().unwrap());

        for segment in &mut self.tail {
            self.color.clone_into(segment.color_mut().unwrap());
        }

        let distance_to_move =
            (self.movement_timer.elapsed() / 1000.0) * self.speed + self.movement_carry;

        if input().key_was_pressed("ArrowUp") && self.real_direction != Down {
            self.next_direction = Up;
        } else if input().key_was_pressed("ArrowDown") && self.real_direction != Up {
            self.next_direction = Down;
        } else if input().key_was_pressed("ArrowLeft") && self.real_direction != Right {
            self.next_direction = Left;
        } else if input().key_was_pressed("ArrowRight") && self.real_direction != Left {
            self.next_direction = Right;
        }

        if input().key_was_pressed("KeyS") {
            self.add_segment();
        }

        if input().key_was_pressed("KeyA") {
            self.add_apple();
        }

        if distance_to_move >= TILE_SIZE {
            self.movement_timer.elapsed_reset();
            self.movement_carry = distance_to_move - TILE_SIZE;
            self.prev_head_position = self.transform().position.clone();

            if self.tail.len() > 0 {
                for i in (1..self.tail.len()).rev() {
                    self.tail[i].transform_mut().position = self.tail[i - 1].transform().position.clone();
                }

                self.tail[0].transform_mut().position = self.transform().position.clone();
            }

            self.real_direction = self.next_direction.clone();

            match self.real_direction {
                Up => {
                    self.transform_mut().position.y += TILE_SIZE;
                    self.head_rect.transform_mut().rotation = 0.0;
                }
                Down => {
                    self.transform_mut().position.y -= TILE_SIZE;
                    self.head_rect.transform_mut().rotation = PI;
                }
                Left => {
                    self.transform_mut().position.x -= TILE_SIZE;
                    self.head_rect.transform_mut().rotation = PI / 2.0;
                }
                Right => {
                    self.transform_mut().position.x += TILE_SIZE;
                    self.head_rect.transform_mut().rotation = -PI / 2.0;
                }
            }
        }

        for seg in &self.tail {
            if self.head_rect.transform().position == seg.transform().position {
                self.transform_mut().position = self.prev_head_position.clone();
                self.dead = true;
                return;
            }
        }

        for object_ref in &om().objects_on_screen {
            let object = match object_ref.try_borrow() {
                Ok(obj) => { obj }
                Err(_) => { continue; }
            };

            if object.tags().contains(&("start".into())) && object.transform().overlaps(self.transform()) {
                om().remove_object_tag("start".into());
                om().remove_object_tag("exit".into());
                self.playing = true;
                self.head_rect.transform_mut().position = [0.0, 0.0].into();
                self.add_apple();
            }

            if object.tags().contains(&("exit".into())) && object.transform().overlaps(self.transform()) {
                exit();
            }

            if object.tags().contains(&("wall".into())) && object.transform().overlaps(self.transform()) {
                self.transform_mut().position = self.prev_head_position.clone();
                self.dead = self.playing;
            }

            if object.tags().contains(&("apple".into())) && object.transform().overlaps(self.transform()) {
                self.add_segment();
                self.add_apple();
                om().remove_object(object_ref.clone());
            }
        }
    }

    fn transform(&self) -> &Transform {
        self.head_rect.transform()
    }

    fn transform_mut(&mut self) -> &mut Transform {
        self.head_rect.transform_mut()
    }

    fn set_transform(&mut self, transform: Transform) {
        self.head_rect.set_transform(transform)
    }

    fn shader(&self) -> &Shader {
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
