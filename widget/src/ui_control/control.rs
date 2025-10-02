use std::{any::Any, panic};

use crate::{
    application::IdType,
    paint::{painter::Painter, shapes::Rect},
    ui_control::ctrl_mgr::CtrlMgr,
};

pub type Handle<T> = crate::ui_control::handle::Handle<T>;
pub type WeakHandle<T> = crate::ui_control::handle::WeakHandle<T>;
pub type EventArg = Option<Box<dyn Any>>;

pub trait Control {
    fn window_id(&self) -> Option<IdType>;

    fn parent_id(&self) -> Option<IdType>;
    fn id(&self) -> IdType;

    /// true on success and false on failure
    fn set_parent(&mut self, parent: WeakHandle<dyn Control>) -> bool;

    fn paint(&mut self, painter: &mut Painter);

    fn set_pos(&mut self, x: u32, y: u32);
    fn set_size(&mut self, width: u32, height: u32);
    fn set_geometry(&mut self, r: Rect) {
        self.set_pos(r.x, r.y);
        self.set_size(r.w, r.h);
    }

    fn pos(&self) -> (u32, u32);
    fn size(&self) -> (u32, u32);
    fn geometry(&self) -> Rect {
        let (x, y) = self.pos();
        let (w, h) = self.size();
        Rect { x, y, w, h }
    }

    fn subscribe(&mut self, _event: String, _provider: WeakHandle<dyn Control>) -> bool {
        panic!("Not implemented");
    }

    // fn on_event(&mut self, event: String, arg: EventArg);

    /// true on success and false on failure
    fn add_child(&mut self, child: WeakHandle<dyn Control>) -> Result<IdType, ()>;
    fn remove_child(&mut self, child: WeakHandle<dyn Control>);
    fn remove_child_by_id(&mut self, child_id: IdType);

    fn get_children(&mut self); // add return type
    fn destroy_children(&mut self);
}

pub trait Insertable: Control {
    fn insert_tree(self, mgr: &mut CtrlMgr);
}

pub trait Eventful: Control {
    fn slot_connect<Arg: 'static>(&mut self, name: String, slot: Box<dyn FnMut(Arg) + 'static>);
    fn raise_signal<Arg: 'static>(&mut self, name: String, arg: Arg);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AddChildErr(pub ());
