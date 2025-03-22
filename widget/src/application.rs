use crate::{loop_args::LoopArgs, window::Window};
use winit::{
    application::ApplicationHandler, error::EventLoopError::*, event::WindowEvent,
    event_loop::EventLoop,
};

pub struct Application {
    pub on_startup: Option<Box<dyn Fn(LoopArgs)>>,
}

static mut STARTUP: bool = false;

impl Application {
    pub fn new() -> Self {
        Application { on_startup: None }
    }

    pub fn enter_event_loop(mut self) -> i32 {
        let result = EventLoop::new().unwrap().run_app(&mut self);
        if let Err(err) = result {
            return match err {
                NotSupported(_) => -128,
                Os(os) => {
                    log::error!("{}", os);
                    -127
                }
                RecreationAttempt => -126,
                ExitFailure(ef) => ef,
            };
        }
        0
    }
}

impl ApplicationHandler for Application {
    fn resumed<'a>(&mut self, event_loop: &'a winit::event_loop::ActiveEventLoop) {
        if unsafe { STARTUP } {
            return;
        }
        unsafe {
            STARTUP = true;
        }
        if let Some(ref f) = self.on_startup {
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
                Window::from_id(window_id).unwrap().show(false);
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
