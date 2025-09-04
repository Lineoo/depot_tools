use std::{cell::RefCell, collections::LinkedList, rc::Rc};

use sdl3::pixels::Color;

use crate::{
    application::IdType,
    id_manager::IdManager,
    paint::{painter::Painter, shapes::Rect},
    ui_control::{
        control::{Control, Handle, Insertable},
        ctrl_creator::CtrlCreator,
        ctrl_mgr::CtrlMgr,
    },
};

pub struct ListWidget {
    id: IdType,
    items: LinkedList<ListWidgetItem>,
    geometry: Rect,
    creator: Rc<CtrlCreator>,
}

impl ListWidget {
    pub fn new(creator: Rc<CtrlCreator>) -> Self {
        let id = creator.id_mgr().borrow_mut().get_id();
        ListWidget {
            id,
            items: LinkedList::new(),
            geometry: Rect::new(0, 0, 0, 0),
            creator,
        }
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
    fn window_id(&self) -> IdType {
        todo!()
    }

    fn parent_id(&self) -> Option<IdType> {
        todo!()
    }

    fn id(&self) -> IdType {
        self.id
    }

    fn set_parent_to(&mut self, parent: Handle<Box<dyn Control>>) {
        todo!()
    }

    fn set_parent_by_id(&mut self, parent_id: IdType) {
        todo!()
    }

    fn paint(&mut self, painter: &mut Painter) {
        painter.set_color(Color::WHITE);
        painter.rect(self.geometry);
        painter.set_color(Color::BLACK);
        painter.rect(Rect::new(3, 3, self.geometry.w - 6, self.geometry.h - 6));
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
