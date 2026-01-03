use std::{
    any::{Any, TypeId},
    collections::HashSet,
};

use crate::control::{Control, ControlCapability, WeakHandle};

pub mod control;
pub mod edit;
pub mod focus;
pub mod keyboard;
pub mod mouse;
pub mod win_init;
pub mod window;

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

impl dyn Event {
    pub fn downcast_ref<T: Event>(&self) -> Option<&T> {
        if self.get_type_id() == TypeId::of::<T>() {
            Some(unsafe { &*(self as *const dyn Event as *const T) })
        } else {
            None
        }
    }

    pub fn downcast_mut<T: Event>(&mut self) -> Option<&mut T> {
        if self.get_type_id() == TypeId::of::<T>() {
            Some(unsafe { &mut *(self as *mut dyn Event as *mut T) })
        } else {
            None
        }
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
