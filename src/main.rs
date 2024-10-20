mod application;
mod event;
mod io;
mod linalg;
mod node;
mod tween;

use std::{ffi::CString, u8};

use application::*;
use linalg::*;
use node::*;
use scalar::*;

fn main() {
    {
        let mut myapp =
            application::Application::new((1280, 720, "hello", glfw::WindowMode::Windowed));

        let mut program = Program::new();

        let (width, height, data) = Texture::read_from_file(
            r"C:\Users\26808\Desktop\Workspace\shimeji\assets\image\emoji.png",
        );
        let tex = Texture::new(GLTexture::Texture2D(
            width as i32,
            height as i32,
            data.as_ptr() as *const std::ffi::c_void,
            true,
        ));
        program.bind_texture(&tex);

        let vertices: Vec<f32> = vec![
            0.5, 0.5, 0.0,
             0.5, -0.5,0.0,
              -0.5, -0.5, 0.0, 
              -0.5, 0.5, 0.0,
        ];

        let color: Vec<f32> = vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0];

        let mut tex_coord: Vec<f32> = vec![1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        tex_coord.reverse();

        let data = compose_data(&vec![(&vertices, 3), (&color, 3), (&tex_coord, 2)]);

        println!("{:?}", data);

        let indices: Vec<u32> = vec![0, 1, 3, 1, 2, 3];

        let attrib = vec![
            (0, 3, gl::FLOAT, gl::FALSE, 0),
            (1, 3, gl::FLOAT, gl::FALSE, 3),
            (2, 2, gl::FLOAT, gl::FALSE, 6),
        ];

        program.bind_pre_draw(Box::new(|| unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
            gl::ClearColor(0.4, 1.0, 0.0, 0.5);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }));
        program.bind_buffer(&data, &indices, gl::STATIC_DRAW, &attrib);

        let vertex_src = r#"#version 330
    layout(location = 0) in vec3 aPos;
    layout(location = 1) in vec3 aCol;
    layout(location = 2) in vec2 texcoord;
    out vec3 Color;
    out vec2 texCoords;
    uniform mat4 transform;
    void main() {
        Color = aCol;
        texCoords = texcoord;
        gl_Position = transform * vec4(aPos, 1.0);
    }
    "#
        .to_string();

        let fragment_src = r#"#version 330
    in vec3 Color;
    in vec2 texCoords;
    out vec4 FragColor;
    uniform sampler2D tex;
    void main() {
        FragColor = texture(tex, texCoords);
    }
    "#
        .to_string();
        program.bind_shader(vertex_src, fragment_src);

        myapp.run(
            |window, glfw| unsafe {
                let aspect = {
                    let size = window.get_framebuffer_size();
                    size.0 as f32 / size.1 as f32
                };
                let mut transform = Mat4f::identity();
                // transform *= Mat4f::perspective(45_f32, aspect, -1.0, 10.0);
                transform *= Mat4f::ortho(-1.0, 1.0, -1.0, 1.0, -100.1, 100.0);
                transform *= Mat4f::rotation_axis(Vec3f::from([[1.0, 1.0, 1.0]]).normalize(), (glfw.get_time() as f32  * 100.0 ).to_radians());
                println!("{:?}", transform);
                program.set_uniform(
                    "transform",
                    transform
                    //Mat4f::rotation_x(glfw.get_time() as f32 * 24.0) * Mat4f::translation_xyz((glfw.get_time() * 20.0).sin() as f32, (glfw.get_time() * 13.0).sin() as f32, 0.0)
                );
                program.draw();
            },
            |window, event| {
                match event {
                    glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                        window.set_should_close(true);
                    }
                    _ => (),
                }
                //println!("{:?}", event);
            },
        );
    }
}
