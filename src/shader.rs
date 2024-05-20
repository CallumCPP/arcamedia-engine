use crate::camera::Camera;
use crate::gl;
use crate::transform::Transform;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};

#[derive(Clone)]
pub struct Shader {
    program: WebGlProgram,
}

impl Shader {
    pub async fn new(vert_path: &str, frag_path: &str) -> Result<Shader, String> {
        let vert_src = crate::web::get_string(
            (String::from("/shaders/") + &*String::from(vert_path)).as_str(),
        )
        .await
        .expect("Should get shader source.");
        let vert_shader = compile_shader(WebGl2RenderingContext::VERTEX_SHADER, vert_src)?;

        let frag_src = crate::web::get_string(
            (String::from("/shaders/") + &*String::from(frag_path)).as_str(),
        )
        .await
        .expect("Should get shader source.");
        let frag_shader = compile_shader(WebGl2RenderingContext::FRAGMENT_SHADER, frag_src)?;

        let program = link_program(&vert_shader, &frag_shader)?;

        Ok(Self { program })
    }

    pub fn bind(&self) {
        gl().use_program(Some(&self.program));
    }

    pub fn get_uniform_location(&self, name: &str) -> Option<WebGlUniformLocation> {
        gl().get_uniform_location(&self.program, name)
    }

    pub fn uniform_transform(&self, transform: &Transform) {
        self.uniform_coordinates_with_f64_array("transform.position", &transform.position.as_arr());
        self.uniform2fv_with_f64_array("transform.size", &transform.size.as_arr());
        self.uniform1f("transform.rotation", transform.rotation as f32);
    }

    pub fn uniform_camera(&self, camera: &Camera) {
        self.uniform_coordinates_with_f64_array("camera.position", &camera.position.as_arr());
        self.uniform1f("camera.zoom", camera.zoom as f32);
    }

    pub fn uniform4fv_with_f32_array(&self, name: &str, data: &[f32; 4]) {
        gl().uniform4fv_with_f32_array(Some(&self.get_uniform_location(name).unwrap()), data);
    }

    pub fn uniform_coordinates_with_f64_array(&self, name: &str, data: &[f64; 2]) {
        gl().uniform2fv_with_f32_array(
            Some(&self.get_uniform_location(name).unwrap()),
            &[data[0] as f32 / 960.0, data[1] as f32 / 540.0],
        );
    }

    pub fn uniform2fv_with_f64_array(&self, name: &str, data: &[f64; 2]) {
        gl().uniform2fv_with_f32_array(
            Some(&self.get_uniform_location(name).unwrap()),
            &[data[0] as f32, data[1] as f32],
        );
    }

    pub fn uniform1f(&self, name: &str, data: f32) {
        gl().uniform1f(Some(&self.get_uniform_location(name).unwrap()), data);
    }

    pub fn uniform1i(&self, name: &str, data: i32) {
        gl().uniform1i(Some(&self.get_uniform_location(name).unwrap()), data);
    }
}

fn compile_shader(shader_type: u32, source: String) -> Result<WebGlShader, String> {
    let shader = gl()
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    gl().shader_source(&shader, source.as_str());
    gl().compile_shader(&shader);

    if gl()
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        let error: String = gl()
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader"));

        log!("Shader compilation error: {error}");

        Err(String::from("Shader compilation error:\n") + &*error)
    }
}

fn link_program(
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = gl()
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    gl().attach_shader(&program, vert_shader);
    gl().attach_shader(&program, frag_shader);
    gl().link_program(&program);

    if gl()
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(String::from("Shader linking error:\n")
            + &*gl()
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
