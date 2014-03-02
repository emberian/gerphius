//! Entry point


#[crate_id = "gerphius"];
#[crate_type = "bin"];

extern crate native;

extern crate cgmath;
extern crate hgl;
extern crate noise;
extern crate png;
extern crate ears;
extern crate glfw = "glfw-rs";

use game::Game;

mod game;

// link to libglfw
#[link(name="glfw")]
extern { }

#[start]
fn start(argc: int, argv: **u8) -> int {
    // a little stub to start on the main thread, which glfw needs
    native::start(argc, argv, main)
}

fn main() {
    // when glfw errors, use the built-in console printer
    glfw::set_error_callback(box glfw::LogErrorHandler);
    // initialize glfw and run
    glfw::start(proc() {
        glfw::window_hint::resizable(true);

        // opengl 3.1
        glfw::window_hint::context_version(3, 1);

        // if you bump the gl version, enable the core profile
        // glfw::window_hint::opengl_profile(glfw::OpenGlCoreProfile);

        let window = glfw::Window::create(400, 300, "Gerphius", glfw::Windowed)
            .expect("Error: could not open a window!");

        // we want every event
        window.set_all_polling(true);

        // use this window's gl context
        window.make_context_current();

        let mut game = Game::new();

        while !window.should_close() {
            glfw::poll_events();
            for event in window.flush_events() {
                game.handle_event(&window, event);
            }
            game.tick();
            game.render();
            window.swap_buffers();
        }
    });
}
