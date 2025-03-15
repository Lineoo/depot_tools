use crate::app_impl::AppImpl;
use winit::{error::EventLoopError::*, event_loop::EventLoop};

pub struct Application {
    app_impl: AppImpl,
    evt_loop: EventLoop<()>,
}

impl Application {
    pub fn new() -> Self {
        Application {
            app_impl: AppImpl::new(),
            evt_loop: EventLoop::new().unwrap(),
        }
    }

    pub fn enter_event_loop(mut self) -> i32 {
        let result = self.evt_loop.run_app(&mut self.app_impl);
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
