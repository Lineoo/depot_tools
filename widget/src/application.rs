//! Application module for managing windows and global hotkeys
//!
//! `Application` manages the whole application lifecycle, including
//! window creation, event handling, and global hotkey registration.
//!
//! Every application should have a single instance of `Application`,
//! but more instances are not prohibited.

use std::{cell::RefCell, collections::HashMap, rc::Rc, time::Duration};

use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager,
    hotkey::{Code, HotKey, Modifiers},
};
use sdl3::{
    Sdl, VideoSubsystem,
    event::{Event, WindowEvent},
    keyboard::Keycode,
};

use crate::window::{
    Window, WindowDirector,
    win_strategy::{CloseStrategy, WindowStrategy},
};

pub struct Application {
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    wins: HashMap<u32, WindowDirector>,
    hotkey_manager: Rc<RefCell<(GlobalHotKeyManager, HashMap<u32, u32>)>>,
}

impl Application {
    pub fn new() -> Self {
        let sdl_context = sdl3::init().expect("Failed to initialize SDL");
        let video_subsystem = sdl_context
            .video()
            .expect("Failed to initialize video subsystem");
        Application {
            sdl_context,
            video_subsystem,
            wins: HashMap::new(),
            hotkey_manager: Rc::new(RefCell::new((
                GlobalHotKeyManager::new().expect("Failed to create hotkey manager"),
                HashMap::new(),
            ))),
        }
    }

    pub fn make_window(&self, title: &str, width: u32, height: u32) -> WindowDirector {
        WindowDirector::new(
            Window::new(
                self.video_subsystem
                    .window(title, width, height)
                    .build()
                    .expect("Failed to create window"),
                self.hotkey_manager.clone(),
            ),
            HashMap::new(),
        )
    }

    pub fn reg_win(&mut self, win: WindowDirector) {
        // Register the window in the application context if needed
        // This could involve storing it in a collection or similar
        self.wins.insert(win.get_win().get_id(), win);
    }

    pub fn run(&mut self) {
        // let hotkey = HotKey::new(Some(Modifiers::CONTROL), Code::Space);
        // let r = manager.register(hotkey);
        // r.expect("Failed to register global hotkey");

        // Application logic goes here
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'event_loop: loop {
            for win in self.wins.values_mut() {
                win.get_win_mut().paint(); // Call paint on each registered window
            }
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        if self.wins.is_empty() {
                            break 'event_loop;
                        }
                    }
                    Event::Window {
                        window_id,
                        win_event: WindowEvent::CloseRequested,
                        ..
                    } => {
                        println!("close called");
                        let win = self.wins.get_mut(&window_id).unwrap();
                        if let Ok(WindowStrategy::Close(CloseStrategy::Close)) =
                            win.call_strategy("close_requested")
                        {
                            self.wins.remove(&window_id);
                            println!("Window {} closed", window_id);
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Q),
                        ..
                    } => break 'event_loop,
                    _ => {}
                }
            }

            if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
                let hm = self.hotkey_manager.borrow_mut();
                let win = self.wins.get_mut(hm.1.get(&event.id).unwrap()).unwrap();
                let _ = win.call_strategy("hotkey");
            }

            // if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            //     self.wins[0].cvs.window_mut().show();
            // }
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
