use core::panic;
use std::{cell::RefCell, collections::LinkedList, fmt::Debug, rc::Rc};

use sdl3::pixels::Color;

use crate::{
    application::IdType,
    paint::{painter::Painter, shapes::Rect},
    ui_control::{
        control::{Container, Control, Group, Handle, Insertable, WeakHandle},
        ctrl_creator::CtrlCreator,
        ctrl_mgr::CtrlMgr,
    },
};

pub struct ListWidget {
    id: IdType,
    parent_id: Option<IdType>,
    win_id: Option<IdType>,
    items: LinkedList<ListWidgetItem>,
    geometry: Rect,
    creator: Rc<CtrlCreator>,
    this: Option<WeakHandle<ListWidget>>,
}

impl ListWidget {
    pub fn create(creator: Rc<CtrlCreator>) -> Handle<Self> {
        let id = creator.id_mgr().borrow_mut().get_id();
        let r = Handle::new(RefCell::new(ListWidget {
            id,
            parent_id: None,
            win_id: None,
            items: LinkedList::new(),
            geometry: Rect::new(0, 0, 0, 0),
            creator,
            this: None,
        }));
        r.borrow_mut().this = Some(Rc::downgrade(&r));
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
        painter.set_color(Color::WHITE);
        painter.rect(self.geometry);
        painter.set_color(Color::BLACK);
        painter.rect(Rect::new(3, 3, self.geometry.w - 6, self.geometry.h - 6));
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

    fn set_geometry(&mut self, r: Rect) {
        self.geometry = r;
    }

    fn pos(&self) -> (u32, u32) {
        (self.geometry.x, self.geometry.y)
    }

    fn size(&self) -> (u32, u32) {
        (self.geometry.w, self.geometry.h)
    }

    fn geometry(&self) -> Rect {
        self.geometry
    }

    fn try_add_child(&mut self, id: IdType) -> bool {
        false
    }
}

impl Insertable for ListWidget {
    fn insert_tree(mut self, mgr: &mut CtrlMgr) {
        mgr.insert_item(Rc::new(RefCell::new(self)));
    }
}

impl Drop for ListWidget {
    fn drop(&mut self) {
        self.creator.id_mgr().borrow_mut().release_id(self.id);
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
