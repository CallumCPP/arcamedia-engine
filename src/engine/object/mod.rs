use crate::engine::shader::Shader;
use crate::engine::transform::Transform;

pub mod player;
pub mod rect;
pub mod snake_game;
pub mod textured_rect;

pub trait Object {
    fn draw(&self) {}
    fn init(&mut self) {}
    fn tick(&mut self, _delta_time: f64) {}
    fn transform(&self) -> Option<&Transform> {
        None
    }
    fn transform_mut(&mut self) -> Option<&mut Transform> {
        None
    }
    fn set_transform(&mut self, _transform: Transform) {}
    fn shader(&self) -> Option<&Shader> {
        None
    }
    fn collides(&self) -> bool {
        false
    }
    fn color_mut(&mut self) -> Option<&mut [f32; 4]> {
        None
    }
    fn tags(&self) -> &Vec<String>;
    fn tags_mut(&mut self) -> &mut Vec<String>;
}
