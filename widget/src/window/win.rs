use std::{cell::RefCell, collections::HashMap, rc::Rc};

use global_hotkey::{GlobalHotKeyManager, hotkey::HotKey};
use sdl3::render::WindowCanvas;
use thiserror::Error;

use crate::window::win_strategy::*;

pub struct Window {
    pub(crate) cvs: WindowCanvas,
    hotkey_manager: Rc<RefCell<(GlobalHotKeyManager, HashMap<u32, u32>)>>,
}

impl Window {
    pub(crate) fn new(
        win: sdl3::video::Window,
        hotkey_manager: Rc<RefCell<(GlobalHotKeyManager, HashMap<u32, u32>)>>,
    ) -> Self {
        Window {
            cvs: win.into_canvas(),
            hotkey_manager,
        }
    }

    pub(crate) fn get_id(&self) -> u32 {
        self.cvs.window().id()
    }

    pub(crate) fn paint(&mut self) {
        self.cvs
            .set_draw_color(sdl3::pixels::Color::RGB(255, 252, 241));
        self.cvs.clear();
        self.cvs.present();
    }

    pub fn show(&mut self) {
        self.cvs.window_mut().show();
    }

    pub fn hide(&mut self) {
        self.cvs.window_mut().hide();
    }

    pub fn reg_hotkey(&mut self, hotkey: HotKey) -> Result<(), global_hotkey::Error> {
        let mut manager = self.hotkey_manager.borrow_mut();
        manager.0.register(hotkey)?;
        manager.1.insert(hotkey.id, self.get_id());
        Ok(())
    }
}

pub struct WindowDirector {
    win: Window,
    pub(crate) strategy: HashMap<String, Box<dyn FnMut(&mut Window) -> WindowStrategy>>,
}

impl WindowDirector {
    pub(crate) fn new(
        win: Window,
        strategy: HashMap<String, Box<dyn FnMut(&mut Window) -> WindowStrategy>>,
    ) -> Self {
        Self { win, strategy }
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

    pub(crate) fn call_strategy(
        &mut self,
        name: &str,
    ) -> Result<WindowStrategy, WindowStrategyError> {
        if let Some(strategy) = self.strategy.get_mut(name) {
            Ok(strategy(&mut self.win))
        } else {
            Err(WindowStrategyError::StrategyNotSet)
        }
    }
}

#[derive(Error, Debug)]
pub enum WindowStrategyError {
    #[error("Target strategy does not exist")]
    TargetStrategyNotExist,
    #[error("Strategy does not set")]
    StrategyNotSet,
}
