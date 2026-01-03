use std::collections::HashSet;

use crate::{
    control::ControlCapability,
    event::{Event, EventType},
};

pub struct MouseEvent {
    pub event_type: MouseEventType,
    pub timestamp: u64,
}

pub enum MouseEventType {
    Move(u32, u32),
    Down,
    Up,
    Wheel,
}

impl Event for MouseEvent {
    fn name(&self) -> &str {
        "MouseEvent"
    }

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn event_type(&self) -> EventType {
        EventType::Tunnel
    }

    fn required_capabilities(&self) -> HashSet<ControlCapability> {
        [ControlCapability::Focus].iter().cloned().collect()
    }
}
