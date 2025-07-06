//! UI widgets with main loop.
//!
//! Provides varieties of widgets, and no other dependencies
//! like winit required.

// use crate::app::App;
// use winit::error::EventLoopError;
// use winit::event_loop::{ControlFlow, EventLoop};

// mod app;
// mod app_impl;
pub mod app_context;
pub mod application;
pub mod button;
pub mod control;
pub mod control_ref;
mod ctx_view;
pub mod cvs_buffer;
mod init;
pub mod paint;
pub mod size;
mod vertex;
mod wgpu_ctx;
pub mod window;
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
