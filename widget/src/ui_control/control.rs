use std::{
    any::Any,
    cell::RefCell,
    panic,
    rc::{Rc, Weak},
};

use crate::{
    application::IdType,
    paint::{painter::Painter, shapes::Rect},
    ui_control::ctrl_mgr::CtrlMgr,
};

pub type Handle<T> = Rc<RefCell<T>>;
pub type WeakHandle<T> = Weak<RefCell<T>>;
pub type EventArg = Option<Box<dyn Any>>;

pub trait Control {
    fn window_id(&self) -> Option<IdType>;

    fn parent_id(&self) -> Option<IdType>;
    fn id(&self) -> IdType;

    /// true on success and false on failure
    fn set_parent_container(&mut self, parent: WeakHandle<dyn Container>) -> bool;
    fn set_parent_group(&mut self, parent: WeakHandle<dyn Group>) -> bool;

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

    fn subscribe_from(&mut self, _event: String, _demander: Weak<RefCell<dyn Control>>) -> bool {
        // FIXME: swap duty of subscriber and demander
        panic!("Not implemented");
    }

    fn subscribe_with(&mut self, _event: String, _function: Box<dyn FnMut(EventArg)>) -> bool {
        // FIXME: swap duty of subscriber and demander
        panic!("Not implemented");
    }

    /// true on success and false on failure
    fn try_add_child(&mut self, id: IdType) -> bool;
}

pub trait Insertable: Control {
    fn insert_tree(self, mgr: &mut CtrlMgr);
}

pub trait Container: Control {
    fn set_child(&mut self, child: WeakHandle<dyn Control>);

    fn child(&self) -> WeakHandle<Box<dyn Control>>;
    fn child_id(&self) -> IdType;
}

pub trait Group: Control {
    fn add_child(&mut self, child: WeakHandle<dyn Control>);
    fn add_children(&mut self, children: &[WeakHandle<dyn Control>]);

    fn child_count(&self) -> usize;

    fn first_child_id(&self) -> Option<IdType> {
        self.child_id_at(0)
    }
    fn last_child_id(&self) -> Option<IdType> {
        self.child_id_at(self.child_count() - 1)
    }
    fn child_id_at(&self, idx: usize) -> Option<IdType>;
}

pub trait Eventful: Control {
    fn slot_connect<Arg: 'static>(&mut self, name: String, slot: Box<dyn FnMut(Arg) + 'static>);
    fn raise_signal<Arg: 'static>(&mut self, name: String, arg: Arg);
}
