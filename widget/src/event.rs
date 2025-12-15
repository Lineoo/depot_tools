use std::any::TypeId;

use crate::ui_control::control::{Control, WeakHandle};

pub mod win_init;

pub trait Event {
    fn name(&self) -> &str;
    fn type_id(&self) -> TypeId;
    fn timestamp(&self) -> u32;
    fn event_type(&self) -> EventType;
    fn sender(&self) -> WeakHandle<dyn Control> {
        WeakHandle::default()
    }
}

pub enum EventType {
    Bubble,
    Tunnel,
}

pub enum SysEvent {
    WinInit(win_init::WinInitEvent),
    Input(),
}
