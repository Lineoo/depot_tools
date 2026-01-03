use std::{collections::LinkedList, fmt::Debug, rc::Rc};

use anyhow::Error;
use sdl3::pixels::Color;

use crate::{
    application::IdType,
    control::{
        Control, ControlCapability, Handle, Insertable, WeakHandle, ctrl_ctx::CtrlCtx,
        ctrl_mgr::CtrlMgr, util::focus_mgr::FocusMgr,
    },
    event::win_init::WinInitEvent,
    paint::{painter::Painter, shapes::Rect},
};

pub struct ListWidget {
    id: IdType,
    parent: WeakHandle<dyn Control>,
    win_id: Option<IdType>,
    items: LinkedList<ListWidgetItem>,
    geometry: Rect,
    ctrl_ctx: Rc<CtrlCtx>,
    this: Option<WeakHandle<ListWidget>>,
}

impl ListWidget {
    pub fn create(ctrl_ctx: Rc<CtrlCtx>) -> Handle<Self> {
        let id = ctrl_ctx.id_mgr().borrow_mut().get_id();
        let r = Handle::new(Self {
            id,
            parent: WeakHandle::new(),
            win_id: None,
            items: LinkedList::new(),
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

    pub fn insert_item(&mut self, item: String, pos: Option<usize>) {
        match pos {
            Some(idx) => {
                if idx >= self.items.len() {
                    self.items.push_back(ListWidgetItem::new(item));
                } else {
                    let mut split = self.items.split_off(idx);
                    self.items.push_back(ListWidgetItem::new(item));
                    self.items.append(&mut split);
                }
            }
            None => {
                self.items.push_back(ListWidgetItem::new(item));
            }
        }
    }

    pub fn item_count(&self) -> usize {
        self.items.len()
    }
}

impl Control for ListWidget {
    fn query_capability(&self, cap: ControlCapability) -> bool {
        match cap {
            ControlCapability::CanInsertChild => false,
            ControlCapability::CanInsertMultiChildren => false,
            ControlCapability::Focus => true,
            _ => false,
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
                .add_child(self.this.clone().unwrap().clone_untyped());
            true
        } else {
            false
        }
    }

    fn paint(&mut self, painter: &mut Painter) {
        painter.set_color(Color::WHITE);
        painter.rect(self.geometry);
        let (x, y) = self.pos();
        painter.set_color(Color::BLACK);
        painter.rect(Rect::new(
            x + 3,
            y + 3,
            self.geometry.w - 6,
            self.geometry.h - 6,
        ));
        // todo!()
    }

    fn set_pos(&mut self, x: i32, y: i32) {
        self.geometry.x = x;
        self.geometry.y = y;
    }

    fn set_size(&mut self, width: u32, height: u32) {
        self.geometry.w = width;
        self.geometry.h = height;
    }

    fn set_geometry(&mut self, r: Rect) {
        self.geometry = r;
    }

    fn pos(&self) -> (i32, i32) {
        (self.geometry.x, self.geometry.y)
    }

    fn size(&self) -> (u32, u32) {
        (self.geometry.w, self.geometry.h)
    }

    fn geometry(&self) -> Rect {
        self.geometry
    }

    fn add_child(&mut self, child: WeakHandle<dyn Control>) -> anyhow::Result<IdType> {
        Err(Error::msg("ListWidget cannot have children"))
    }

    fn remove_child(&mut self, child: WeakHandle<dyn Control>) {}

    fn remove_child_by_id(&mut self, child_id: IdType) {}

    fn get_children(&mut self) {}

    fn destroy_children(&mut self) {}

    fn on_init(&mut self, _event: &WinInitEvent) {}

    fn process_event(&mut self, event: Box<dyn crate::event::Event>) -> bool {
        todo!()
    }

    fn insert_tree(&self, focus_mgr: &mut FocusMgr) {
        focus_mgr.insert(self.this.clone().unwrap().into_untyped());
    }
}

impl Insertable for ListWidget {
    fn insert_tree(mut self, mgr: &mut CtrlMgr) {
        mgr.insert_item(Handle::new(self).clone_untyped());
    }
}

impl Drop for ListWidget {
    fn drop(&mut self) {
        self.ctrl_ctx.id_mgr().borrow_mut().release_id(self.id);
    }
}

impl Debug for ListWidget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListWidget")
            .field("id", &self.id)
            .field("items_count", &self.items.len())
            .field("geometry", &self.geometry)
            .field(
                "items",
                &self.items.iter().map(|i| &i.text).collect::<Vec<_>>(),
            )
            .finish()
    }
}

struct ListWidgetItem {
    pub(crate) text: String,
}

impl ListWidgetItem {
    pub(crate) fn new(text: String) -> Self {
        Self { text }
    }

    pub(crate) fn set_text(&mut self, text: String) {
        self.text = text;
    }
}
