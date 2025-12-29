use std::{
    any::Any,
    cell::RefCell,
    collections::{HashMap, HashSet, LinkedList},
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::{
    application::IdType,
    event::{Event, win_init::WinInitEvent},
    paint::{painter::Painter, shapes::Rect},
    ui_control::{
        control::{Control, WeakHandle},
        font::FontMgr,
        util::focus_mgr::FocusMgr,
    },
};
use crate::{id_manager::IdManager, window::win_strategy::*};
use global_hotkey::{GlobalHotKeyManager, hotkey::HotKey};
use sdl3::{
    keyboard::TextInputUtil,
    pixels::Color,
    render::{FRect, TextureCreator, WindowCanvas},
    ttf,
    video::{Window as SdlWindow, WindowContext},
};
use thiserror::Error;

pub struct Window {
    pub cvs: WindowCanvas,
    texture_creator: Rc<RefCell<TextureCreator<WindowContext>>>,
    font_mgr: Rc<RefCell<FontMgr>>,
    hotkey_manager: Rc<RefCell<(GlobalHotKeyManager, HashMap<u32, u32>)>>,
    hotkeys: HashSet<HotKey>,
    id: IdType,
    id_mgr: Rc<RefCell<IdManager>>,
    userdata: Option<Box<dyn Any>>,
    input_util: Rc<RefCell<TextInputUtil>>,

    focus_mgr: FocusMgr,
    event_queue: LinkedList<Box<dyn Event>>,

    child: Option<WeakHandle<dyn Control>>,
}

impl Window {
    pub(crate) fn new(
        win: SdlWindow,
        hotkey_manager: Rc<RefCell<(GlobalHotKeyManager, HashMap<u32, u32>)>>,
        id_mgr: Rc<RefCell<IdManager>>,
        input_util: Rc<RefCell<TextInputUtil>>,
        focus_mgr: FocusMgr,
        font_mgr: Rc<RefCell<FontMgr>>,
    ) -> Self {
        let id = id_mgr.borrow_mut().get_id();
        let cvs = win.into_canvas();
        let texture_creator = Rc::new(RefCell::new(cvs.texture_creator()));
        Window {
            cvs,
            texture_creator,
            font_mgr,
            hotkey_manager,
            hotkeys: HashSet::new(),
            id,
            id_mgr,
            userdata: None,
            input_util,
            focus_mgr,
            event_queue: LinkedList::new(),
            child: None,
        }
    }

    pub fn init(&mut self, event: WinInitEvent) {
        todo!()
    }

    pub fn no_decorations(&mut self) {
        // todo!();
        // let handle = self.cvs.window_mut().
        self.cvs.window_mut().set_bordered(false);
    }

    pub(crate) fn get_id(&self) -> u32 {
        self.cvs.window().id()
    }

    pub(crate) fn get_id_mgr(&self) -> Rc<RefCell<IdManager>> {
        self.id_mgr.clone()
    }

    pub(crate) fn paint(&mut self) {
        let mut p = Painter::new(
            self.cvs.window().size(),
            self.texture_creator.clone(),
            self.font_mgr.clone(),
        );
        if let Some(child) = &self.child
            && let Some(child) = child.upgrade()
        {
            let mut child = child.borrow_mut();
            let (width, height) = self.cvs.window().size();
            child.set_size(width, height);
            child.paint(&mut p);
        }
        self.cvs.clear();
        let (w, h) = self.cvs.window().size();
        let rect = FRect::new(0.0, 0.0, w as f32, h as f32);
        self.cvs.copy(
            &p.canvas
                .into_surface()
                .as_texture(self.texture_creator.borrow().deref())
                .unwrap(),
            Some(rect),
            Some(rect),
        );
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

    pub fn start_input_at(&self, area: Rect) {
        let input_util = self.input_util.borrow();
        input_util.set_rect(self.cvs.window(), area.try_into().unwrap(), 10);
        input_util.start(self.cvs.window());
    }

    pub fn end_input(&self) {
        let input_util = self.input_util.borrow();
        input_util.stop(self.cvs.window());
    }

    pub fn push_event<Evt: Event + 'static>(&mut self, event: Evt) {
        self.event_queue.push_back(Box::new(event));
    }

    pub(crate) fn distribute_event_one(&mut self) -> Result<bool, WindowEventDistributeError> {
        if self.event_queue.is_empty() {
            return Ok(false);
        }
        let event = self.event_queue.pop_front().unwrap();
        let id;
        if let Some(child) = self.child.as_ref()
            && let Some(child) = child.upgrade()
        {
            if let Some(child_ref) = child.try_borrow() {
                if !child_ref.receives_event(event.get_type_id()) {
                    return Ok(true);
                }
                id = child_ref.id();
                // continue to distribute event
            } else {
                return Err(WindowEventDistributeError::ControlBorrowError);
            }
        } else {
            return Ok(false);
        }

        // distribute event
        if let Some(child) = self.child.as_ref()
            && let Some(child) = child.upgrade()
        {
            if let Some(mut child_ref) = child.try_borrow_mut()
                && event
                    .required_capabilities()
                    .iter()
                    .all(|c| child_ref.query_capability(*c))
            {
                child_ref.process_event(event);
                Ok(true)
            } else {
                Err(WindowEventDistributeError::ControlMutablyBorrowError(id))
            }
        } else {
            Ok(false)
        }
    }

    pub(crate) fn distribute_event_some(
        &mut self,
        count: usize,
    ) -> Result<usize, WindowEventDistributeError> {
        let mut actual_count = 0;
        while actual_count != count && self.distribute_event_one()? {
            actual_count += 1;
        }
        Ok(actual_count)
    }

    pub(crate) fn distribute_event_all(&mut self) -> Result<usize, WindowEventDistributeError> {
        self.distribute_event_some(self.event_queue.len())
    }

    pub fn event_count(&self) -> usize {
        self.event_queue.len()
    }

    pub fn size(&self) -> (u32, u32) {
        self.cvs.window().size()
    }

    pub fn set_child(&mut self, child: WeakHandle<dyn Control>) {
        self.child = Some(child);
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

    pub unsafe fn raw(&self) -> &SdlWindow {
        self.cvs.window()
    }

    pub fn get_active_control(&self) -> Option<WeakHandle<dyn Control>> {
        self.focus_mgr.current()
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
type SlotFn = Box<dyn FnMut(&mut Window, Option<Box<dyn Any>>)>;

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
        F: FnMut(&mut Window, Option<Box<dyn Any>>) + 'static,
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
            slot(&mut self.win, Some(Box::new(arg)));
            Ok(())
        } else {
            Err(WindowSlotError::TargetSlotNotExist(name.to_string()))
        }
    }

    pub(crate) fn call_slot_option<Arg: Any>(
        &mut self,
        name: &str,
        arg: Option<Arg>,
    ) -> Result<(), WindowSlotError> {
        if let Some(slot) = self.slots.get_mut(name) {
            slot(
                &mut self.win,
                if let Some(arg) = arg {
                    Some(Box::new(arg))
                } else {
                    None
                },
            );
            Ok(())
        } else {
            Err(WindowSlotError::TargetSlotNotExist(name.to_string()))
        }
    }
}

impl Deref for WindowDirector {
    type Target = Window;

    fn deref(&self) -> &Self::Target {
        &self.win
    }
}

impl DerefMut for WindowDirector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.win
    }
}

#[derive(Error, Debug)]
pub enum WindowEventDistributeError {
    #[error("Could not borrow child control")]
    ControlBorrowError,
    #[error("Could not mutably borrow child control, id: {0}")]
    ControlMutablyBorrowError(IdType),
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
