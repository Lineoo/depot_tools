use crate::event::{Event, EventType};

pub struct WinResizeEvent {
    pub win_id: u32,
    pub new_size: (u32, u32),
    pub timestamp: u64,
}
impl Event for WinResizeEvent {
    fn name(&self) -> &str {
        "WinResizeEvent"
    }

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn event_type(&self) -> EventType {
        EventType::Tunnel
    }
}
