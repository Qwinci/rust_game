use glfw::{Action, Context, Key, OpenGlProfileHint, WindowEvent, WindowHint};
use glad_gl::gl;

fn main() {
    // Initialize GLFW
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // create a window with OpenGL 4.5 core context
    glfw.window_hint(WindowHint::ContextVersion(4, 5));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    let (mut window, events) = glfw.create_window(
        800,
        600,
        "Game",
        glfw::WindowMode::Windowed
    ).expect("Failed to create window");

    // Make the window's OpenGL context current (it needs to be current before calling
    // any OpenGL functions.
    window.make_current();
    // Make GLFW automatically poll for keys without needing to have a manual callback.
    window.set_key_polling(true);

    // Use GLAD and GLFW to load OpenGL functions before we use them.
    gl::load(|name| glfw.get_proc_address_raw(name));

    // Set background clear color to black
    unsafe {
        gl::ClearColor(0f32, 0f32, 0f32, 0f32);
    }

    while !window.should_close() {
        // Poll for events and process them.
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Key(Key::Escape, _scancode, Action::Press, _modifiers) => {
                    window.set_should_close(true);
                },
                _ => {}
            }
        }

        // Clear the window color buffer with the previously defined background color.
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // swap the front and back buffers so what we drew previously appears on the screen.
        window.swap_buffers();
    }
}
