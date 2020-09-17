mod graphics;
pub mod sphere;

use cgmath::{perspective, vec3, Deg, Matrix4};

use gl;
use graphics::shader::Shader;
use graphics::window::Window;

trait Timed {
    fn elapse_time(&mut self, delta: f64);
}

trait Object: Timed {
    unsafe fn draw(&self);
}

// light ----------
pub struct Light {
    x: f64,
    y: f64,
    z: f64,
    r: f64,
    g: f64,
    b: f64,
    timer_func: Box<dyn FnMut(&mut Self, f64)>,
}

impl Timed for Light {
    fn elapse_time(&mut self, delta: f64) {
        (self.timer_func)(self, delta);
    }
}

// camera ----------
pub struct Camera {
    x: f64,
    y: f64,
    z: f64,
    fovy_deg: f64,
    near: f64,
    far: f64,
    timer_func: Box<dyn FnMut(&mut Self, f64)>,
}

impl Timed for Camera {
    fn elapse_time(&mut self, delta: f64) {
        (self.timer_func)(self, delta);
    }
}

// world ----------

pub struct World {
    window: Window,
    shader: Shader,
    camera: Camera,
    objects: Vec<Box<dyn Object>>,
    light: Light,
}

impl World {
    pub fn new(title: &str, width: i32, height: i32) -> World {
        let window = Window::new(title, width, height);

        World {
            window: window,
            shader: Shader::new("texture"),
            camera: Camera {
                x: 0.,
                y: 0.,
                z: 0.,
                fovy_deg: 0.,
                near: 0.,
                far: 0.,
                timer_func: Box::new(|_, _| {}),
            },
            light: Light {
                x: 0.,
                y: 0.,
                z: 0.,
                r: 0.,
                g: 0.,
                b: 0.,
                timer_func: Box::new(|_, _| {}),
            },
            objects: vec![],
        }
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn set_light(&mut self, light: Light) {
        self.light = light;
    }

    pub fn main_loop(mut self) {
        let shader = self.shader;
        let mut camera = self.camera;
        let mut light = self.light;
        let objects = self.objects;
        self.window.render_loop(|delta| unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
            gl::FrontFace(gl::CW);

            shader.use_program();

            // time
            camera.elapse_time(delta);
            light.elapse_time(delta);
            for o in objects {
                o.elapse_time(delta);
            }

            // set light
            shader.set_vec3("lightColor", light.r as f32, light.g as f32, light.b as f32);
            shader.set_vec3("lightPos", light.x as f32, light.y as f32, light.z as f32);

            // set camera
            shader.set_mat4(
                "view",
                &Matrix4::from_translation(vec3(
                    -camera.x as f32,
                    -camera.y as f32,
                    -camera.z as f32,
                )),
            );
            let aspect_rate = 800 as f32 / 600 as f32; // TODO
            shader.set_mat4(
                "projection",
                &perspective(
                    Deg(camera.fovy_deg as f32),
                    aspect_rate,
                    camera.near as f32,
                    camera.far as f32,
                ),
            );

            // shader.set_mat4("model", &Matrix4::from_angle_y(Deg(deg as f32)));
            // draw
        });
    }
}
