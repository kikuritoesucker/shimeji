use std::{os::raw::c_void, rc::Rc};

use super::shaderprogram::ShaderProgram;
use gl::types::*;
use glfw;

pub struct Buffer<'a> {
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,

    indices: &'a Vec<GLuint>,
}

impl<'a> Buffer<'a> {
    /// `attributes : (index, size, type, normalized, stride, offset)`
    pub fn new<T>(
        data: &Vec<T>,
        indices: &'a Vec<GLuint>,
        usage: GLenum,
        attributes: &Vec<(GLuint, GLint, GLenum, GLboolean, GLsizei, GLuint)>,
    ) -> Self {
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;
        let mut ebo: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of::<T>() * data.len()) as GLsizeiptr,
                data.as_ptr().cast(),
                usage,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (std::mem::size_of::<GLuint>() * indices.len()) as GLsizeiptr,
                indices.as_ptr().cast(),
                usage,
            );

            for &attribute in attributes {
                let (index, size, type_, normalized, stride, offset) = attribute;
                gl::VertexAttribPointer(index, size, type_, normalized, stride, (std::mem::size_of::<T>() as u32 * offset) as *const _);
                gl::EnableVertexAttribArray(index);
            }
            gl::BindVertexArray(0);
        }
        Self {
            vao,
            vbo,
            ebo,
            indices,
        }
    }

    pub fn draw(&self, program: &ShaderProgram) {
        unsafe {
            gl::UseProgram(program.id);
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null());
           // gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}
