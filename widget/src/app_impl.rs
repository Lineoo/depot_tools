use winit::{application::ApplicationHandler, event::WindowEvent};

use crate::window::{self, Window};

pub struct AppImpl {}

impl AppImpl {
    pub fn new() -> Self {
        AppImpl {}
    }
}

impl ApplicationHandler for AppImpl {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        todo!()
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                window::Window::from_id(window_id).unwrap().show(false);
            }
            WindowEvent::RedrawRequested => {
                if let Some(f) = Window::from_id(window_id).unwrap().on_paint {
                    f();
                }
            }
            _ => {}
        }
        todo!()
    }
}
