use std::mem;
use std::os::raw::c_void;
use std::ptr;

use gl;
use gl::types::*;

pub struct Object {
    pub vao: u32,
    pub vertices_count: i32, // number of vertex, for gl::DrawElements arg
    vbo: u32,
    ebo: u32,
}

impl Object {
    // verticies: [pos.x,pos.y,pos.z, tex.u,tex.v, normal.x,normal.y,normal.z]
    pub fn new(vertices: Vec<[f32; 8]>, indices: Vec<[i32; 3]>) -> Object {
        let mut obj = Object {
            vao: 0,
            vertices_count: 0,
            vbo: 0,
            ebo: 0,
        };

        obj.vao = unsafe {
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            vao
        };

        obj.vbo = {
            let mut vbo = 0;

            let joined = vertices.concat();
            obj.vertices_count = joined.len() as i32;
            unsafe {
                gl::GenBuffers(1, &mut vbo);
                gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (joined.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                    &joined[0] as *const f32 as *const c_void,
                    gl::STATIC_DRAW,
                );
            }
            vbo
        };

        obj.ebo = {
            let mut ebo = 0;

            let joined = indices.concat();
            unsafe {
                gl::GenBuffers(1, &mut ebo);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (joined.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                    &joined[0] as *const i32 as *const c_void,
                    gl::STATIC_DRAW,
                );
            }
            ebo
        };

        unsafe {
            let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;
            // position attribute
            gl::VertexAttribPointer(
                0, // attribute index
                3, // count of value
                gl::FLOAT,
                gl::FALSE,
                stride,
                ptr::null(), // offset
            );
            gl::EnableVertexAttribArray(0);
            // texture coord attribute
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);
            // normal attribute
            gl::VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (5 * mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(2);
        }

        obj
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}
