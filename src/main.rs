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

fn init() -> Result<(glium_sdl2::SDL2Facade, sdl2::EventPump), String> {
    // TODO: initialize music

    // initialize window and eventpump
    let window_tuple = presentation::graphics::renderer::create_window();
    let mut window = window_tuple.0;
    let mut event_pump = window_tuple.1;
    // load image -> type glium::texture::texture2d::Texture2d
    let texture = presentation::graphics::renderer::init_texture(&window);
    // create vertex buffer, indices, shader program
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }

    implement_vertex!(Vertex, position, tex_coords);
    let vertex1 = Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] };
    let vertex2 = Vertex { position: [0.0, 0.5], tex_coords: [0.0, 1.0] };
    let vertex3 = Vertex { position: [0.5, -0.5], tex_coords: [1.0, 0.0] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&window, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program = glium::Program::from_source(&window, include_str!("./presentation/graphics/vs.vert"),
                                              include_str!("./presentation/graphics/fs.frag"), None).unwrap();

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
