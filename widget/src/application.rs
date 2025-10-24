//! Application module for managing windows and global hotkeys
//!
//! `Application` manages the whole application lifecycle, including
//! window creation, event handling, and global hotkey registration.
//!
//! Every application should have a single instance of `Application`,
//! but more instances are not prohibited.

use crate::id_manager::IdManager;
use crate::ui_control::ctrl_ctx::CtrlCtx;
use crate::ui_control::ctrl_mgr::{CtrlMgr, make_ctrl_ctx};
use crate::ui_control::font_mgr::FontMgr;
use crate::ui_control::util::text_edit;
use crate::window::WindowStrategyError;
use crate::window::{
    Window, WindowDirector,
    win_strategy::{CloseStrategy, MinimizeStrategy, WindowStrategy},
};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use sdl3::keyboard::TextInputUtil;
use sdl3::ttf::Sdl3TtfContext;
use sdl3::{
    Sdl, VideoSubsystem,
    event::{Event, WindowEvent},
    keyboard::Keycode,
};
use std::num::NonZero;
use std::rc::{Rc, Weak};
use std::{cell::RefCell, collections::HashMap, time::Duration};

pub type IdType = NonZero<u64>;

pub struct Application {
    // SDL context and video subsystem
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    ttf_ctx: Rc<RefCell<Sdl3TtfContext>>,
    pub input_util: Rc<RefCell<TextInputUtil>>,

    // HashMap to store windows by their IDs
    pub(crate) wins: HashMap<u32, Rc<RefCell<WindowDirector>>>,
    pub(crate) controls: Rc<RefCell<CtrlMgr>>,
    // Global hotkey manager and a map to associate hotkeys with window IDs
    hotkey_mgr: Rc<RefCell<(GlobalHotKeyManager, HashMap<u32, u32>)>>,

    // ID manager for generating unique IDs for controls
    id_mgr: Rc<RefCell<IdManager>>,
    ctrl_ctx: Rc<CtrlCtx>,

    pub(crate) active_text_edit: Rc<RefCell<Option<Weak<RefCell<text_edit::TextEdit>>>>>,
}

impl Application {
    pub fn new() -> Self {
        let sdl_context = sdl3::init().expect("Failed to initialize SDL");
        let video_subsystem = sdl_context
            .video()
            .expect("Failed to initialize video subsystem");
        let ttf_ctx = Rc::new(RefCell::new(
            sdl3::ttf::init().expect("Failed to initialize TTF context"),
        ));
        let input_util = Rc::new(RefCell::new(video_subsystem.text_input()));
        let id_mgr = Rc::new(RefCell::new(IdManager::new()));
        let controls = Rc::new(RefCell::new(CtrlMgr::new()));
        let active_text_edit = Rc::new(RefCell::new(None));
        let ctrl_ctx = Rc::new(make_ctrl_ctx(
            id_mgr.clone(),
            controls.clone(),
            Rc::new(RefCell::new(FontMgr::new(ttf_ctx.clone()))),
            active_text_edit.clone(),
        ));
        Application {
            sdl_context,
            video_subsystem,
            ttf_ctx,
            input_util,
            wins: HashMap::new(),
            controls,
            hotkey_mgr: Rc::new(RefCell::new((
                GlobalHotKeyManager::new().expect("Failed to create hotkey manager"),
                HashMap::new(),
            ))),
            id_mgr,
            ctrl_ctx,
            active_text_edit,
        }
    }

    pub fn make_window(&mut self, title: &str, width: u32, height: u32) -> WindowDirector {
        WindowDirector::new(
            Window::new(
                self.video_subsystem
                    .window(title, width, height)
                    .build()
                    .expect("Failed to create window"),
                self.hotkey_mgr.clone(),
                self.get_id_mgr(),
                self.ttf_ctx.clone(),
                self.input_util.clone(),
            ),
            HashMap::new(),
        )
    }

    pub fn reg_win(&mut self, win: WindowDirector) {
        self.wins
            .insert(win.get_win().get_id(), Rc::new(RefCell::new(win)));
    }

    pub fn apply_control_id(&mut self) -> IdType {
        self.id_mgr.borrow_mut().get_id()
    }

    pub fn drop_control_id(&mut self, id: IdType) {
        self.id_mgr.borrow_mut().release_id(id)
    }

    pub(crate) fn get_id_mgr(&self) -> Rc<RefCell<IdManager>> {
        self.id_mgr.clone()
    }

    pub fn ctrl_ctx(&self) -> Rc<CtrlCtx> {
        self.ctrl_ctx.clone()
    }

    pub fn run(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'event_loop: loop {
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
                        let result = self
                            .wins
                            .get_mut(&window_id)
                            .unwrap()
                            .borrow_mut()
                            .call_strategy("close_requested");
                        if let Ok(WindowStrategy::Close(CloseStrategy::Close))
                        | Err(WindowStrategyError::StrategyNotSet(..)) = result
                        {
                            self.wins.remove(&window_id);
                        }
                        {}
                    }
                    Event::Window {
                        window_id,
                        win_event: WindowEvent::Minimized,
                        ..
                    } => {
                        let win = self.wins.get_mut(&window_id).unwrap();
                        if let Ok(WindowStrategy::Minimize(MinimizeStrategy::Minimize)) =
                            win.borrow_mut().call_strategy("minimize")
                        {
                            win.borrow_mut().minimize();
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Q),
                        ..
                    } => break 'event_loop,
                    Event::KeyDown {
                        keycode: Some(Keycode::W),
                        window_id,
                        ..
                    } => {
                        self.wins.remove(&window_id);
                    }
                    Event::KeyDown {
                        window_id, keycode, ..
                    } => {
                        // wins.get_mut(&window_id)
                        //     .unwrap()
                        //     .call_slot_option("keydown", keycode)
                        //     .unwrap();
                    }
                    _ => {}
                }
            }
            for win in self.wins.values_mut() {
                win.borrow_mut().paint(); // Call paint on each registered window
            }

            if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
                let hm = self.hotkey_mgr.borrow_mut();
                let win = self.wins.get_mut(hm.1.get(&event.id).unwrap()).unwrap();
                let _ = win.borrow_mut().call_slot("hotkey", event.id);
            }

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
