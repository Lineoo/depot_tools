use crate::event::{Event, EventType};

pub struct CtrlResizeEvent {
    pub win_id: u32,
    pub new_size: (u32, u32),
    pub timestamp: u64,
}
impl Event for CtrlResizeEvent {
    fn name(&self) -> &str {
        "CtrlResizeEvent"
    }

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn event_type(&self) -> EventType {
        EventType::Tunnel
    }
}
