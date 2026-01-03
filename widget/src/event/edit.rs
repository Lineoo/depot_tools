use std::collections::HashSet;

use crate::control::ControlCapability;

use super::{Event, EventType};

pub struct TextEditEvent {
    pub text: String,
    pub timestamp: u64,
}

impl TextEditEvent {
    pub fn new(text: String, timestamp: u64) -> Self {
        Self { text, timestamp }
    }
}

impl Event for TextEditEvent {
    fn name(&self) -> &str {
        "TextEditEvent"
    }

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn event_type(&self) -> EventType {
        EventType::Direct
    }

    fn required_capabilities(&self) -> HashSet<ControlCapability> {
        [ControlCapability::TextEdit].iter().cloned().collect()
    }
}

pub struct ImEditEvent {
    pub text: String,
    pub start: usize,
    pub length: usize,
    pub timestamp: u64,
}

impl ImEditEvent {
    pub fn new(text: String, start: usize, length: usize, timestamp: u64) -> Self {
        Self {
            timestamp,
            text,
            start,
            length,
        }
    }
}

impl Event for ImEditEvent {
    fn name(&self) -> &str {
        "ImEditEvent"
    }

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn event_type(&self) -> EventType {
        EventType::Direct
    }

    fn required_capabilities(&self) -> HashSet<ControlCapability> {
        [ControlCapability::TextEdit].iter().cloned().collect()
    }
}
