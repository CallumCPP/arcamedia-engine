mod segment;
mod snake;

use crate::engine::exit;
use crate::engine::input::input;
use crate::engine::object::rect::Rect;
use crate::engine::object::snake_game::snake::Snake;
use crate::engine::object::Object;
use crate::engine::object_manager::om;
use crate::engine::texture_manager::tm;
use crate::engine::transform::Transform;
use crate::engine::vec2::Vec2;
use crate::object;
use js_sys::Math::random;
use std::cell::RefCell;
use std::rc::Rc;

const TILE_SIZE: f64 = 50.0;

pub struct SnakeGame<'a> {
    tags: Vec<String>,
    bounds: Vec2,
    color: [f32; 4],
    playing: bool,
    dummy_wall: Rect,
    dummy_apple: Rect,
    snake: Snake<'a>,
}

impl SnakeGame<'_> {
    pub async fn new(bounds: Vec2) -> Self {
        let mut dummy_apple = Rect::new(
            [0.0, 0.0].into(),
            [TILE_SIZE - 5.0, TILE_SIZE - 5.0].into(),
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

        let tags: Vec<String> = [String::from("snake game")].into();

        let snake = Snake::new(
            200.0,
            [0.0, 0.0].into(),
            tm().get_texture("snake head.png").await,
        )
        .await;

        Self {
            dummy_apple,
            dummy_wall,
            tags,
            bounds,
            color: [40.0 / 255.0, 220.0 / 255.0, 40.0 / 255.0, 1.0],
            playing: false,
            snake,
        }
    }

    fn add_apple(&mut self) {
        let mut possible_spawns: Vec<Vec2> = Vec::new();
        let mut x = -self.bounds.x / 2.0 + TILE_SIZE;
        let mut y = -self.bounds.y / 2.0 + TILE_SIZE;

        while x < self.bounds.x / 2.0 {
            while y < self.bounds.y / 2.0 {
                possible_spawns.push([x, y].into());
                y += TILE_SIZE;
            }

            x += TILE_SIZE;
            y = -self.bounds.y / 2.0 + TILE_SIZE;
        }

        possible_spawns.retain(|pos| {
            for segment in &self.snake.tail {
                if *pos == segment.transform().unwrap().position {
                    return false;
                }
            }

            *pos != self.snake.head_rect.transform().unwrap().position
        });

        let pos =
            possible_spawns[(random() * possible_spawns.len() as f64).round() as usize].clone();
        let mut apple = self.dummy_apple.clone();
        apple.transform_mut().unwrap().position = pos;
        object!(apple);
    }
}

impl Object for SnakeGame<'_> {
    fn draw(&self) {
        self.snake.draw();
    }

    fn init(&mut self) {
        let bounds = &self.bounds;

        let mut wall = self.dummy_wall.clone();
        let transform = wall.transform_mut().unwrap();
        transform.position = [0.0, -bounds.y / 2.0].into();
        transform.size = [bounds.x + 10.0, 10.0].into();
        object!(wall);

        let mut wall = self.dummy_wall.clone();
        let transform = wall.transform_mut().unwrap();
        transform.position = [0.0, bounds.y / 2.0].into();
        transform.size = [bounds.x + 10.0, 10.0].into();
        object!(wall);

        let mut wall = self.dummy_wall.clone();
        let transform = wall.transform_mut().unwrap();
        transform.position = [-bounds.x / 2.0, 0.0].into();
        transform.size = [10.0, bounds.y + 10.0].into();
        object!(wall);

        let mut wall = self.dummy_wall.clone();
        let transform = wall.transform_mut().unwrap();
        transform.position = [bounds.x / 2.0, 0.0].into();
        transform.size = [10.0, bounds.y + 10.0].into();
        object!(wall);

        let button_size = Vec2::from([bounds.x / 4.0 - 1.0, bounds.y - TILE_SIZE]);

        let mut exit = self.dummy_wall.clone();
        let transform = exit.transform_mut().unwrap();
        transform.position = [-bounds.x / 2.0 + button_size.x / 2.0 + TILE_SIZE / 2.0, 0.0].into();
        transform.size = button_size.clone();
        [1.0, 0.0, 0.0, 1.0].clone_into(exit.color_mut().unwrap());
        exit.tags_mut().push("exit".into());
        object!(exit);

        let mut start = self.dummy_wall.clone();
        let transform = start.transform_mut().unwrap();
        transform.position = [bounds.x / 2.0 - button_size.x / 2.0 - TILE_SIZE / 2.0, 0.0].into();
        transform.size = button_size;
        [0.0, 1.0, 0.0, 1.0].clone_into(start.color_mut().unwrap());
        start.tags_mut().push("start".into());
        object!(start);
    }

    fn tick(&mut self, delta_time: f64) {
        if input().key_was_pressed("KeyA") {
            self.add_apple();
        }

        self.snake.tick(delta_time);

        for object_ref in &om().objects_on_screen {
            let object = match object_ref.try_borrow() {
                Ok(obj) => obj,
                Err(_) => {
                    continue;
                }
            };

            if object.tags().contains(&("start".into()))
                && object
                    .transform()
                    .unwrap()
                    .overlaps(self.snake.transform().unwrap())
            {
                om().remove_object_tag("start".into());
                om().remove_object_tag("exit".into());
                self.playing = true;
                self.snake.head_rect.transform_mut().unwrap().position = [0.0, 0.0].into();
                self.add_apple();
            }

            if object.tags().contains(&("exit".into()))
                && object
                    .transform()
                    .unwrap()
                    .overlaps(self.snake.transform().unwrap())
            {
                exit();
            }

            if object.tags().contains(&("wall".into()))
                && object
                    .transform()
                    .unwrap()
                    .overlaps(self.snake.transform().unwrap())
            {
                self.snake.transform_mut().unwrap().position =
                    self.snake.prev_head_position.clone();

                if self.playing {
                    self.snake.kill();
                }
            }

            if object.tags().contains(&("apple".into()))
                && object
                    .transform()
                    .unwrap()
                    .overlaps(self.snake.transform().unwrap())
            {
                self.snake.add_segment();
                self.add_apple();
                om().remove_object(object_ref.clone());
            }
        }
    }

    fn transform(&self) -> Option<&Transform> {
        self.snake.transform()
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
