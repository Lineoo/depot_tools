//! UI widgets with main loop.
//!
//! Provides varieties of widgets, and no other dependencies
//! like winit required.

// use crate::app::App;
// use winit::error::EventLoopError;
// use winit::event_loop::{ControlFlow, EventLoop};

#[macro_use]
extern crate lazy_static;

// mod app;
mod app_impl;
mod application;
mod init;
mod loop_args;
mod vertex;
mod wgpu_ctx;
mod window;

// fn main() -> Result<(), EventLoopError> {
//     env_logger::init();
//     let event_loop = EventLoop::new().unwrap();
//     event_loop.set_control_flow(ControlFlow::Poll);
//     let mut app = App::default();
//     event_loop.run_app(&mut app)
// }

#[cfg(test)]
mod tests {

    #[test]
    fn window_test() {
        todo!()
    }
}
