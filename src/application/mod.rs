#![allow(unused)]
extern crate gl;
extern crate glfw;
extern crate glutin;
use core::panic;
use std::ffi::{c_char, c_void, CString};

use gl::types::*;
use glfw::{
    ffi::{glfwCreateWindow, glfwGetMonitors, glfwSwapBuffers},
    Context, WindowEvent,
};

pub struct Program {
    pub id: GLuint,
}

impl Program {
    pub fn new(mut vertex_src: String, mut fragment_src: String) -> Program {
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
            Program { id: program }
        }
    }
    pub fn get_attribute_id(&self, attrib: &str) -> u32 {
        unsafe { gl::GetAttribLocation(self.id, CString::new(attrib).unwrap().as_ptr()) as u32 }
    }

    pub fn get_uniform_id(&self, uniform: &str) -> u32 {
        unsafe { gl::GetUniformLocation(self.id, CString::new(uniform).unwrap().as_ptr()) as u32 }
    }
}

pub struct Application {
    glfw_context: glfw::Glfw,
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, WindowEvent)>,
}

impl Application {
    /// #### Window Settings:
    /// - `width`: `u32`
    /// - `height`: `u32`
    /// - `title`: `String`
    /// - `mode`: `WindowMode`
    pub fn new(window_settings: (u32, u32, &str, glfw::WindowMode)) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        let (width, height, title, mode) = window_settings;
        let (mut window, events) = glfw
            .create_window(width, height, title, mode)
            .expect("Failed to create window.");

        window.set_all_polling(true);
        window.make_current();

        gl::load_with(|s| window.get_proc_address(s) as *const _);

        glfw.set_swap_interval(glfw::SwapInterval::Adaptive);

        window.set_framebuffer_size_callback(|window, width, height| unsafe {
            gl::Viewport(0, 0, width, height);
        });

        Self {
            glfw_context: glfw,
            window,
            events,
        }
    }

    pub fn run(
        &mut self,
        program: impl Fn(&mut glfw::PWindow),
        event_handler: impl Fn(&mut glfw::PWindow, glfw::WindowEvent),
    ) {
        while !self.window.should_close() {
            self.glfw_context.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                (event_handler)(&mut self.window, event);
            }

            unsafe {
                (program)(&mut self.window);
            }

            self.window.swap_buffers();
            self.glfw_context.poll_events();
        }
    }

    pub fn kill(&mut self) {
        self.window.set_should_close(true);
    }
}
