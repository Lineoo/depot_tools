use std::{cell::RefCell, rc::Rc};

use sdl3::keyboard::TextInputUtil;

use crate::{
    application::IdType,
    paint::{painter::Painter, shapes::Rect},
    ui_control::{
        control::{Container, Control, Group, WeakHandle},
        ctrl_ctx::CtrlCtx,
    },
};

pub struct InputBox {
    id: IdType,
    parent_id: Option<IdType>,
    win_id: Option<IdType>,
    geometry: Rect,
    ctrl_ctx: Rc<CtrlCtx>,
    this: Option<WeakHandle<Self>>,

    text: String,
    cursor_pos: usize,
    font_height: Option<f32>,
    placeholder: String,
    input_util: Rc<RefCell<TextInputUtil>>,
    focused: bool,
}

impl InputBox {}

impl Control for InputBox {
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
        painter.text(&self.text, 10, 10);
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

    fn try_add_child(&mut self, id: IdType) -> bool {
        false
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

    pub fn end(self) -> InputBox {
        self.input_box
    }
}
