use std::any::TypeId;

use crate::ui_control::control::{Control, WeakHandle};

pub mod win_init;

pub trait Event: 'static {
    fn name(&self) -> &str;
    fn timestamp(&self) -> u32;
    fn event_type(&self) -> EventType;

    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

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
