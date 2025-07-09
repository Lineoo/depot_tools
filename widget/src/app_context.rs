use winit::window::WindowAttributes;

use crate::{application::Application, window::Window};

pub struct AppContext<'a, 'e> {
    pub(crate) active_evt_loop: &'e winit::event_loop::ActiveEventLoop,
    pub app: &'a mut Application<'a>,
}

impl<'a, 'e> AppContext<'a, 'e> {
    pub(crate) fn new(
        active_evt_loop: &'e winit::event_loop::ActiveEventLoop,
        app: &'a mut Application<'a>,
    ) -> Self {
        AppContext {
            active_evt_loop,
            app,
        }
    }

    pub fn create_window(&self, title: String) -> Window {
        Window::from_native_win(
            self.active_evt_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title(title)
                        .with_visible(false),
                )
                .unwrap(),
        )
    }
}
