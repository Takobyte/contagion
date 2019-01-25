pub mod core;
pub mod presentation;
pub mod simulation;

#[macro_use] extern crate glium;

fn main() {
    presentation::graphics::renderer::renderer();
}
