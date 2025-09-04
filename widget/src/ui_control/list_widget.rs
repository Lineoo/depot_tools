use std::{cell::RefCell, collections::LinkedList, rc::Rc};

use sdl3::pixels::Color;

use crate::{
    application::{Application, IdType},
    id_manager::IdManager,
    paint::{painter::Painter, shapes::Rect},
    ui_control::{
        control::{Control, Group, Handle, Insertable},
        ctrl_mgr::CtrlMgr,
    },
};

pub struct ListWidget {
    id: Option<IdType>,
    items: LinkedList<ListWidgetItem>,
    geometry: Rect,
    id_mgr: Option<Rc<RefCell<IdManager>>>,
}

impl ListWidget {
    pub fn new() -> Self {
        ListWidget {
            id: None,
            items: LinkedList::new(),
            geometry: Rect::new(0, 0, 0, 0),
            id_mgr: None,
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
        debug_assert!(
            self.id.is_some(),
            "Control ID is only available after being added to the application."
        );
        self.id.unwrap()
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

    fn reg_tree(&mut self, ctrl_mgr: &mut CtrlMgr) {
        debug_assert!(self.id.is_none(), "Control can only be registered once.");
        let mut id_mgr = ctrl_mgr.id_mgr.borrow_mut();
        self.id = Some(id_mgr.get_id());
    }
}

impl Insertable for ListWidget {
    fn insert_tree(mut self, mgr: &mut CtrlMgr) {
        self.id_mgr = Some(mgr.id_mgr.clone());
        mgr.insert_item(Rc::new(RefCell::new(self)));
    }
}

impl Default for ListWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ListWidget {
    fn drop(&mut self) {
        if let Some(id_mgr) = &self.id_mgr {
            id_mgr.borrow_mut().release_id(self.id.unwrap());
        }
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
