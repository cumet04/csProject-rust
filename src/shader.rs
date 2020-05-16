use std::ffi::CString;
use std::ptr;
use std::str;

use gl;
use gl::types::*;

// use cgmath::prelude::*;
use cgmath::{Matrix, Matrix4 /*, Vector3*/};

static SHADERS: include_dir::Dir = include_dir!("./resources/shaders");

pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(shader_name: &str) -> Shader {
        let mut shader = Shader { id: 0 };

        // read shader files
        let v_cstr = CString::new(
            SHADERS
                .get_file(format!("{}.vs", shader_name))
                .unwrap()
                .contents(),
        )
        .unwrap();
        let f_cstr = CString::new(
            SHADERS
                .get_file(format!("{}.fs", shader_name))
                .unwrap()
                .contents(),
        )
        .unwrap();

        // compile and attach shaders
        unsafe {
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &v_cstr.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.check_compile_errors(vertex, "VERTEX");

            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &f_cstr.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.check_compile_errors(fragment, "FRAGMENT");

            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);
            shader.check_compile_errors(id, "PROGRAM");

            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            shader.id = id;
        }

        shader
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id)
    }

    // utility uniform functions
    // ------------------------------------------------------------------------
    // pub unsafe fn set_bool(&self, name: &str, value: bool) {
    //     gl::Uniform1i(
    //         gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr()),
    //         value as i32,
    //     );
    // }
    // pub unsafe fn set_int(&self, name: &str, value: i32) {
    //     gl::Uniform1i(
    //         gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr()),
    //         value,
    //     );
    // }

    // pub unsafe fn set_float(&self, name: &str, value: f32) {
    //     gl::Uniform1f(
    //         gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr()),
    //         value,
    //     );
    // }

    // pub unsafe fn set_vector3(&self, name: &str, value: &Vector3<f32>) {
    //     gl::Uniform3fv(
    //         gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr()),
    //         1,
    //         value.as_ptr(),
    //     );
    // }

    pub unsafe fn set_vec3(&self, name: &str, x: f32, y: f32, z: f32) {
        gl::Uniform3f(
            gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr()),
            x,
            y,
            z,
        );
    }

    pub unsafe fn set_mat4(&self, name: &str, mat: &Matrix4<f32>) {
        gl::UniformMatrix4fv(
            gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr()),
            1,
            gl::FALSE,
            mat.as_ptr(),
        );
    }

    unsafe fn check_compile_errors(&self, shader: u32, type_: &str) {
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(1024);
        info_log.set_len(1024 - 1); // subtract 1 to skip the trailing null character
        if type_ != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                    type_,
                    str::from_utf8(&info_log).unwrap()
                );
            }
        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                    type_,
                    str::from_utf8(&info_log).unwrap()
                );
            }
        }
    }
}
