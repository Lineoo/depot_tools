use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use smallvec::SmallVec;

use crate::{
    application::IdType,
    paint::{painter::Painter, shapes::Rect},
    ui_control::{
        control::{Container, Control, Group, Handle, WeakHandle},
        ctrl_creator::CtrlCreator,
    },
};

struct HBoxItem {
    ctrl: Handle<dyn Control>,
    expand: bool,
    // ratio: f32,
    // padding: u32,
}

pub struct HBox {
    id: IdType,
    parent_id: Option<IdType>,
    win_id: Option<IdType>,
    children: SmallVec<[HBoxItem; 3]>,
    geometry: Rect,
    creator: Rc<CtrlCreator>,
    this: Option<WeakHandle<Self>>,
}

impl HBox {
    pub fn create(creator: Rc<CtrlCreator>) -> Rc<RefCell<Self>> {
        let id = creator.id_mgr().borrow_mut().get_id();
        let r = Rc::new(RefCell::new(HBox {
            id,
            parent_id: None,
            win_id: None,
            children: SmallVec::new(),
            geometry: Rect::new(0, 0, 0, 0),
            creator,
            this: None,
        }));
        r.borrow_mut().this = Some(Rc::downgrade(&r));
        r
    }

    pub fn add(
        &mut self,
        child: Weak<RefCell<dyn Control>>,
        expand: bool,
        position: InsertPosition,
    ) {
        let child = child.upgrade();
        if let Some(child) = child {
            let item = HBoxItem {
                ctrl: child,
                expand,
            };
            match position {
                InsertPosition::First => self.children.insert(0, item),
                InsertPosition::Last => self.children.push(item),
                InsertPosition::Index(idx) => {
                    if idx >= self.children.len() {
                        self.children.push(item)
                    } else {
                        self.children.insert(idx, item)
                    }
                }
            }
        }
    }
}

impl Control for HBox {
    fn window_id(&self) -> Option<IdType> {
        self.win_id
    }

    fn parent_id(&self) -> Option<IdType> {
        self.parent_id
    }

    fn id(&self) -> IdType {
        self.id
    }

    fn set_parent_container(&mut self, parent: WeakHandle<dyn Container>) -> bool {
        if let Some(parent) = parent.upgrade() {
            parent.borrow_mut().set_child(self.this.clone().unwrap());
            true
        } else {
            false
        }
    }

    fn set_parent_group(&mut self, parent: WeakHandle<dyn Group>) -> bool {
        if let Some(parent) = parent.upgrade() {
            parent.borrow_mut().add_child(self.this.clone().unwrap());
            true
        } else {
            false
        }
    }

    fn paint(&mut self, painter: &mut Painter) {
        todo!()
    }

    fn set_pos(&mut self, x: u32, y: u32) {
        self.geometry.x = x;
        self.geometry.y = y;
    }

    fn set_size(&mut self, width: u32, height: u32) {
        self.geometry.w = width;
        self.geometry.h = height;
    }

    fn pos(&self) -> (u32, u32) {
        self.geometry.pos()
    }

    fn size(&self) -> (u32, u32) {
        self.geometry.size()
    }

    fn geometry(&self) -> Rect {
        self.geometry
    }

    fn try_add_child(&mut self, id: IdType) -> bool {
        if let Some(ctrl) = self.creator.ctrl_mgr().get_ctrl(id) {
            self.add_child(Rc::downgrade(&ctrl));
            true
        } else {
            false
        }
    }
}

impl Group for HBox {
    fn add_child(&mut self, child: WeakHandle<dyn Control>) {
        self.add(child, false, InsertPosition::First);
    }

    fn add_children(&mut self, children: &[WeakHandle<dyn Control>]) {
        for child in children {
            self.add_child(child.clone());
        }
    }

    fn child_count(&self) -> usize {
        self.children.len()
    }

    fn child_id_at(&self, idx: usize) -> Option<IdType> {
        if idx >= self.children.len() {
            None
        } else {
            self.children[idx].ctrl.borrow().id().into()
        }
    }
}

pub enum InsertPosition {
    First,
    Last,
    Index(usize),
}
