use crate::ui_control::control::{Control, WeakHandle};

pub mod win_init;

pub trait Event {
    fn name(&self) -> &str;
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
