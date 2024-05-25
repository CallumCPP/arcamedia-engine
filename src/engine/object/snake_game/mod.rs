use crate::engine::{exit, gl};
use crate::engine::object::rect::Rect;
use crate::engine::object::snake_game::snake::Snake;
use crate::engine::object::Object;
use crate::engine::object_manager::om;
use crate::engine::text_renderer::tr;
use crate::engine::texture_manager::tm;
use crate::engine::transform::Transform;
use crate::engine::vec2f::Vec2f;
use crate::object;
use js_sys::Math::random;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::WebGl2RenderingContext;
use crate::engine::timer::Timer;

mod segment;
mod snake;

const TILE_SIZE: f64 = 50.0;

pub struct SnakeGame<'a> {
    tags: Vec<String>,
    bounds: Vec2f,
    color: [f32; 4],
    playing: bool,
    dummy_wall: Rect,
    dummy_apple: Rect,
    snake: Snake<'a>,
    end_timer: Timer,
    end: bool,
    won: bool,
    max_segments: i32,
}

impl SnakeGame<'_> {
    pub async fn new(bounds: Vec2f) -> Self {
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

        let mut max_segments = (((bounds.x-TILE_SIZE) / TILE_SIZE) * ((bounds.y-TILE_SIZE) / TILE_SIZE)) as i32;
        max_segments -= 1;

        log!("{max_segments}");

        Self {
            dummy_apple,
            dummy_wall,
            tags,
            bounds,
            color: [40.0 / 255.0, 220.0 / 255.0, 40.0 / 255.0, 1.0],
            playing: false,
            snake,
            end_timer: Timer::new(),
            end: false,
            won: false,
            max_segments,
        }
    }

    fn add_apple(&mut self) {
        let mut possible_spawns: Vec<Vec2f> = Vec::new();
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

    fn end_game(&mut self) {
        let _ = self.end_timer.elapsed_reset();
        self.end = true;

        self.playing = false;
        self.snake.kill();

        tr().clear();
        tr().draw_text("You died!", [960.0, 350.0].into());
        tr().draw_text(format!("Final score: {}", self.snake.tail.len()).as_str(), [960.0, 400.0].into());
    }

    fn win_game(&mut self) {
        self.won = true;
    }
}

impl Object for SnakeGame<'_> {
    fn draw(&self) {
        if self.won {
            gl().clear_color(random() as f32, random() as f32, random() as f32, 1.0);


            tr().draw_text("WIN", [random() * 1920.0, random() * 1080.0].into());

            gl().clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        }

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

        let button_size = Vec2f::from([bounds.x / 4.0 - 1.0, bounds.y - TILE_SIZE]);

        let mut exit = self.dummy_wall.clone();
        let transform = exit.transform_mut().unwrap();
        transform.position = [-bounds.x / 2.0 + button_size.x / 2.0 + TILE_SIZE / 2.0, 0.0].into();
        transform.size = button_size.clone();
        tr().draw_text(
            "Exit",
            [transform.position.x + 960.0, transform.position.y + 540.0].into(),
        );
        [1.0, 0.0, 0.0, 1.0].clone_into(exit.color_mut().unwrap());
        exit.tags_mut().push("exit".into());
        object!(exit);

        let mut start = self.dummy_wall.clone();
        let transform = start.transform_mut().unwrap();
        transform.position = [bounds.x / 2.0 - button_size.x / 2.0 - TILE_SIZE / 2.0, 0.0].into();
        transform.size = button_size;
        tr().draw_text(
            "Start",
            [transform.position.x + 960.0, transform.position.y + 540.0].into(),
        );
        [0.0, 1.0, 0.0, 1.0].clone_into(start.color_mut().unwrap());
        start.tags_mut().push("start".into());
        object!(start);
    }

    fn tick(&mut self, delta_time: f64) {
        if self.snake.tail.len() as i32 == self.max_segments {
            self.win_game();
        }

        if self.end && self.end_timer.elapsed() / 1000.0 > 5.0 {
            tr().set_text_color("black".into());

            self.end = false;
            self.snake.reset();

            let button_size = Vec2f::from([self.bounds.x / 4.0 - 1.0, self.bounds.y - TILE_SIZE]);

            let mut exit = self.dummy_wall.clone();
            let transform = exit.transform_mut().unwrap();
            transform.position = [-self.bounds.x / 2.0 + button_size.x / 2.0 + TILE_SIZE / 2.0, 0.0].into();
            transform.size = button_size.clone();
            tr().draw_text(
                "Exit",
                [transform.position.x + 960.0, transform.position.y + 540.0].into(),
            );
            [1.0, 0.0, 0.0, 1.0].clone_into(exit.color_mut().unwrap());
            exit.tags_mut().push("exit".into());
            object!(exit);

            let mut start = self.dummy_wall.clone();
            let transform = start.transform_mut().unwrap();
            transform.position = [self.bounds.x / 2.0 - button_size.x / 2.0 - TILE_SIZE / 2.0, 0.0].into();
            transform.size = button_size;
            tr().draw_text(
                "Start",
                [transform.position.x + 960.0, transform.position.y + 540.0].into(),
            );
            [0.0, 1.0, 0.0, 1.0].clone_into(start.color_mut().unwrap());
            start.tags_mut().push("start".into());
            object!(start);
        }

        if self.playing {
            tr().clear();
            tr().draw_text(
                format!("{}", self.snake.tail.len()).as_str(),
                [960.0, 80.0].into(),
            );
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
                tr().clear();
                tr().set_text_color("yellow".into());
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
                    self.end_game();
                }
            }

            if object.tags().contains(&("apple".into()))
                && object
                    .transform()
                    .unwrap()
                    .overlaps(self.snake.transform().unwrap())
            {
                self.snake.speed += 5.0;
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
