use super::graphics::object::Object;
use super::graphics::texture::Texture;
use super::Timed;

use cgmath::Matrix4;
use std::f64::consts::PI;

pub struct Sphere {
    object: Object,
    texture: Texture,
    pub translate: Matrix4<f32>,
    timer_func: Box<dyn FnMut(&mut Self, f64)>,
}

impl Timed for Sphere {
    fn elapse_time(&mut self, delta: f64) {
        (self.timer_func)(self, delta);
    }
}

impl Sphere {
    pub fn new(radius: f64, slices: i32, stacks: i32, texture: &str) -> Sphere {
        if radius < 0. || slices < 3 || stacks < 2 {
            panic!("invalid params")
        };

        let mut vertices = vec![];
        let mut indices = vec![];

        for i in 0..(stacks + 1) {
            let irate = i as f64 / stacks as f64;
            let idir = PI * irate;
            for j in 0..(slices + 1) {
                let jrate = j as f64 / slices as f64;
                let jdir = 2. * PI * jrate;
                let (x, y, z) = (idir.sin() * jdir.cos(), idir.sin() * jdir.sin(), idir.cos());
                vertices.push([x * radius, y * radius, z * radius, jrate, irate, x, y, z]);

                // indices
                if j == slices {
                    continue;
                }
                let p = i * (slices + 1) + j;
                let pn = (i + 1) * (slices + 1) + j;
                if i != 0 {
                    indices.push([p, p + 1, pn + 1]);
                }
                if i != stacks {
                    indices.push([p, pn + 1, pn]);
                }
            }
        }

        Sphere {
            object: Object::new(vertices, indices),
            texture: Texture::new(texture),
            translate: Matrix4::from_scale(1.),
            timer_func: Box::new(|_, _| {}),
        }
    }
    pub fn set_timer_func(&mut self, f: Box<dyn FnMut(&mut Self, f64)>) {
        self.timer_func = f;
    }
}

impl super::Object for Sphere {
    unsafe fn draw(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.texture.id);
        self.object.draw();
    }
}
