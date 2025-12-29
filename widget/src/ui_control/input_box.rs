use std::{cell::RefCell, path::Path, rc::Rc, sync::Arc};

use anyhow::Error;
use sdl3::{keyboard::TextInputUtil, pixels::Color};

use crate::{
    application::IdType,
    event::{Event, win_init::WinInitEvent},
    paint::{painter::Painter, shapes::Rect},
    ui_control::{
        control::{Control, ControlCapability, Handle, WeakHandle},
        ctrl_ctx::CtrlCtx,
        font::Font,
    },
};

pub struct InputBox {
    id: IdType,
    parent: WeakHandle<dyn Control>,
    win_id: Option<IdType>,
    geometry: Rect,
    ctrl_ctx: Rc<CtrlCtx>,
    font: Rc<RefCell<Font>>,
    this: Option<WeakHandle<Self>>,

    text: String,
    cursor_pos: usize,
    font_height: Option<f32>,
    placeholder: String,
    input_util: Option<Rc<RefCell<TextInputUtil>>>,
}

impl InputBox {
    pub fn create(ctrl_ctx: Rc<CtrlCtx>) -> Handle<Self> {
        let r = Self::new(ctrl_ctx);
        Handle::new(r)
    }

    pub fn builder(ctrl_ctx: Rc<CtrlCtx>) -> InputBoxBuilder {
        InputBoxBuilder {
            input_box: Self::new(ctrl_ctx),
        }
    }

    fn new(ctrl_ctx: Rc<CtrlCtx>) -> Self {
        let id = ctrl_ctx.id_mgr().borrow_mut().get_id();
        ctrl_ctx
            .font_mgr()
            .borrow_mut()
            .load_local_family("FiraCode-Regular.ttf");
        Self {
            id,
            parent: WeakHandle::new(),
            win_id: None,
            geometry: Rect::new(0, 0, 100, 30),
            ctrl_ctx: ctrl_ctx.clone(),
            font: Rc::new(RefCell::new(
                ctrl_ctx
                    .font_mgr()
                    .borrow()
                    .get_font_by_name("FiraCode-Regular", 22)
                    .unwrap(),
            )),
            this: None,
            text: String::new(),
            cursor_pos: 0,
            font_height: None,
            placeholder: String::from("Input..."),
            input_util: None,
        }
    }
}

impl Control for InputBox {
    fn query_capability(&self, cap: ControlCapability) -> bool {
        match cap {
            ControlCapability::CanInsertChild => false,
            ControlCapability::CanInsertMultiChildren => false,
            ControlCapability::TextEdit => true,
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
        if let Some(old_parent) = self.parent.upgrade() {
            old_parent
                .borrow_mut()
                .remove_child(self.this.clone().unwrap().into_untyped());
        }
        if let Some(new_parent) = parent.upgrade() {
            new_parent
                .borrow_mut()
                .add_child(self.this.clone().unwrap().into_untyped());
            self.parent = parent;
            true
        } else {
            false
        }
    }

    fn paint(&mut self, painter: &mut Painter) {
        let (x, y) = self.pos();
        painter.set_color(Color::WHITE);
        painter.text(&self.text, x + 2, y + 2, self.font.clone());
        // TODO: more decorations and cursor
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

    fn add_child(&mut self, child: WeakHandle<dyn Control>) -> anyhow::Result<IdType> {
        Err(Error::msg("InputBox cannot have children"))
    }

    fn remove_child(&mut self, child: WeakHandle<dyn Control>) {}

    fn remove_child_by_id(&mut self, child_id: IdType) {}

    fn get_children(&mut self) {}

    fn destroy_children(&mut self) {}

    fn on_init(&mut self, event: &WinInitEvent) {}

    fn process_event(&mut self, event: Box<dyn Event>) -> bool {
        todo!()
    }
}

impl Drop for InputBox {
    fn drop(&mut self) {
        self.ctrl_ctx.id_mgr().borrow_mut().release_id(self.id);
    }
}

pub struct InputBoxBuilder {
    input_box: InputBox,
}

impl InputBoxBuilder {
    pub fn placeholder(mut self, text: &str) -> Self {
        self.input_box.placeholder = text.to_string();
        self
    }

    pub fn font_height(mut self, height: f32) -> Self {
        self.input_box.font_height = Some(height);
        self
    }

    pub fn build(self) -> Handle<InputBox> {
        Handle::new(self.input_box)
    }
}
