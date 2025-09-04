pub mod application;
// pub mod control;
pub mod id_manager;
pub mod paint;
mod slot_handle;
pub mod ui_control;
pub mod window;

pub use global_hotkey;
pub use sdl3;
pub use sdl3::keyboard::Keycode;

pub use ui_control as control;
