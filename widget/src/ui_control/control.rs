use std::{cell::RefCell, rc::Rc};

use crate::{
    application::IdType,
    paint::{painter::Painter, shapes::Rect},
    ui_control::ctrl_mgr::CtrlMgr,
};

pub type Handle<T> = Rc<RefCell<T>>;

pub trait Control {
    fn window_id(&self) -> Option<IdType>;

    fn parent_id(&self) -> Option<IdType>;
    fn id(&self) -> IdType;

    /// true on success and false on failure
    fn set_parent(&mut self, parent_id: IdType) -> bool;

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

    /// true on success and false on failure
    fn try_add_child(&mut self, id: IdType) -> bool;
}

pub trait Insertable: Control {
    fn insert_tree(self, mgr: &mut CtrlMgr);
}

pub trait Container: Control {
    fn set_children(&mut self, child: Handle<RefCell<dyn Control>>);

    fn child(&self) -> Handle<Box<dyn Control>>;
    fn child_id(&self) -> IdType;
}

pub trait Group: Control {
    fn add_child(&mut self, child: Handle<RefCell<dyn Control>>);
    fn add_children(&mut self, children: &[Handle<RefCell<dyn Control>>]);

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
