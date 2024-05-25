use crate::engine::camera::Camera;
use crate::engine::shader::Shader;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

static mut SM: Option<Box<ShaderManager>> = None;

pub fn sm() -> &'static mut ShaderManager {
    unsafe {
        SM.as_deref_mut()
            .expect("Shader manager should be initialized")
    }
}

pub struct ShaderManager {
    shaders: HashMap<String, Shader>,
    curr_id: i32,
}

impl ShaderManager {
    pub fn init() {
        let shaders: HashMap<String, Shader> = HashMap::new();

        unsafe {
            SM = Some(Box::new(Self {
                shaders,
                curr_id: 0,
            }))
        }
    }

    pub async fn get_shader(&mut self, vert_path: &str, frag_path: &str) -> &Shader {
        let path_amalgam = [vert_path, frag_path].join("");

        match self.shaders.entry(path_amalgam) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let shader = Shader::new(vert_path, frag_path, self.curr_id)
                    .await
                    .expect("Should be able to create shader");
                self.curr_id += 1;
                entry.insert(shader)
            }
        }
    }

    pub fn update_camera(&mut self, camera: &Camera) {
        for shader in self.shaders.values() {
            shader.bind();
            shader.uniform_camera(camera);
        }
    }
}
