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
    fn set_transform(&mut self, transform: Transform);
    fn shader(&self) -> &Shader;
    fn collides(&self) -> bool {
        false
    }
    fn color_mut(&mut self) -> Option<&mut [f32; 4]> {
        None
    }
}
