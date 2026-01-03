use crate::event::{Event, EventType};

pub struct GainFocusEvent;

pub struct LoseFocusEvent;

impl Event for GainFocusEvent {
    fn name(&self) -> &str {
        "GainFocusEvent"
    }

    fn timestamp(&self) -> u64 {
        0
    }

    fn event_type(&self) -> EventType {
        EventType::Direct
    }
}

impl Event for LoseFocusEvent {
    fn name(&self) -> &str {
        "LoseFocusEvent"
    }

    fn timestamp(&self) -> u64 {
        0
    }

    fn event_type(&self) -> EventType {
        EventType::Direct
    }
}
