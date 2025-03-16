use crate::{app_impl::AppImpl, loop_args::LoopArgs};
use winit::{error::EventLoopError::*, event_loop::EventLoop};

pub struct Application {
    app_impl: Option<AppImpl>,
    evt_loop: EventLoop<()>,
    pub on_startup: Option<Box<dyn Fn(LoopArgs)>>,
}

impl Application {
    pub fn new() -> Self {
        let evt_loop = EventLoop::new().unwrap();
        let mut app = Application {
            app_impl: None,
            evt_loop,
            on_startup: None,
        };
        app.app_impl = Some(AppImpl::new(&app));
        app
    }

    pub fn enter_event_loop(mut self) -> i32 {
        let result = self.evt_loop.run_app(&mut self.app_impl.unwrap());
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

    pub(crate) fn get_env_loop(&self) -> &EventLoop<()> {
        &self.evt_loop
    }
}
