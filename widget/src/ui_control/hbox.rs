use crate::{
    application::IdType,
    paint::painter::Painter,
    ui_control::control::{Control, Group},
};

pub struct HBox {}

impl HBox {}

impl Control for HBox {
    fn window_id(&self) -> Option<IdType> {
        todo!()
    }

    fn parent_id(&self) -> Option<IdType> {
        todo!()
    }

    fn id(&self) -> IdType {
        todo!()
    }

    fn set_parent(&mut self, parent_id: IdType) -> bool {
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

    fn try_add_child(&mut self, id: IdType) -> bool {
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

    fn child_id_at(&self, idx: usize) -> Option<IdType> {
        todo!()
    }
}
