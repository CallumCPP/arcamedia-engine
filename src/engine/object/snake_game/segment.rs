use crate::engine::object::rect::Rect;
use crate::engine::object::snake_game::TILE_SIZE;
use crate::engine::object::Object;
use crate::engine::shader::Shader;
use crate::engine::transform::Transform;
use crate::engine::vec2f::Vec2f;

#[derive(Clone)]
pub struct Segment {
    tags: Vec<String>,
    pub rect: Rect,
    prev_position: Vec2f,
}

impl Segment {
    pub async fn new() -> Self {
        let rect = Rect::new(
            [0.0, 0.0].into(),
            [TILE_SIZE - 5.0, TILE_SIZE - 5.0].into(),
            0.0,
            [40.0 / 255.0, 220.0 / 255.0, 40.0 / 255.0, 1.0],
            true,
        )
        .await;

        Self {
            tags: [String::from("segment")].into(),
            rect,
            prev_position: [0.0, 0.0].into(),
        }
    }

    pub fn update_pos(&mut self, position: Vec2f) {
        self.prev_position = self.transform().unwrap().position.clone();
        self.transform_mut().unwrap().position = position;
    }

    pub fn move_back(&mut self) {
        self.transform_mut().unwrap().position = self.prev_position.clone();
    }
}

impl Object for Segment {
    fn draw(&self) {
        self.rect.draw();
    }

    fn transform(&self) -> Option<&Transform> {
        self.rect.transform()
    }

    fn transform_mut(&mut self) -> Option<&mut Transform> {
        self.rect.transform_mut()
    }

    fn set_transform(&mut self, transform: Transform) {
        self.rect.set_transform(transform);
    }

    fn shader(&self) -> Option<&Shader> {
        self.rect.shader()
    }

    fn color_mut(&mut self) -> Option<&mut [f32; 4]> {
        self.rect.color_mut()
    }

    fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    fn tags_mut(&mut self) -> &mut Vec<String> {
        &mut self.tags
    }
}
