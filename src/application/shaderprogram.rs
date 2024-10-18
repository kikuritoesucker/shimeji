use core::panic;
use std::ffi::CString;
use gl::types::*;

pub struct ShaderProgram {
    pub id: GLuint,
}

impl ShaderProgram {
    pub fn new(mut vertex_src: String, mut fragment_src: String) -> ShaderProgram {
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
            
            // {
            //     let mut v: Vec<u8> = Vec::with_capacity(1024);
            //     let mut log_len = 0_i32;
            //     gl::GetShaderSource(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            //     v.set_len(log_len.try_into().unwrap());
            //     println!("-----vertex-----");
            //     println!("{}", String::from_utf8_lossy(&v));

            //     gl::GetShaderSource(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            //     v.set_len(log_len.try_into().unwrap());
            //     println!("-----fragment-----");
            //     println!("{}", String::from_utf8_lossy(&v));
            // }
            

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            ShaderProgram { id: program }
        }
    }
    pub fn get_attribute_id(&self, attrib: &str) -> u32 {
        unsafe { gl::GetAttribLocation(self.id, CString::new(attrib).unwrap().as_ptr()) as u32 }
    }

    pub fn get_uniform_id(&self, uniform: &str) -> u32 {
        unsafe { gl::GetUniformLocation(self.id, CString::new(uniform).unwrap().as_ptr()) as u32 }
    }
}