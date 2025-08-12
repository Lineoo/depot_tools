use std::{
    any::Any,
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::application::{IdType, TextMgr};
use crate::window::win_strategy::*;
use cosmic_text::{Buffer, Metrics};
use global_hotkey::{GlobalHotKeyManager, hotkey::HotKey};
use sdl3::render::WindowCanvas;
use thiserror::Error;

pub struct Window {
    pub(crate) cvs: WindowCanvas,
    hotkey_manager: Rc<RefCell<(GlobalHotKeyManager, HashMap<u32, u32>)>>,
    hotkeys: HashSet<HotKey>,
    id: IdType,
    userdata: Option<Box<dyn Any>>,
}

impl Window {
    pub(crate) fn new(
        win: sdl3::video::Window,
        hotkey_manager: Rc<RefCell<(GlobalHotKeyManager, HashMap<u32, u32>)>>,
        id: IdType,
        text_mgr: Rc<RefCell<TextMgr>>,
    ) -> Self {
        let mut text_mgr_ref = text_mgr.borrow_mut();
        let metrics = Metrics::new(24.0, 20.0);
        let mut buffer = Buffer::new(&mut text_mgr_ref.font_system, metrics);
        let mut buffer = buffer.borrow_with(&mut text_mgr_ref.font_system);

        Window {
            cvs: win.into_canvas(),
            hotkey_manager,
            hotkeys: HashSet::new(),
            id,
            userdata: None,
        }
    }

    pub(crate) fn get_id(&self) -> u32 {
        self.cvs.window().id()
    }

    pub(crate) fn paint(&mut self) {
        self.cvs
            .set_draw_color(sdl3::pixels::Color::RGB(255, 252, 241));
        self.cvs.clear();

        self.cvs.set_draw_color(sdl3::pixels::Color::RGB(0, 0, 0));
        self.cvs
            .fill_rect(sdl3::rect::Rect::new(5, 5, 400, 30))
            .expect("Failed to fill rectangle");

        self.cvs.present();
    }

    pub fn show(&mut self) {
        self.cvs.window_mut().show();
    }

    pub fn hide(&mut self) {
        self.cvs.window_mut().hide();
    }

    pub fn minimize(&mut self) {
        self.cvs.window_mut().minimize();
    }

    pub fn maximize(&mut self) {
        self.cvs.window_mut().maximize();
    }

    pub fn normalize(&mut self) {
        self.cvs.window_mut().restore();
    }

    pub fn reg_hotkey(&mut self, hotkey: HotKey) -> Result<(), global_hotkey::Error> {
        let mut manager = self.hotkey_manager.borrow_mut();
        manager.0.register(hotkey)?;
        manager.1.insert(hotkey.id, self.get_id());
        self.hotkeys.insert(hotkey);
        Ok(())
    }

    pub fn set_userdata<T: Any>(&mut self, data: T) {
        self.userdata = Some(Box::new(data));
    }

    pub fn get_userdata<T: Any>(&self) -> Option<&T> {
        self.userdata.as_ref()?.downcast_ref::<T>()
    }

    pub fn get_userdata_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.userdata.as_mut()?.downcast_mut::<T>()
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        let mut manager = self.hotkey_manager.borrow_mut();
        for hotkey in &self.hotkeys {
            manager.0.unregister(*hotkey).unwrap();
            manager.1.remove(&hotkey.id);
        }
    }
}

type StrategyFn = Box<dyn FnMut(&mut Window) -> WindowStrategy>;
type SlotFn = Box<dyn FnMut(&mut Window, Box<dyn Any>)>;

pub struct WindowDirector {
    win: Window,
    pub(crate) strategy: HashMap<String, StrategyFn>,
    pub(crate) slots: HashMap<String, SlotFn>,
}

impl WindowDirector {
    pub(crate) fn new(win: Window, strategy: HashMap<String, StrategyFn>) -> Self {
        Self {
            win,
            strategy,
            slots: HashMap::new(),
        }
    }

    pub fn get_win(&self) -> &Window {
        &self.win
    }

    pub fn get_win_mut(&mut self) -> &mut Window {
        &mut self.win
    }

    pub fn set_strategy<F: FnMut(&mut Window) -> WindowStrategy + 'static>(
        &mut self,
        name: String,
        strategy: F,
    ) {
        self.strategy.insert(name, Box::new(strategy));
    }

    pub fn rm_strategy(&mut self, name: &str) {
        self.strategy.remove(name);
    }

    pub fn set_slot<F>(&mut self, name: String, slot: F)
    where
        F: FnMut(&mut Window, Box<dyn Any>) + 'static,
    {
        self.slots.insert(name, Box::new(slot));
    }

    pub fn rm_slot(&mut self, name: &str) {
        self.slots.remove(name);
    }

    pub(crate) fn call_strategy(
        &mut self,
        name: &str,
    ) -> Result<WindowStrategy, WindowStrategyError> {
        if let Some(strategy) = self.strategy.get_mut(name) {
            Ok(strategy(&mut self.win))
        } else {
            Err(WindowStrategyError::StrategyNotSet(name.to_string()))
        }
    }

    pub(crate) fn call_slot<Arg: Any>(
        &mut self,
        name: &str,
        arg: Arg,
    ) -> Result<(), WindowSlotError> {
        if let Some(slot) = self.slots.get_mut(name) {
            slot(&mut self.win, Box::new(arg));
            Ok(())
        } else {
            Err(WindowSlotError::TargetSlotNotExist(name.to_string()))
        }
    }
}

#[derive(Error, Debug)]
pub enum WindowStrategyError {
    #[error("Target strategy \'{0}\' does not exist")]
    TargetStrategyNotExist(String),
    #[error("Strategy \'{0}\' does not set")]
    StrategyNotSet(String),
}

#[derive(Error, Debug)]
pub enum WindowSlotError {
    #[error("Target slot \'{0}\' does not exist")]
    TargetSlotNotExist(String),
    #[error("Slot \'{0}\' does not set")]
    SlotNotSet(String),
}
