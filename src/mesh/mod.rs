pub mod dynamic_mesh;
pub mod dynamic_mesh_t;
pub mod static_mesh;

pub trait Mesh {
    fn new(vertices: Vec<f32>) -> Self;
    fn draw(&self);
}
