//! Application module for managing windows and global hotkeys
//!
//! `Application` manages the whole application lifecycle, including
//! window creation, event handling, and global hotkey registration.
//!
//! Every application should have a single instance of `Application`,
//! but more instances are not prohibited.

use crate::control::ctrl_ctx::CtrlCtx;
use crate::control::ctrl_mgr::{CtrlMgr, make_ctrl_ctx};
use crate::control::font::{DefaultFamily, FontMgr};
use crate::control::util::focus_mgr::FocusMgr;
use crate::control::util::text_edit;
use crate::control::{Control, ControlCapability, Handle};
use crate::event::control::CtrlResizeEvent;
use crate::event::edit::{ImEditEvent, TextEditEvent};
use crate::event::focus::GainFocusEvent;
use crate::event::keyboard::{KeyState, KeyboardEvent};
use crate::event::win_init::WinInitEvent;
use crate::event::window::WinResizeEvent;
use crate::id_manager::IdManager;
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
use std::sync::{Arc, Mutex};
use std::{cell::RefCell, collections::HashMap, time::Duration};

pub type IdType = NonZero<u64>;

pub struct Application {
    // SDL context and video subsystem
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    pub input_util: Rc<RefCell<TextInputUtil>>,

    // HashMap to store windows by their IDs
    pub(crate) wins: HashMap<u32, Rc<RefCell<WindowDirector>>>,
    pub(crate) controls: Rc<RefCell<CtrlMgr>>,
    // Global hotkey manager and a map to associate hotkeys with window IDs
    hotkey_mgr: Rc<RefCell<(GlobalHotKeyManager, HashMap<u32, u32>)>>,

    // ID manager for generating unique IDs for controls
    id_mgr: Rc<RefCell<IdManager>>,
    ctrl_ctx: Rc<CtrlCtx>,
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
        let ctrl_ctx = Rc::new(make_ctrl_ctx(
            id_mgr.clone(),
            controls.clone(),
            Rc::new(RefCell::new(default_font_mgr().unwrap())),
            Arc::new(Mutex::new(FocusMgr::new())),
            input_util.clone(),
        ));
        Application {
            sdl_context,
            video_subsystem,
            input_util,
            wins: HashMap::new(),
            controls,
            hotkey_mgr: Rc::new(RefCell::new((
                GlobalHotKeyManager::new().expect("Failed to create hotkey manager"),
                HashMap::new(),
            ))),
            id_mgr,
            ctrl_ctx,
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
                self.input_util.clone(),
                FocusMgr::new(),
                self.ctrl_ctx.font_mgr(),
            ),
            HashMap::new(),
        )
    }

    pub fn reg_win(&mut self, mut win: WindowDirector) {
        let control = win.get_child();
        let id = win.get_id();
        self.wins
            .insert(win.get_win().get_id(), Rc::new(RefCell::new(win)));
        if let Some(control) = control.upgrade()
            && let Some(mut control) = control.try_borrow_mut()
        {
            control.attach_window(Rc::downgrade(self.wins.get(&id).unwrap()));
        }
        let child;
        self.wins.get(&id).unwrap().borrow_mut().init();
        if let Some(ctrl) = self.wins.get(&id).unwrap().borrow_mut().focus_mgr.current()
            && let Some(ctrl) = ctrl.upgrade()
        {
            child = Some(ctrl.clone());
        } else {
            child = None;
        }
        if let Some(child) = child
            && let Some(mut child) = child.try_borrow_mut()
        {
            child.process_event(Box::new(GainFocusEvent));
        }
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
                    Event::Window {
                        window_id,
                        win_event: WindowEvent::Resized(w, h),
                        timestamp,
                    } => {
                        if let Some(child) =
                            get_win_child(self.wins.get(&window_id).unwrap().clone())
                            && let Some(mut child) = child.try_borrow_mut()
                        {
                            child.process_event(Box::new(WinResizeEvent {
                                win_id: window_id,
                                new_size: (w as u32, h as u32),
                                timestamp,
                            }));
                            child.process_event(Box::new(CtrlResizeEvent {
                                win_id: window_id,
                                new_size: (w as u32, h as u32),
                                timestamp,
                            }));
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::F4),
                        ..
                    } => break 'event_loop,
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        window_id,
                        ..
                    } => {
                        self.wins.remove(&window_id);
                    }
                    Event::KeyDown {
                        window_id,
                        keycode,
                        timestamp,
                        ..
                    } => {
                        if let Some(child) =
                            get_win_child(self.wins.get(&window_id).unwrap().clone())
                            && let Some(mut child) = child.try_borrow_mut()
                            && let Some(keycode) = keycode
                        {
                            child.process_event(Box::new(KeyboardEvent {
                                keycode,
                                state: KeyState::Pressed,
                                timestamp,
                            }));
                        }
                    }
                    Event::KeyUp {
                        window_id,
                        keycode,
                        timestamp,
                        ..
                    } => {
                        if let Some(child) =
                            get_win_child(self.wins.get(&window_id).unwrap().clone())
                            && let Some(mut child) = child.try_borrow_mut()
                            && let Some(keycode) = keycode
                        {
                            child.process_event(Box::new(KeyboardEvent {
                                keycode,
                                state: KeyState::Released,
                                timestamp,
                            }));
                        }
                    }
                    Event::TextEditing {
                        window_id,
                        text,
                        start,
                        length,
                        timestamp,
                    } => {
                        let text_edit_enabled = if let Some(active_control) = self
                            .wins
                            .get(&window_id)
                            .unwrap()
                            .borrow()
                            .get_active_control()
                            .clone()
                            && active_control
                                .upgrade()
                                .unwrap()
                                .borrow()
                                .query_capability(ControlCapability::TextEdit)
                        {
                            true
                        } else {
                            false
                        };
                        if text_edit_enabled {
                            self.wins[&window_id]
                                .borrow_mut()
                                .queue_event(ImEditEvent::new(
                                    text,
                                    start as usize,
                                    length as usize,
                                    timestamp,
                                ));
                        }
                    }
                    Event::TextInput {
                        window_id,
                        text,
                        timestamp,
                    } => {
                        let text_edit_enabled = if let Some(active_control) = self
                            .wins
                            .get(&window_id)
                            .unwrap()
                            .borrow()
                            .get_active_control()
                            && active_control
                                .upgrade()
                                .unwrap()
                                .borrow()
                                .query_capability(ControlCapability::TextEdit)
                        {
                            true
                        } else {
                            false
                        };
                        if text_edit_enabled {
                            self.wins[&window_id]
                                .borrow_mut()
                                .queue_event(TextEditEvent::new(text, timestamp));
                        }
                    }
                    _ => {}
                }
            }
            for win in self.wins.values_mut() {
                let mut ww = win.borrow_mut();
                ww.paint(); // Call paint on each registered window
                if ww.distribute_event_all().is_err() {
                    return; // TODO: Handle errors
                }
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

fn default_font_mgr() -> Option<FontMgr> {
    Some(
        FontMgr::new()?, // .load_default_family(DefaultFamily::SansSerif)
                         // .load_default_family(DefaultFamily::Serif)
                         // .load_default_family(DefaultFamily::Monospace)
                         // .load_default_family(DefaultFamily::UiFont)
                         // .load_default_family(DefaultFamily::CodeFont)
                         // .load_default_family(DefaultFamily::TerminalFont),
    )
}

/// Get child control and keep the window not borrowed
///
/// # Notes
/// - This function will fail if the window is borrowed
fn get_win_child(win: Rc<RefCell<WindowDirector>>) -> Option<Handle<dyn Control>> {
    if let Ok(win) = win.try_borrow()
        && let Some(ctrl) = win.get_child().upgrade()
    {
        Some(ctrl.clone())
    } else {
        None
    }
}
