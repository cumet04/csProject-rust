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

fn main() {
    let mut window = Window::new("csProject-rust", 800, 600);

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

    window.render_loop(|window| unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::BindTexture(gl::TEXTURE_2D, texture.id);
        shader.use_program();

        shader.set_vec3("lightColor", 1.0, 1.0, 1.0);
        shader.set_vec3("lightPos", 0.0, 0.0, 2.0);

        shader.set_mat4("model", &Matrix4::from_angle_x(Deg(-55.)));
        shader.set_mat4("view", &Matrix4::from_translation(vec3(0., 0., -3.)));
        let aspect_rate = window.width as f32 / window.height as f32;
        shader.set_mat4(
            "projection",
            &perspective(Deg(45.0), aspect_rate, 0.1, 100.0),
        );
        object.draw();
    });
}
