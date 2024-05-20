use crate::shader::Shader;
use crate::transform::Transform;
use std::cell::RefCell;
use std::rc::Rc;

pub mod player;
pub mod rect;
pub mod textured_rect;

pub trait Object {
    fn draw(&self);
    fn tick(&mut self, delta_time: f64) {}
    fn transform(&self) -> &Transform;
    fn transform_mut(&mut self) -> &mut Transform;
    fn shader(&self) -> &Shader;
}
