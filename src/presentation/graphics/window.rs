extern crate sdl2;
use sdl2::render;
use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

pub fn create_window() {
    // initialize SDL2 libraries
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    // create a window
    let _window = video_subsystem
        .window("Contagion", 1000, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    // renderer
    let mut canvas : Canvas<Window> = _window.into_canvas()
        .present_vsync().build().unwrap();

    // make a square
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 210, 0));
    canvas.fill_rect(Rect::new(10, 10, 780, 580));
    canvas.present();

    // event receiver
    let mut event_pump = sdl.event_pump().unwrap();

    // main loop for the game
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }
    }
}