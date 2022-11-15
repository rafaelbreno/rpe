extern crate glfw;

use gl33::global_loader::{glClearColor, load_global_gl};
use glfw::{Action, Context, Key};

const HEIGHT: u32 = 480;
const WIDTH: u32 = 640;
const TITLE: &str = "RPE";

fn main() {
    println!("Hello, world!");

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(WIDTH, HEIGHT, TITLE, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // glfwSetKeyCallback
    window.set_key_polling(true);

    // glfwMakeContextCurrent
    window.make_current();

    unsafe {
        load_global_gl(&|ptr| {
            let c_str = std::ffi::CStr::from_ptr(ptr as *const i8);
            let r_str = c_str.to_str().unwrap();
            window.get_proc_address(r_str)
        });
        glClearColor(0.2, 0.3, 0.3, 1.0)
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event)
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        },
        _ => {}
    }
}
