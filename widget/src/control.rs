use std::any::Any;

use sdl3::render::{Canvas, RenderTarget};

use crate::application::IdType;

pub trait Control {
    fn get_id(&self) -> IdType;

    fn parent(&self) -> Option<IdType>;
}

pub trait ParentCtrl: Control {
    fn child(&self) -> Option<IdType>;
}

pub trait VisualControl: Control {
    fn draw<T: RenderTarget>(&mut self, canvas: &mut Canvas<T>);
}

pub trait Eventful: Control {
    fn raise_signal(&mut self, name: String, direction: EventDirection);
    fn raise_command<T: Any>(&mut self, name: String, data: T, direction: EventDirection);

    fn receive_signal(&mut self, name: String, direction: EventDirection);
    fn receive_command<T: Any>(&mut self, name: String, data: T, direction: EventDirection);
}

pub enum EventDirection {
    Up,
    Down,
}
