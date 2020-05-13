use std::ffi::CString;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;

extern crate gl;

extern crate cgmath;
use cgmath::{perspective, vec3, Deg, Matrix4};

extern crate image;
use image::GenericImageView;

mod window;
use window::Window;

mod shader;
use shader::Shader;

mod object;
use object::Object;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
    let mut window = Window::new("csProject-rust", SCR_WIDTH, SCR_HEIGHT);

    let shader = Shader::new("texture");

    let object = Object::new(
        vec![
            [0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
            [0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0],
            [-0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0],
            [-0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
        ],
        vec![[0, 1, 3], [1, 2, 3]],
    );

    let texture = {
        let mut texture = 0;
        // load image, create texture and generate mipmaps
        let img = image::open(&Path::new("resources/textures/128.png")).unwrap();
        let data = img.to_bytes();
        let (width, height) = img.dimensions();
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        texture
    };

    window.render_loop(|| unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::BindTexture(gl::TEXTURE_2D, texture);
        shader.use_program();

        shader.set_vec3(&CString::new("lightColor").unwrap(), 1.0, 1.0, 1.0);
        shader.set_vec3(&CString::new("lightPos").unwrap(), 0.0, 0.0, 2.0);

        shader.set_mat4(
            &CString::new("model").unwrap(),
            &Matrix4::from_angle_x(Deg(-55.)),
        );
        shader.set_mat4(
            &CString::new("view").unwrap(),
            &Matrix4::from_translation(vec3(0., 0., -3.)),
        );
        shader.set_mat4(
            &CString::new("projection").unwrap(),
            &perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0),
        );

        gl::BindVertexArray(object.vao);
        gl::DrawElements(
            gl::TRIANGLES,
            object.vertices_count,
            gl::UNSIGNED_INT,
            ptr::null(),
        );
    });
}
