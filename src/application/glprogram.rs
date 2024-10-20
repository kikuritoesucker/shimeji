use core::panic;
use gl::types::*;
use std::ffi::CString;

use crate::linalg::*;

pub struct Program {
    pub id: u32,

    pub vao: u32,
    pub vbo: u32,
    pub ebo: u32,

    pub texture_set: Vec<u32>,

    element_num: i32,

    pre_draw: Option<Box<dyn Fn()>>,
    post_draw: Option<Box<dyn Fn()>>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            id: 0,
            vao: 0,
            vbo: 0,
            ebo: 0,
            texture_set: vec![],
            element_num: 0,
            pre_draw: None,
            post_draw: None,
        }
    }

    /// Draw callbacks won't be copied. It's required to bind methods from scratch.
    pub fn from(other: &Self) -> Self {
        Self {
            id: other.id,
            vao: other.vao,
            vbo: other.vbo,
            ebo: other.ebo,
            texture_set: other.texture_set.clone(),

            element_num: other.element_num,
            pre_draw: None,
            post_draw: None,
        }
    }

    pub fn get_program_id(&self) -> GLuint {
        self.id
    }

    pub fn bind_shader(&mut self, mut vertex_src: String, mut fragment_src: String) {
        unsafe {
            vertex_src.push('\0');
            fragment_src.push('\0');
            // let vertex_source = CString::new(vertex_source);
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);

            let vertex_src_c = vertex_src.as_bytes().as_ptr() as *const _;
            let fragment_src_c = fragment_src.as_bytes().as_ptr() as *const _;

            gl::ShaderSource(vertex_shader, 1, &vertex_src_c, std::ptr::null());
            gl::CompileShader(vertex_shader);

            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("vertex shader error {}", String::from_utf8_lossy(&v));
            }

            gl::ShaderSource(fragment_shader, 1, &fragment_src_c, std::ptr::null());
            gl::CompileShader(fragment_shader);

            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("fragment shader error {}", String::from_utf8_lossy(&v));
            }

            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetProgramInfoLog(program, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("linking error {}", String::from_utf8_lossy(&v));
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            self.id = program;
        }
    }

    pub fn bind_buffer<T>(
        &mut self,
        data: &Vec<T>,
        indices: &Vec<GLuint>,
        usage: GLenum,
        attributes: &Vec<(GLuint, GLint, GLenum, GLboolean, GLuint)>,
    ) {
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

            let mut stride = 0;
            for &attribute in attributes {
                stride += attribute.1;
            }
            stride *= std::mem::size_of::<T>() as i32;

            for &attribute in attributes {
                let (index, size, type_, normalized, offset) = attribute;
                gl::VertexAttribPointer(
                    index,
                    size,
                    type_,
                    normalized,
                    stride,
                    (std::mem::size_of::<T>() as u32 * offset) as *const _,
                );
                gl::EnableVertexAttribArray(index);
            }
            gl::BindVertexArray(0);
        }
        self.vao = vao;
        self.vbo = vbo;
        self.ebo = ebo;
        self.element_num = indices.len().try_into().unwrap();
    }

    pub fn bind_pre_draw(&mut self, callback: Box<dyn Fn()>) {
        self.pre_draw = Some(callback);
    }

    pub fn bind_post_draw(&mut self, callback: Box<dyn Fn()>) {
        self.post_draw = Some(callback);
    }

    pub fn bind_texture(&mut self, tex: &super::Texture) {
        assert_ne!(tex.id, 0);
        self.texture_set.push(tex.id);
    }

    pub fn draw(&self) {
        unsafe {
            if let Some(callback) = &self.pre_draw {
                (callback)();
            }
            let tex_set = &self.texture_set;
            for (i, id) in tex_set.into_iter().enumerate() {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);
                gl::BindTexture(gl::TEXTURE_2D, *id);
            }

            gl::UseProgram(self.id);
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.element_num,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );

            if let Some(callback) = &self.post_draw {
                (callback)();
            }
        }
    }
    pub fn get_attribute_id(&self, attrib: &str) -> i32 {
        unsafe { gl::GetAttribLocation(self.id, CString::new(attrib).unwrap().as_ptr()) }
    }

    pub fn get_uniform_id(&self, uniform: &str) -> i32 {
        unsafe { gl::GetUniformLocation(self.id, CString::new(uniform).unwrap().as_ptr()) }
    }

    pub fn set_uniform<T>(&self, uniform: &str, value: T)
    where
        T: UniformData,
    {
        value.set_uniform(self, uniform);
    }
}

pub trait UniformData {
    fn set_uniform(&self, program: &Program, uniform: &str);
}

impl UniformData for f32 {
    fn set_uniform(&self, program: &Program, uniform: &str) {
        let location = program.get_uniform_id(uniform);
        unsafe {
            gl::Uniform1f(location, *self);
        }
    }
}

impl UniformData for f64 {
    fn set_uniform(&self, program: &Program, uniform: &str) {
        let location = program.get_uniform_id(uniform);
        unsafe {
            gl::Uniform1d(location, *self);
        }
    }
}

impl UniformData for i32 {
    fn set_uniform(&self, program: &Program, uniform: &str) {
        let location = program.get_uniform_id(uniform);
        unsafe {
            gl::Uniform1i(location, *self);
        }
    }
}

impl UniformData for Vec2f {
    fn set_uniform(&self, program: &Program, uniform: &str) {
        let location = program.get_uniform_id(uniform);
        unsafe {
            gl::Uniform2fv(location, 1, self.data[0].as_ptr() as *const GLfloat);
        }
    }
}

impl UniformData for Vec3f {
    fn set_uniform(&self, program: &Program, uniform: &str) {
        let location = program.get_uniform_id(uniform);
        unsafe {
            gl::Uniform3fv(location, 1, self.data[0].as_ptr() as *const GLfloat);
        }
    }
}

impl UniformData for Vec4f {
    fn set_uniform(&self, program: &Program, uniform: &str) {
        let location = program.get_uniform_id(uniform);
        unsafe {
            gl::Uniform4fv(location, 1, self.data[0].as_ptr() as *const GLfloat);
        }
    }
}

impl UniformData for Mat2f {
    fn set_uniform(&self, program: &Program, uniform: &str) {
        let location = program.get_uniform_id(uniform);
        unsafe {
            gl::UniformMatrix2fv(
                location,
                1,
                gl::FALSE,
                self.data[0].as_ptr() as *const GLfloat,
            );
        }
    }
}

impl UniformData for Mat3f {
    fn set_uniform(&self, program: &Program, uniform: &str) {
        let location = program.get_uniform_id(uniform);
        unsafe {
            gl::UniformMatrix3fv(
                location,
                1,
                gl::FALSE,
                self.data[0].as_ptr() as *const GLfloat,
            );
        }
    }
}

impl UniformData for Mat4f {
    fn set_uniform(&self, program: &Program, uniform: &str) {
        let location = program.get_uniform_id(uniform);
        unsafe {
            gl::UniformMatrix4fv(
                location,
                1,
                gl::FALSE,
                self.data[0].as_ptr() as *const GLfloat,
            );
        }
    }
}

pub fn compose_data<T>(datas: &Vec<(&Vec<T>, usize)>) -> Vec<T>
where
    T: Copy,
{
    let mut synthesized: Vec<T> = Vec::new();
    let batch_num = datas[0].0.len() / datas[0].1;
    for i in 0..batch_num {
        for (data, stride) in datas {
            assert_eq!(data.len() / stride, batch_num);
            let l = i * stride;
            let r = (i + 1) * stride;
            for e in l..r {
                synthesized.push(data[e]);
            }
        }
    }
    synthesized
}
