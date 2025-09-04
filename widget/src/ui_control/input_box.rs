use crate::{paint::painter::Painter, ui_control::control::Control};

pub struct InputBox {}

impl InputBox {}

impl Control for InputBox {
    fn window_id(&self) -> crate::application::IdType {
        todo!()
    }

    fn parent_id(&self) -> Option<crate::application::IdType> {
        todo!()
    }

    fn id(&self) -> crate::application::IdType {
        todo!()
    }

    fn set_parent_to(&mut self, parent: super::control::Handle<Box<dyn Control>>) {
        todo!()
    }

    fn set_parent_by_id(&mut self, parent_id: crate::application::IdType) {
        todo!()
    }

    fn paint(&mut self, painter: &mut Painter) {
        todo!()
    }

    fn set_pos(&mut self, x: u32, y: u32) {
        todo!()
    }

    fn set_size(&mut self, width: u32, height: u32) {
        todo!()
    }

    fn pos(&self) -> (u32, u32) {
        todo!()
    }

    fn size(&self) -> (u32, u32) {
        todo!()
    }
}

impl Drop for InputBox {
    fn drop(&mut self) {
        todo!()
    }
}
