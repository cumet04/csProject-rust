extern crate gl;

extern crate cgmath;
use cgmath::{Deg, Matrix4};

extern crate image;

#[macro_use]
extern crate include_dir;

mod world;
use world::sphere::Sphere;
use world::{Camera, Light, World};

fn main() {
    let mut world = World::new("csProject-rust", 800, 600);
    world.set_camera(Camera {
        x: 0.,
        y: 0.,
        z: 0.,
        fovy_deg: 45.,
        near: 0.1,
        far: 100.,
        timer_func: Box::new(|_, _| {}),
    });
    world.set_light(Light {
        x: 0.,
        y: 0.,
        z: 5.,
        r: 1.,
        g: 1.,
        b: 1.,
        timer_func: Box::new(|_, _| {}),
    });

    let mut sphere = Sphere::new(0.5, 32, 32, "earthmap.jpg");
    let mut deg = 0.;
    sphere.set_timer_func(Box::new(move |obj, delta| {
        deg = deg + delta * 100.;
        if deg > 360. {
            deg = deg - 360.;
        }
        obj.translate = Matrix4::from_angle_y(Deg(deg as f32));
    }));

    world.main_loop();
}
