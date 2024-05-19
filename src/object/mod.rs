use crate::shader::Shader;
use crate::transform::Transform;

pub mod rect;
pub mod textured_rect;

pub trait Object {
    fn draw(&mut self);
    fn tick(&mut self, delta_time: f64);
    fn transform(&mut self) -> &mut Transform;
    fn shader(&self) -> &Shader;
}
