mod application;
mod event;
mod io;
mod linalg;
mod node;
mod tween;

use application::*;
use linalg::*;
use node::*;
use scalar::*;


fn main() {
    {let mut myapp =
        application::Application::new((1280, 720, "HelloWorld", glfw::WindowMode::Windowed));

    let mut program = Program::new();

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
        -0.5, 0.5, 0.0,
        0.5, 0.0, 0.0,
    ];

    let color : Vec<f32> = vec![
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
    ];

    let tex_coord : Vec<f32> = vec![
        0.5, 0.0,
        0.0, 0.5,
        1.0, 0.0,
    ];

    let data = synthesize_data(&vec![
        (&vertices, 3), 
        (&color, 3), 
        (&tex_coord, 2)
    ]);

    println!("{:?}", data);

    let indices : Vec<u32> = vec![0, 1, 2];
    let attrib  = vec![
        (0, 3, gl::FLOAT, gl::FALSE, 0),
        (1, 3, gl::FLOAT, gl::FALSE, 3),
        (2, 2, gl::FLOAT, gl::FALSE, 6),
    ];

    program.bind_pre_draw(Box::new(||{
        unsafe{gl::ClearColor(0.2, 0.3, 0.4, 1.0);}
    }));
    program.bind_buffer(&data, &indices, gl::STATIC_DRAW, &attrib);

    // let (mut vao, mut vbo, mut ebo) = (0, 0, 0);
    // unsafe {
    //     gl::GenVertexArrays(1, &mut vao);
    //     gl::GenBuffers(1, &mut vbo);
    //     gl::BindVertexArray(vao);
    //     gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    //     gl::BufferData(
    //         gl::ARRAY_BUFFER,
    //         (std::mem::size_of::<f32>() * vertices.len()) as GLsizeiptr,
    //         vertices.as_ptr().cast(),
    //         gl::STATIC_DRAW,
    //     );
    //     gl::VertexAttribPointer(
    //         0,
    //         3,
    //         gl::FLOAT,
    //         gl::FALSE,
    //         (3 * std::mem::size_of::<f32>()) as i32,
    //         std::ptr::null(),
    //     );
    //     gl::EnableVertexAttribArray(0);
    // }

    let vertex_src = r#"#version 330
    layout(location = 0) in vec3 aPos;
    layout(location = 1) in vec3 aCol;
    layout(location = 2) in vec2 test;
    out vec3 Color;
    void main() {
        Color = aCol;
        gl_Position = vec4(aPos, 1.0) + vec4(test, 0.0, 0.0);
    }
    "#
    .to_string();

    let fragment_src = r#"#version 330
    in vec3 Color;
    out vec4 FragColor;
    void main() {
        FragColor = vec4(Color, 1.0);
    }
    "#
    .to_string();

    program.attach_shader(vertex_src, fragment_src);

    myapp.run(
        |_| unsafe {
            program.draw();
        },
        |window, event| {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {window.set_should_close(true);},
                _ => (),
            }
            //println!("{:?}", event);
        },
    );}
}
