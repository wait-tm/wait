extern crate core;

mod app;
mod renderer;
mod game;
mod input;
mod registry;

use crate::app::App;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let event_loop =
        EventLoop::new().expect("Can't create event loop");

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();

    event_loop
        .run_app(&mut app)
        .expect("failed to run app")
}