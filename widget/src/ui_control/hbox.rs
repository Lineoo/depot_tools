use crate::ui_control::control::{Control, Group};

pub struct HBox {}

impl HBox {}

impl Control for HBox {
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

    fn paint(&mut self, painter: &mut crate::paint::painter::Painter) {
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

impl Group for HBox {
    fn add_child(&mut self, child: super::control::Handle<std::cell::RefCell<dyn Control>>) {
        todo!()
    }

    fn add_children(
        &mut self,
        children: &[super::control::Handle<std::cell::RefCell<dyn Control>>],
    ) {
        todo!()
    }

    fn child_count(&self) -> usize {
        todo!()
    }

    fn child_id_at(&self, idx: usize) -> Option<crate::application::IdType> {
        todo!()
    }
}
