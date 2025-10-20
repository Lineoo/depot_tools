use std::rc::Rc;

use smallvec::SmallVec;

use crate::{
    application::IdType,
    event::win_init::WinInitEvent,
    paint::{painter::Painter, shapes::Rect},
    ui_control::{
        control::{Control, Handle, WeakHandle},
        ctrl_ctx::CtrlCtx,
    },
};

struct VBoxItem {
    ctrl: Handle<dyn Control>,
    expand: bool,
    // ratio: f32,
    // padding: u32,
}

pub struct VBox {
    id: IdType,
    parent_id: Option<IdType>,
    win_id: Option<IdType>,
    children: SmallVec<[VBoxItem; 3]>,
    geometry: Rect,
    ctrl_ctx: Rc<CtrlCtx>,
    this: Option<WeakHandle<Self>>,
}

impl VBox {
    pub fn create(ctrl_ctx: Rc<CtrlCtx>) -> Handle<Self> {
        let id = ctrl_ctx.id_mgr().borrow_mut().get_id();
        let r = Handle::new(VBox {
            id,
            parent_id: None,
            win_id: None,
            children: SmallVec::new(),
            geometry: Rect::new(0, 0, 0, 0),
            ctrl_ctx: ctrl_ctx.clone(),
            this: None,
        });
        r.borrow_mut().this = Some(r.downgrade());
        ctrl_ctx
            .ctrl_mgr()
            .borrow_mut()
            .insert_item(r.clone_untyped());
        r
    }

    pub fn add(&mut self, child: WeakHandle<dyn Control>, expand: bool, position: InsertPosition) {
        let child = child.upgrade();
        if let Some(child) = child {
            let item = VBoxItem {
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

impl Control for VBox {
    fn window_id(&self) -> Option<IdType> {
        self.win_id
    }

    fn parent_id(&self) -> Option<IdType> {
        self.parent_id
    }

    fn id(&self) -> IdType {
        self.id
    }

    fn set_parent(&mut self, parent: WeakHandle<dyn Control>) -> bool {
        if let Some(parent) = parent.upgrade() {
            parent
                .borrow_mut()
                .add_child(self.this.clone().unwrap().clone_untyped());
            true
        } else {
            false
        }
    }

    fn paint(&mut self, painter: &mut Painter) {
        let expend_count = self
            .children
            .iter()
            .filter(|item| item.expand)
            .count()
            .max(1);
        let (w, h) = self.size();
        let rest_height = h - self
            .children
            .iter()
            .filter(|item| !item.expand)
            .map(|item| item.ctrl.borrow().size().1)
            .sum::<u32>();
        let expend_height = rest_height / expend_count as u32;
        let mut y = self.pos().1;
        for child in &self.children {
            let (_cw, ch) = child.ctrl.borrow().size();
            let pos = child.ctrl.borrow().pos();
            child.ctrl.borrow_mut().set_pos(pos.0, y);
            let new_height = if child.expand { expend_height } else { ch };
            y += new_height;
            child.ctrl.borrow_mut().set_size(w, new_height);
            child.ctrl.borrow_mut().paint(painter);
        }
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

    fn add_child(&mut self, child: WeakHandle<dyn Control>) -> Result<IdType, ()> {
        todo!()
    }

    fn remove_child(&mut self, child: WeakHandle<dyn Control>) {
        todo!()
    }

    fn remove_child_by_id(&mut self, child_id: IdType) {
        todo!()
    }

    fn get_children(&mut self) {
        todo!()
    }

    fn destroy_children(&mut self) {
        todo!()
    }

    fn on_init(&mut self, event: &WinInitEvent) {
        for child in &self.children {
            if let Some(mut ctrl) = child.ctrl.try_borrow_mut() {
                ctrl.on_init(event);
            }
        }
    }
}

pub enum InsertPosition {
    First,
    Last,
    Index(usize),
}
