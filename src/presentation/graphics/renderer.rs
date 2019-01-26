extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;

use std::ffi::CString;
use std::{thread, time};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::time::Instant;

use glium::Surface;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

pub fn renderer() {
    use glium_sdl2::DisplayBuild;
    // initialize SDL library
    let sdl_context = sdl2::init().unwrap();
    // initialize video subsystem
    let video_subsystem = sdl_context.video().unwrap();
    // OpenGL context getters and setters
    let gl_attr = video_subsystem.gl_attr();
    let mut pause_time = false;

    // OpenGL version switcher for platform compatibility
    let mut major = 0;
    let mut minor = 0;
    if cfg!(macos) {
        major = 4;
        minor = 1;
    }
    if cfg!(linux) {
        major = 4;
        minor = 1;
    }
    if cfg!(windows) {
        major = 4;
        minor = 5;
    }

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core); // setting type of GL context
    // Set the context into debug mode
    gl_attr.set_context_flags().debug().set();
    gl_attr.set_context_version(4, 5); // specifying OpenGL version

    // creating window
    // available functionality: https://nukep.github.io/rust-sdl2/sdl2/video/struct.WindowBuilder.html#method.resizable
    let window = video_subsystem
        .window("Contagion", WIDTH, HEIGHT)
        .resizable()
        .build_glium()
        .unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&window, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut time = Instant::now();

    let program = glium::Program::from_source(&window, include_str!("triangle.vert"), include_str!("triangle.frag"), None).unwrap();

    // main loop
    let mut running = true;
    let mut event_pump = sdl_context.event_pump().unwrap();
    while running {
        let mut target = window.draw();
        // do drawing here...
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        // calculate time
        let time_now = Instant::now();
        let elapsed_t = time.elapsed();
        time = time_now;

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
