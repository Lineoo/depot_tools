use std::{collections::HashMap, process::exit};

use crate::{
    app_context::AppContext,
    window::{WinHandle, Window},
};
use winit::{
    application::ApplicationHandler, error::EventLoopError::*, event::WindowEvent,
    event_loop::EventLoop,
};

pub struct Application {
    on_startup: Option<Box<dyn FnOnce(AppContext)>>,
    started: bool,
    win_map: HashMap<winit::window::WindowId, Window>,
}

impl Application {
    pub fn new() -> Self {
        Application {
            on_startup: None,
            started: false,
            win_map: HashMap::new(),
        }
    }

    pub fn enter_event_loop(mut self) {
        let result = EventLoop::new().unwrap().run_app(&mut self);
        if let Err(err) = result {
            match err {
                NotSupported(_) => {}
                Os(os) => {
                    log::error!("{}", os);
                }
                RecreationAttempt => {}
                ExitFailure(ef) => exit(ef),
            };
        }
    }

    pub fn on_init<T: FnOnce(AppContext) + 'static>(&mut self, f: T) {
        self.on_startup = Some(Box::new(f));
    }

    pub fn reg_win(&mut self, w: Window) {
        self.win_map.insert(w.win.id(), w);
    }

    pub fn get_win(&self, handle: WinHandle) -> Option<&Window> {
        self.win_map.get(&handle.id)
    }

    pub fn get_win_mut(&mut self, handle: WinHandle) -> Option<&mut Window> {
        self.win_map.get_mut(&handle.id)
    }

    pub fn destroy_win(&mut self, handle: WinHandle) {
        self.win_map.remove(&handle.id);
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.started {
            return;
        }
        self.started = true;

        if let Some(f) = self.on_startup.take() {
            f(AppContext::new(event_loop, self));
        }

        // todo!()
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                // Window::from_id(window_id).unwrap().show(false);
                self.win_map.remove(&window_id);
            }
            WindowEvent::RedrawRequested => {
                if let Some(f) = self.win_map.get(&window_id).unwrap().on_paint {
                    f();
                }
            }
            _ => {}
        }
        // todo!()
    }
}
