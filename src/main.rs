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
use std::ffi::CString;


fn init() -> Result<(glium_sdl2::SDL2Facade, sdl2::EventPump, glium::texture::texture2d::Texture2d, glium::Program,
                     (glium::VertexBuffer<presentation::graphics::renderer::Vertex>, glium::index::NoIndices)), String> {
    // TODO: initialize music

    // initialize window and eventpump
    let window_tuple = presentation::graphics::renderer::create_window();
    let mut window = window_tuple.0;
    let mut event_pump = window_tuple.1;

    // load image -> type glium::texture::texture2d::Texture2d
    let texture = presentation::graphics::renderer::init_texture(&window);

    // create vertex buffer, indices, shader program
    let shader = presentation::graphics::renderer::init_shader(&window);
    let program = glium::Program::from_source(&window, include_str!("./presentation/graphics/vs.vert"),
                                              include_str!("./presentation/graphics/fs.frag"), None).unwrap();

Ok((window, event_pump, texture ,program, shader))
}

fn main() {
    // init
    let (mut window, mut event_pump, mut texture, mut program, mut shader) = match init() {
        // error handler if init fails
        Ok(t) => t,
        Err(err) => {
            println!("{}",err);
            std::process::exit(1);
        },
    };
    let vertex_buffer = shader.0;
    let indices = shader.1;

    // main game loop
    let mut running = true;
    while running {
        // draw background
        let mut target = window.draw();
        // do drawing here...
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ 0.0 , 0.0, 0.0, 1.0f32],
            ],
            tex: &texture,
        };
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        // Event loop: polls for events sent to all windows
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
