use std::ffi::CString;
use std::ptr;

extern crate gl;

extern crate cgmath;
use cgmath::{perspective, vec3, Deg, Matrix4};

extern crate image;

mod window;
use window::Window;

mod shader;
use shader::Shader;

mod object;
use object::Object;

mod texture;
use texture::Texture;

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

    let texture = Texture::new("128.png");

    window.render_loop(|| unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::BindTexture(gl::TEXTURE_2D, texture.id);
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
