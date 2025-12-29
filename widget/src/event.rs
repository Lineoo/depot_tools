use std::{any::TypeId, collections::HashSet};

use crate::ui_control::control::{Control, ControlCapability, WeakHandle};

pub mod edit;
pub mod win_init;

pub trait Event: 'static {
    fn name(&self) -> &str;
    fn timestamp(&self) -> u64;
    fn event_type(&self) -> EventType;

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn sender(&self) -> WeakHandle<dyn Control> {
        WeakHandle::default()
    }

    fn required_capabilities(&self) -> HashSet<ControlCapability> {
        HashSet::new()
    }
}

pub enum EventType {
    Bubble,
    Tunnel,
    Direct,
}

pub enum SysEvent {
    WinInit(win_init::WinInitEvent),
    Input(),
}
