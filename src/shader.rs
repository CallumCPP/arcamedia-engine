use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};

pub struct Shader<'a> {
    gl: &'a WebGl2RenderingContext,
    program: WebGlProgram,
}

impl<'a> Shader<'a> {
    pub fn new(
        gl: &'a WebGl2RenderingContext,
        vert_src: &str,
        frag_src: &str,
    ) -> Result<Shader<'a>, String> {
        let vert_shader = compile_shader(gl, WebGl2RenderingContext::VERTEX_SHADER, vert_src)?;

        let frag_shader = compile_shader(gl, WebGl2RenderingContext::FRAGMENT_SHADER, frag_src)?;

        let program = link_program(gl, &vert_shader, &frag_shader)?;

        Ok(Self { gl, program })
    }

    pub fn bind(&self) {
        self.gl.use_program(Some(&self.program));
    }

    pub fn get_uniform_location(&self, name: &str) -> Option<WebGlUniformLocation> {
        self.gl.get_uniform_location(&self.program, name)
    }

    pub fn uniform4fv_with_f32_array(&self, name: &str, data: [f32; 4]) {
        self.bind();

        self.gl.uniform4fv_with_f32_array(
            Some(&self.get_uniform_location(name).unwrap()),
            &data,
        );
    }
}

fn compile_shader(
    gl: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(String::from("Shader compilation error:\n")
            + &*gl
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    gl: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    gl.attach_shader(&program, vert_shader);
    gl.attach_shader(&program, frag_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(String::from("Shader linking error:\n")
            + &*gl
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
