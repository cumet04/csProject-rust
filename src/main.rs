extern crate gl;

extern crate cgmath;
use cgmath::{perspective, vec3, Deg, Matrix4};

extern crate image;

extern crate cs_project_rust;

use cs_project_rust::shader::Shader;
use cs_project_rust::sphere::Sphere;
use cs_project_rust::world::World;

fn main() {
    let mut world = World::<Box<dyn FnMut(f64)>>::new("csProject-rust", 800, 600);

    let shader = Shader::new("texture");
    let sphere = Sphere::new(0.5, 32, 32, "earthmap.jpg");

    let mut deg = 0.;

    let timer_func = Box::new(move |delta| unsafe {
        shader.use_program();

        shader.set_vec3("lightColor", 1.0, 1.0, 1.0);
        shader.set_vec3("lightPos", 0.0, 0.0, 5.0);

        deg = deg + delta * 100.;
        if deg > 360. {
            deg = deg - 360.;
        }
        shader.set_mat4("model", &Matrix4::from_angle_y(Deg(deg as f32)));
        shader.set_mat4("view", &Matrix4::from_translation(vec3(0., 0., -3.)));
        let aspect_rate = 800 as f32 / 600 as f32;
        shader.set_mat4(
            "projection",
            &perspective(Deg(45.0), aspect_rate, 0.1, 100.0),
        );
        sphere.draw();
    });
    world.set_timer_func(timer_func);

    world.main_loop();
}
