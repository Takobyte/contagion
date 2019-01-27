pub mod constants;
pub mod core;
pub mod presentation;
pub mod simulation;

#[macro_use] extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;
extern crate image;

use std::io::Cursor;
use glium::Surface;
use glium::texture;

fn init() -> Result<(glium_sdl2::SDL2Facade, sdl2::EventPump), String> {
    // TODO: initialize music

    // initialize window and eventpump
    let window_tuple = presentation::graphics::renderer::create_window();
    let mut window = window_tuple.0;
    let mut event_pump = window_tuple.1;

    Ok((window, event_pump))
}

fn main() {
    // init
    let (mut window, mut event_pump) = match init() {
        // error handler if init fails
        Ok(t) => t,
        Err(err) => {
            println!("{}",err);
            std::process::exit(1);
        },
    };

    // main game loop
    let mut running = true;
    while running {
        // draw background
        let mut target = window.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();

        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                // TODO: key inputs
                Event::Quit { .. } => {
                    running = false;
                },
                _ => ()
            }
        }
    }
}
