extern crate gl;
extern crate sdl2;

use crate::presentation::graphics::render_gl;

use std::ffi::CString;
use std::{thread, time};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

pub fn create_window() {
    // initialize SDL library
    let sdl = sdl2::init().unwrap();
    // initialize video subsystem
    let video_subsystem = sdl.video().unwrap();
    // OpenGL context getters and setters
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core); // setting type of GL context
    gl_attr.set_context_version(4, 5); // specifying OpenGL version

    // creating window
    // available functionality: https://nukep.github.io/rust-sdl2/sdl2/video/struct.WindowBuilder.html#method.resizable
    let window = video_subsystem
        .window("Contagion", WIDTH, HEIGHT)
        .opengl()
        .resizable()
        .position_centered()
        .build()
        .unwrap();

    let mut time = Instant::now();

    // create OpenGL context
    let gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // setting up shader
    let vs =
        render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
            .unwrap();

    let fs =
        render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
            .unwrap();

    let shader_program = render_gl::Program::from_shaders(&[vs, fs]).unwrap();

    // set up vertex buffer object
        let vertices: Vec<f32> = vec![-1.0, -1.0, 0.0,  // Vertex 1 (X, Y, Z)
                                      1.0, -1.0, 0.0,  // Vertex 2 (X, Y, Z)
                                      0.0,  1.0, 0.0]; // Vertex 3 (X, Y, Z)

        let mut vbo: gl::types::GLuint = 0;
        unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,                                                       // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                               // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    // set up vertex array object
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),                                     // offset of the first component
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    // set up shared state for window

    unsafe {
        gl::Viewport(0, 0, 1024, 768); // viewport dimension
        gl::ClearColor(0.3, 0.3, 0.5, 1.0); // background color
    }

    // main loop

    let mut event_pump = sdl.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                // TODO: key input
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let time_now = Instant::now();
        let elapsed_t = time.elapsed();
        time = time_now;

        // TODO: draw other objects
        // ex: object.update(time);

        // draw triangle example
        shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                3,             // number of indices to be rendered
            );
        }

        window.gl_swap_window();
    }
}