use crate::shader::Shader;
use crate::transform::Transform;

pub mod player;
pub mod rect;
pub mod textured_rect;

pub trait Object {
    fn draw(&self);
    fn tick(&mut self, _delta_time: f64) {}
    fn transform(&self) -> &Transform;
    fn transform_mut(&mut self) -> &mut Transform;
    fn shader(&self) -> &Shader;
    fn collides(&self) -> bool {
        false
    }
}
