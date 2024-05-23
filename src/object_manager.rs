use crate::camera::Camera;
use crate::object::Object;
use crate::shader_manager::sm;
use crate::transform::Transform;
use std::cell::RefCell;
use std::rc::Rc;
use std::ptr;

#[macro_export]
macro_rules! object {
    ($object: expr) => {
        om().add_object(Rc::new(RefCell::new($object)))
    };
}

static mut OM: Option<Box<ObjectManager>> = None;

pub struct ObjectManager {
    pub objects: Vec<Rc<RefCell<dyn Object>>>,
    pub objects_on_screen: Vec<Rc<RefCell<dyn Object>>>,
    pub camera: Camera,
    screen_transform: Transform,
}

impl ObjectManager {
    pub fn init() {
        let objects: Vec<Rc<RefCell<dyn Object>>> = Vec::new();
        let objects_on_screen: Vec<Rc<RefCell<dyn Object>>> = Vec::new();
        let camera = Camera::new([0.0, 0.0].into(), 1.0);

        let screen_transform: Transform =
            Transform::new(camera.position.clone(), [2120.0, 1280.0].into(), 0.0); // 200px buffer around edge of screen

        unsafe {
            OM = Some(Box::new(Self {
                objects,
                objects_on_screen,
                camera,
                screen_transform,
            }))
        }
    }

    pub fn add_object(&mut self, object: Rc<RefCell<dyn Object>>) -> Rc<RefCell<dyn Object>> {
        self.objects.push(object);
        self.objects.last().expect("Will be an object.").clone()
    }

    pub fn remove_object(&mut self, object: Rc<RefCell<dyn Object>>) {
        for i in 1..self.objects.len() {
            if Rc::ptr_eq(&self.objects[i], &object) {
                self.objects.remove(i);
                return;
            }
        }
    }

    pub fn tick(&mut self, delta_time: f64) {
        self.screen_transform.position = self.camera.position.clone();

        self.objects_on_screen = self.objects_in_bounds(&self.screen_transform);

        self.objects[0].borrow_mut().tick(delta_time);

        self.camera.tick();
        sm().update_camera(&self.camera);
    }

    pub fn draw(&self) {
        let mut curr_shader_id = -1;
        for object in &self.objects_on_screen[1..] {
            let object = object.borrow();
            let shader = object.shader();

            if shader.id() != curr_shader_id {
                curr_shader_id = shader.id();
                shader.bind();
            }

            object.draw();
        }

        self.objects[0].borrow().shader().bind();
        self.objects[0].borrow().draw();
    }

    pub fn objects_in_bounds(&self, transform: &Transform) -> Vec<Rc<RefCell<dyn Object>>> {
        let mut objects_in_bounds: Vec<Rc<RefCell<dyn Object>>> = Vec::new();

        for object in &self.objects {
            let object_ref = match object.try_borrow() {
                Ok(r) => r,
                Err(_) => {
                    continue;
                }
            };

            if object_ref.transform().overlaps_lazy(transform) {
                objects_in_bounds.push(object.clone());
            }
        }

        objects_in_bounds
    }
}

pub fn om() -> &'static mut ObjectManager {
    unsafe {
        OM.as_deref_mut()
            .expect("Shader manager should be initialized")
    }
}
