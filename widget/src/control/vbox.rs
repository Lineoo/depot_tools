use std::{
    any::{Any, TypeId},
    cell::RefCell,
    rc::{Rc, Weak},
};

use smallvec::SmallVec;

use crate::{
    application::IdType,
    control::{Control, ControlCapability, Handle, WeakHandle, ctrl_ctx::CtrlCtx},
    event::{Event, control::CtrlResizeEvent, win_init::WinInitEvent, window::WinResizeEvent},
    paint::{painter::Painter, shapes::Rect},
    window::WindowDirector,
};

struct VBoxItem {
    ctrl: Handle<dyn Control>,
    expand: bool,
    // ratio: f32,
    // padding: u32,
}

pub struct VBox {
    id: IdType,
    parent: WeakHandle<dyn Control>,
    win_id: Option<IdType>,
    children: SmallVec<[VBoxItem; 3]>,
    geometry: Rect,
    ctrl_ctx: Rc<CtrlCtx>,

    preferred_size: (u32, u32),
    min_size: (u32, u32),
    max_size: (u32, u32),

    this: Option<WeakHandle<Self>>,
}

impl VBox {
    pub fn create(ctrl_ctx: Rc<CtrlCtx>) -> Handle<Self> {
        let id = ctrl_ctx.id_mgr().borrow_mut().get_id();
        let r = Handle::new(VBox {
            id,
            parent: WeakHandle::empty(),
            win_id: None,
            children: SmallVec::new(),
            geometry: Rect::new(0, 0, 0, 0),
            ctrl_ctx: ctrl_ctx.clone(),
            preferred_size: (0, 0),
            min_size: (0, 0),
            max_size: (u32::MAX, u32::MAX),
            this: None,
        });
        r.borrow_mut().this = Some(r.downgrade());
        ctrl_ctx
            .ctrl_mgr()
            .borrow_mut()
            .insert_item(r.clone().into_untyped());
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
    fn query_capability(&self, cap: ControlCapability) -> bool {
        match cap {
            ControlCapability::CanInsertChild => true,
            ControlCapability::CanInsertMultiChildren => true,
            _ => self
                .children
                .iter()
                .any(|c| c.ctrl.borrow().query_capability(cap)),
        }
    }

    fn window_id(&self) -> Option<IdType> {
        self.win_id
    }

    fn parent(&self) -> WeakHandle<dyn Control> {
        self.parent.clone()
    }

    fn id(&self) -> IdType {
        self.id
    }

    fn set_parent(&mut self, parent: WeakHandle<dyn Control>) -> bool {
        if let Some(parent) = parent.upgrade() {
            parent
                .borrow_mut()
                .add_child(self.this.clone().unwrap().clone().into_untyped());
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
            let (_cw, ch) = child.ctrl.borrow().preferred_size();
            let pos = child.ctrl.borrow().pos();
            child.ctrl.borrow_mut().set_pos(pos.0, y);
            let new_height = if child.expand { expend_height } else { ch };
            y += new_height as i32;
            child.ctrl.borrow_mut().set_size(w, new_height);
            child.ctrl.borrow_mut().paint(painter);
        }
    }

    fn set_pos(&mut self, x: i32, y: i32) {
        self.geometry.x = x;
        self.geometry.y = y;
    }

    fn set_size(&mut self, width: u32, height: u32) {
        self.geometry.w = width;
        self.geometry.h = height;
    }

    fn pos(&self) -> (i32, i32) {
        self.geometry.pos()
    }

    fn size(&self) -> (u32, u32) {
        self.geometry.size()
    }

    fn geometry(&self) -> Rect {
        self.geometry
    }

    fn add_child(&mut self, child: WeakHandle<dyn Control>) -> anyhow::Result<IdType> {
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

    fn receives_event(&self, event_type: std::any::TypeId) -> bool {
        self.children
            .iter()
            .any(|item| item.ctrl.borrow().receives_event(event_type))
    }

    fn process_event(&mut self, event: Box<dyn Event>) -> bool {
        if event.get_type_id() == TypeId::of::<CtrlResizeEvent>() {
            let event = event.downcast_ref::<CtrlResizeEvent>().unwrap();
            let (w, h) = event.new_size;
            self.set_size(w, h);
        }
        for item in self.children.iter() {
            if item.ctrl.borrow().receives_event(event.get_type_id()) {
                return item.ctrl.borrow_mut().process_event(event);
            }
        }
        false
    }

    fn insert_tree(&self, focus_mgr: &mut super::util::focus_mgr::FocusMgr) {
        for item in self.children.iter() {
            item.ctrl.borrow_mut().insert_tree(focus_mgr);
        }
    }

    fn attach_window(&mut self, win: Weak<RefCell<WindowDirector>>) {
        for c in self.children.iter() {
            c.ctrl.borrow_mut().attach_window(win.clone());
        }
    }

    fn set_preferred_size(&mut self, preferred_size: (u32, u32)) {
        self.preferred_size = preferred_size;
    }

    fn set_min_size(&mut self, min_size: (u32, u32)) {
        self.min_size = min_size;
    }

    fn set_max_size(&mut self, max_size: (u32, u32)) {
        self.max_size = max_size;
    }

    fn preferred_size(&self) -> (u32, u32) {
        if self.preferred_size == (0, 0) {
            (
                self.children
                    .iter()
                    .map(|c| c.ctrl.borrow().preferred_width())
                    .max()
                    .unwrap_or(10),
                self.children
                    .iter()
                    .map(|c| c.ctrl.borrow().preferred_height())
                    .sum::<u32>()
                    .max(10),
            )
        } else {
            self.preferred_size
        }
    }

    fn min_size(&self) -> (u32, u32) {
        if self.min_size == (0, 0) {
            (
                self.children
                    .iter()
                    .map(|c| c.ctrl.borrow().min_width())
                    .max()
                    .unwrap_or(10),
                self.children
                    .iter()
                    .map(|c| c.ctrl.borrow().min_height())
                    .sum::<u32>()
                    .max(10),
            )
        } else {
            self.min_size
        }
    }

    fn max_size(&self) -> (u32, u32) {
        if self.max_size == (0, 0) {
            (
                self.children
                    .iter()
                    .map(|c| c.ctrl.borrow().max_width())
                    .max()
                    .unwrap_or(10),
                self.children
                    .iter()
                    .map(|c| c.ctrl.borrow().max_height())
                    .sum::<u32>()
                    .max(10),
            )
        } else {
            self.max_size
        }
    }
}

impl Drop for VBox {
    fn drop(&mut self) {
        self.ctrl_ctx.id_mgr().borrow_mut().release_id(self.id);
    }
}

pub enum InsertPosition {
    First,
    Last,
    Index(usize),
}
