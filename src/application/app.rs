use glfw::{
    ffi::{glfwCreateWindow, glfwGetMonitors, glfwSwapBuffers},
    Context, WindowEvent,
};

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
                gl::Clear(gl::COLOR_BUFFER_BIT);
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