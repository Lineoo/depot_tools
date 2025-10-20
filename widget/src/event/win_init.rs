use std::{cell::RefCell, rc::Weak};

use crate::{
    event::{Event, EventType},
    window::Window,
};

pub struct WinInitEvent {
    pub win_id: u32,
    pub win: Weak<RefCell<Window>>,
}

impl WinInitEvent {
    pub(crate) fn new(win_id: u32, win: Weak<RefCell<Window>>) -> Self {
        WinInitEvent { win_id, win }
    }

    pub fn win_id(&self) -> u32 {
        self.win_id
    }

    pub fn win(&self) -> Weak<RefCell<Window>> {
        self.win.clone()
    }
}

impl Event for WinInitEvent {
    fn name(&self) -> &str {
        "WinInitEvent"
    }

    fn timestamp(&self) -> u32 {
        0
    }

    fn event_type(&self) -> crate::event::EventType {
        EventType::Tunnel
    }
}
