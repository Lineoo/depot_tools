use winit::{application::ApplicationHandler, event::WindowEvent};

use crate::{
    application::Application,
    loop_args::LoopArgs,
    window::{self, Window},
};

pub struct AppImpl {
    app: *const Application,
}

impl AppImpl {
    pub fn new(application: &Application) -> Self {
        AppImpl { app: &*application }
    }
}

static mut STARTUP: bool = false;

impl ApplicationHandler for AppImpl {
    fn resumed<'a>(&mut self, event_loop: &'a winit::event_loop::ActiveEventLoop) {
        if unsafe { STARTUP } {
            return;
        }
        let f = unsafe {
            STARTUP = true;
            &*self.app
        };
        if let Some(f) = &f.on_startup {
            f(LoopArgs::new(&event_loop));
        }

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
