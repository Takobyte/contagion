//extern crate glfw;
extern crate sdl2;
//use glfw::{Action, Context, Key};

pub fn create_window() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let _window = video_subsystem
        .window("Contagion", 1000, 600)
        .resizable()
        .build()
        .unwrap();

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        // render window contents here
    }
}

//fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
//    match event {
//        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
//            window.set_should_close(true)
//        },
//        _ => {},
//    }
//}