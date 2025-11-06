use std::sync::{Arc, Mutex, Weak};

use crate::ui_control::control::{Control, WeakHandle};

struct FocusCtx {
    active_handle: Weak<Mutex<FocusHandleData>>,
}

pub struct FocusMgr {
    ctx: Arc<Mutex<FocusCtx>>,
}

impl FocusMgr {
    pub(crate) fn new() -> Self {
        Self {
            ctx: Arc::new(Mutex::new(FocusCtx {
                active_handle: Weak::new(),
            })),
        }
    }

    pub fn get_handle(&self) -> Arc<FocusHandle> {
        Arc::new(FocusHandle {
            data: Arc::new(Mutex::new(FocusHandleData {
                ctx: self.ctx.clone(),
                tab_stop: false,
                tab_index: None,
                is_focused: false,
                focus_inside: false,
                on_gain_focus: None,
                on_lose_focus: None,
                on_focus_inside: None,
                control: WeakHandle::new(),
            })),
            prev: None,
            next: None,
        })
    }
}

struct FocusHandleData {
    ctx: Arc<Mutex<FocusCtx>>,
    tab_stop: bool,
    tab_index: Option<u32>,
    is_focused: bool,
    focus_inside: bool,
    pub on_gain_focus: Option<Box<dyn FnMut()>>,
    pub on_lose_focus: Option<Box<dyn FnMut()>>,
    pub on_focus_inside: Option<Box<dyn FnMut()>>,
    pub control: WeakHandle<dyn Control>,
}

unsafe impl Send for FocusHandleData {}
unsafe impl Sync for FocusHandleData {}

pub struct FocusHandle {
    data: Arc<Mutex<FocusHandleData>>,
    prev: Option<Weak<Self>>,
    next: Option<Weak<Self>>,
}

unsafe impl Send for FocusHandle {}
unsafe impl Sync for FocusHandle {}

impl FocusHandle {
    pub fn gain_focus(&self) {
        let mut data = self.data.lock().unwrap();
        data.is_focused = true;
        data.ctx.lock().unwrap().active_handle = Arc::downgrade(&self.data);
        if let Some(ref mut on_gain_focus) = data.on_gain_focus {
            on_gain_focus();
        }
    }

    pub fn lose_focus(&self) {
        let mut data = self.data.lock().unwrap();
        data.is_focused = false;
        data.ctx.lock().unwrap().active_handle = Weak::new();
        if let Some(ref mut on_lose_focus) = data.on_lose_focus {
            on_lose_focus();
        }
    }

    fn lose_focus_unnoticed(&self) {
        let mut data = self.data.lock().unwrap();
        data.is_focused = false;
        if let Some(ref mut on_lose_focus) = data.on_lose_focus {
            on_lose_focus();
        }
    }

    pub fn guide_focus_inside(&self) {
        let mut data = self.data.lock().unwrap();
        data.is_focused = true;
        data.focus_inside = true;
        if let Some(ref mut on_focus_inside) = data.on_focus_inside {
            on_focus_inside();
        }
    }

    pub fn is_focused(&self) -> bool {
        self.data.lock().unwrap().is_focused
    }

    pub fn is_focus_inside(&self) -> bool {
        self.data.lock().unwrap().focus_inside
    }

    pub fn is_tab_stop(&self) -> bool {
        self.data.lock().unwrap().tab_stop
    }

    pub fn tab_index(&self) -> Option<u32> {
        self.data.lock().unwrap().tab_index
    }

    pub fn set_tab_stop(&mut self, will_stop: bool) {
        let mut data = self.data.lock().unwrap();
        data.tab_stop = will_stop;
        if !will_stop {
            data.tab_index = None;
        }
    }

    pub fn set_tab_index(&mut self, index: u32) {
        self.data.lock().unwrap().tab_index = Some(index);
    }

    pub fn next_handle(&mut self) -> Option<Weak<Self>> {
        self.next.clone()
    }

    pub fn prev_handle(&mut self) -> Option<Weak<Self>> {
        self.prev.clone()
    }

    pub fn set_next_handle(&mut self, next_handle: Weak<Self>) {
        self.next = Some(next_handle);
    }

    pub fn set_prev_handle(&mut self, prev_handle: Weak<Self>) {
        self.prev = Some(prev_handle);
    }

    pub fn focus_next(&self) {
        if let Some(next) = self.next.clone()
            && let Some(next) = next.upgrade()
        {
            self.lose_focus_unnoticed();
            next.gain_focus();
        } else {
            self.lose_focus();
        }
    }

    pub fn focus_prev(&self) {
        if let Some(prev) = self.prev.clone()
            && let Some(prev) = prev.upgrade()
        {
            self.lose_focus_unnoticed();
            prev.gain_focus();
        } else {
            self.lose_focus();
        }
    }

    pub fn set_control(&mut self, control: WeakHandle<dyn Control>) {
        self.data.lock().unwrap().control = control;
    }

    pub fn control(&self) -> WeakHandle<dyn Control> {
        self.data.lock().unwrap().control.clone()
    }
}
