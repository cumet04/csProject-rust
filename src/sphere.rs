use super::object::Object;
use super::texture::Texture;

use std::f64::consts::PI;

pub struct Sphere {
    object: Object,
    texture: Texture,
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

                if i > 0 {
                    indices.push([
                        j + (i - 1) * slices,
                        j + 1 + (i - 1) * slices,
                        j + 1 + i * slices,
                    ]);
                    indices.push([
                        j + 1 + (i - 1) * slices,
                        j + 2 + i * slices,
                        j + 1 + i * slices,
                    ]);
                }
            }
        }

        Sphere {
            object: Object::new(vertices, indices),
            texture: Texture::new(texture),
        }
    }

    pub unsafe fn draw(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.texture.id);
        self.object.draw();
    }
}
