use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    option::Option,
    sync::{Arc, Mutex, Weak},
};
use winit::window::WindowAttributes;

use crate::{application::Application, loop_args::LoopArgs};

pub struct Window {
    win: winit::window::Window,
    pub on_size: Option<fn((i32, i32))>,
    pub before_close: Option<fn() -> bool>,
    pub on_paint: Option<fn()>,
    pub on_key_down: Option<fn(u32)>, // u32 is the virtual key code
    pub on_key_up: Option<fn(u32)>,   // key code
    pub on_mouse_down: Option<fn(u32, i32, i32)>, // mouse button, x, y
    pub on_mouse_up: Option<fn(u32, i32, i32)>, // mouse button, x, y
    pub on_mouse_move: Option<fn(i32, i32)>, // x, y
}

lazy_static! {
    static ref WINDOW_MAP: Mutex<HashMap<winit::window::WindowId, Weak<Window>>> =
        Mutex::new(HashMap::new());
}

impl Window {
    pub fn new(args: &LoopArgs, title: String) -> Arc<Self> {
        let w = Arc::new(Window {
            win: args
                .active_evt_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title(title)
                        .with_visible(false),
                )
                .unwrap(),

            on_size: None,
            before_close: None,
            on_paint: None,
            on_key_down: None,
            on_key_up: None,
            on_mouse_down: None,
            on_mouse_up: None,
            on_mouse_move: None,
        });
        let mut m = WINDOW_MAP.lock().unwrap();
        (*m).insert(w.win.id(), Arc::downgrade(&w));
        w.clone()
    }

    pub fn from_id(id: winit::window::WindowId) -> Option<Arc<Self>> {
        let m = WINDOW_MAP.lock().unwrap();
        match (*m).get(&id) {
            Some(w) => w.upgrade(),
            None => None,
        }
    }

    pub fn show(&self, visible: bool) {
        self.win.set_visible(visible);
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        let mut m = WINDOW_MAP.lock().unwrap();
        (*m).remove(&self.win.id());
    }
}
