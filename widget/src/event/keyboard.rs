use sdl3::keyboard::Keycode;

use crate::event::Event;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyState {
    Pressed,
    Released,
}

pub struct KeyboardEvent {
    pub keycode: Keycode,
    pub state: KeyState,
    pub timestamp: u64,
}

impl Event for KeyboardEvent {
    fn name(&self) -> &str {
        "keyboardEvent"
    }

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn event_type(&self) -> super::EventType {
        super::EventType::Tunnel
    }
}
