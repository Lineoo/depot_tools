use std::{
    cell::RefCell,
    collections::LinkedList,
    fmt::Debug,
    rc::{Rc, Weak},
};

use anyhow::Error;
use sdl3::pixels::Color;

use crate::{
    application::IdType,
    control::{
        Control, ControlCapability, Handle, WeakHandle, ctrl_ctx::CtrlCtx, ctrl_mgr::CtrlMgr,
        util::focus_mgr::FocusMgr,
    },
    event::{Event, win_init::WinInitEvent},
    paint::{creator::PainterCreator, painter::Painter, shapes::Rect},
    window::{WindowDirector, win},
};

pub struct RichListWidget {
    id: IdType,
    parent: WeakHandle<dyn Control>,
    win_id: Option<IdType>,
    items: LinkedList<RichListWidgetItem>,
    geometry: Rect,
    ctrl_ctx: Rc<CtrlCtx>,
    painter: Option<Painter>,
    painter_creator: Option<PainterCreator>,
    need_redraw: bool,

    old_size: usize,
    selected: Option<usize>,

    preferred_size: (u32, u32),
    min_size: (u32, u32),
    max_size: (u32, u32),

    this: Option<WeakHandle<RichListWidget>>,
}

impl RichListWidget {
    pub fn create(ctrl_ctx: Rc<CtrlCtx>) -> Handle<Self> {
        let id = ctrl_ctx.id_mgr().borrow_mut().get_id();
        let r = Handle::new(Self {
            id,
            parent: WeakHandle::empty(),
            win_id: None,
            items: LinkedList::new(),
            geometry: Rect::new(0, 0, 0, 0),
            ctrl_ctx: ctrl_ctx.clone(),
            painter: None,
            painter_creator: None,
            need_redraw: false,
            old_size: 0,
            selected: None,
            preferred_size: (100, 100),
            min_size: (0, 0),
            max_size: (0, 0),
            this: None,
        });
        r.borrow_mut().this = Some(r.downgrade());
        ctrl_ctx
            .ctrl_mgr()
            .borrow_mut()
            .insert_item(r.clone().into_untyped());
        r
    }

    pub fn insert_item(&mut self, item: String, desc: String, pos: Option<usize>) {
        self.need_redraw = true;
        match pos {
            Some(idx) => {
                if idx >= self.items.len() {
                    self.items.push_back(RichListWidgetItem::new(item, desc));
                } else {
                    let mut split = self.items.split_off(idx);
                    self.items.push_back(RichListWidgetItem::new(item, desc));
                    self.items.append(&mut split);
                }
            }
            None => {
                self.items.push_back(RichListWidgetItem::new(item, desc));
            }
        }
        self.old_size += 1;
    }

    pub fn item_list_ref(&self) -> &LinkedList<RichListWidgetItem> {
        &self.items
    }

    pub fn item_list_mut(&mut self) -> &mut LinkedList<RichListWidgetItem> {
        self.need_redraw = true;
        &mut self.items
    }

    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    pub fn select_up(&mut self, wrap: bool) {
        if self.items.is_empty() {
            return;
        }
        if self.selected.is_none() {
            self.selected = Some(0);
        } else {
            let idx = self.selected.unwrap();
            if idx == 0 {
                if wrap {
                    self.selected = Some(self.item_count() - 1);
                }
            } else {
                self.selected = Some(idx - 1);
            }
        }
    }

    pub fn select_down(&mut self, wrap: bool) {
        if self.items.is_empty() {
            return;
        }
        if self.selected.is_none() {
            self.selected = Some(0);
        } else {
            let idx = self.selected.unwrap();
            if idx >= self.item_count() - 1 {
                if wrap {
                    self.selected = Some(0);
                }
            } else {
                self.selected = Some(idx + 1);
            }
        }
    }

    pub fn select_item(&mut self, idx: usize) {
        self.selected = if idx < self.item_count() {
            Some(idx)
        } else {
            None
        };
        self.need_redraw = true;
    }

    pub fn selected_item(&self) -> Option<usize> {
        self.selected
    }

    pub fn unselect_item(&mut self) {
        self.selected = None;
    }

    fn calc_inner_height(&self) -> u32 {
        (self.item_count() * 43 + 3) as u32
    }

    fn render_inner_items(&mut self) {
        if !self.need_redraw {
            return;
        }
        if self.old_size != self.item_count() {
            // items changed in `item_list_mut`
            self.selected = None;
        }
        self.old_size = self.item_count();
        self.need_redraw = false;
        let painter = self.painter.as_mut().unwrap();
        let mut current_y = 3;
        let width = self.geometry.w - 12;
        for (idx, item) in self.items.iter().enumerate() {
            painter.set_color(
                if let Some(selected) = self.selected
                    && selected == idx
                {
                    Color::RED
                } else {
                    Color::YELLOW
                },
            );
            painter.rect(Rect::new(3, current_y, width, 40));
            painter.set_color(Color::BLACK);
            painter.text(&item.text, 6, current_y, None);
            painter.set_color(Color::GRAY);
            painter.text(&item.desc, 6, current_y + 20, None);
            current_y += 43;
        }
    }
}

impl Control for RichListWidget {
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
                .add_child(self.this.clone().unwrap().clone().into_untyped());
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
        self.render_inner_items();
        let w = self.size().0;
        let h = self.calc_inner_height() + 6;
        painter.copy(
            self.painter.as_mut().unwrap(),
            Rect::new(0, 0, w - 6, h - 6),
            Rect::new(self.geometry.x + 3, self.geometry.y + 3, w - 6, h - 6),
        );
    }

    fn set_pos(&mut self, x: i32, y: i32) {
        self.geometry.x = x;
        self.geometry.y = y;
    }

    fn set_size(&mut self, width: u32, height: u32) {
        self.need_redraw = true;
        self.geometry.w = width;
        self.geometry.h = height;
        self.painter = Some(
            self.painter_creator
                .as_ref()
                .unwrap()
                .create((width - 6, self.calc_inner_height())),
        );
    }

    fn set_geometry(&mut self, r: Rect) {
        self.need_redraw = true;
        self.geometry = r;
        self.painter = Some(
            self.painter_creator
                .as_ref()
                .unwrap()
                .create((r.w - 6, self.calc_inner_height())),
        );
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

    fn process_event(&mut self, event: Box<dyn Event>) -> bool {
        todo!()
    }

    fn insert_tree(&self, focus_mgr: &mut FocusMgr) {
        focus_mgr.insert(self.this.clone().unwrap().into_untyped());
    }

    fn attach_window(&mut self, win: Weak<RefCell<WindowDirector>>) {
        let win = win.upgrade().unwrap();
        let win = win.borrow();
        self.painter_creator = Some(win.painter_creator());
        self.painter = Some(win.make_painter((0, 0)));
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
        self.preferred_size
    }

    fn min_size(&self) -> (u32, u32) {
        self.min_size
    }

    fn max_size(&self) -> (u32, u32) {
        self.max_size
    }
}

impl Drop for RichListWidget {
    fn drop(&mut self) {
        self.ctrl_ctx.id_mgr().borrow_mut().release_id(self.id);
    }
}

impl Debug for RichListWidget {
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

#[derive(Debug, Clone)]
pub struct RichListWidgetItem {
    pub(crate) text: String,
    pub(crate) desc: String,
    // pub(crate) icon: Option<>,
}

impl RichListWidgetItem {
    pub(crate) fn new(text: String, desc: String) -> Self {
        Self { text, desc }
    }
}
