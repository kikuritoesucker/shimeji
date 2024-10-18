mod application;
mod event;
mod io;
mod linalg;
mod node;
mod tween;
use std::{cell::RefCell, rc::Rc};

use application::Program;
use gl::types::GLsizeiptr;
use linalg::*;
use node::*;
use scalar::*;

fn main() {
    let mut myapp =
        application::Application::new((1280, 720, "HelloWorld", glfw::WindowMode::Windowed));

    let vertices: Vec<f32> = vec![
        0.1, 0.1, 0.0,
        0.1, 0.7, 0.0,
        0.9, 0.4, 0.0];

    let (mut vao, mut vbo, mut ebo) = (0, 0, 0);
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (std::mem::size_of::<f32>() * vertices.len()) as GLsizeiptr,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
    }

    let VERTEX_SRC = r#"#version 330
    layout(location = 0) in vec3 aPos;
    void main() {
        gl_Position = vec4(aPos, 1.0);
    }
    "#
    .to_string();

    let FRAGMENT_SRC = r#"#version 330
    out vec4 FragColor;
    void main() {
        FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    }
    "#
    .to_string();

    let program = Program::new(VERTEX_SRC, FRAGMENT_SRC);

    myapp.run(
        |_| unsafe {
            gl::UseProgram(program.id);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(0);
            //gl::ClearColor(1.0, 0.5, 0.2, 1.0);
        },
        |window, event| {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {}
                _ => (),
            }
            //println!("{:?}", event);
        },
    );
}
