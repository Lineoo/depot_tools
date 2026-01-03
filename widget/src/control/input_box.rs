use std::{
    any::TypeId,
    cell::RefCell,
    path::Path,
    rc::{Rc, Weak},
    sync::Arc,
};

use anyhow::Error;
use cosmic_text::{Action, Motion as CosmicMotion};
use sdl3::{
    keyboard::{Keycode, TextInputUtil},
    pixels::Color,
};

use crate::{
    application::IdType,
    control::{
        Control, ControlCapability, Handle, WeakHandle,
        ctrl_ctx::CtrlCtx,
        font::Font,
        util::{focus_mgr::FocusMgr, text_edit::TextEdit},
    },
    event::{
        Event,
        control::CtrlResizeEvent,
        edit::{ImEditEvent, TextEditEvent},
        focus::{GainFocusEvent, LoseFocusEvent},
        keyboard::{KeyState, KeyboardEvent},
        win_init::WinInitEvent,
    },
    paint::{painter::Painter, shapes::Rect},
    window::WindowDirector,
};

pub struct InputBox {
    id: IdType,
    parent: WeakHandle<dyn Control>,
    win_id: Option<IdType>,
    geometry: Rect,
    ctrl_ctx: Rc<CtrlCtx>,
    // font: Rc<RefCell<Font>>,
    this: Option<WeakHandle<Self>>,

    edit: TextEdit,
    font_height: Option<f32>,
    placeholder: String,
    input_util: Option<Rc<RefCell<TextInputUtil>>>,
}

impl InputBox {
    pub fn create(ctrl_ctx: Rc<CtrlCtx>) -> Handle<Self> {
        let r = Handle::new(Self::new(ctrl_ctx.clone()));
        r.borrow_mut().this = Some(r.downgrade());
        ctrl_ctx
            .ctrl_mgr()
            .borrow_mut()
            .insert_item(r.clone_untyped());
        r
    }

    pub fn builder(ctrl_ctx: Rc<CtrlCtx>) -> InputBoxBuilder {
        InputBoxBuilder {
            input_box: Self::new(ctrl_ctx),
        }
    }

    fn new(ctrl_ctx: Rc<CtrlCtx>) -> Self {
        let id = ctrl_ctx.id_mgr().borrow_mut().get_id();
        ctrl_ctx.font_mgr().borrow_mut();
        // .load_local_family("FiraCode-Regular.ttf");
        Self {
            id,
            parent: WeakHandle::new(),
            win_id: None,
            geometry: Rect::new(0, 0, 100, 30),
            ctrl_ctx: ctrl_ctx.clone(),
            // font: Rc::new(RefCell::new(
            //     ctrl_ctx
            //         .font_mgr()
            //         .borrow()
            //         .get_font_by_name("FiraCode-Regular", 22)
            //         .unwrap(),
            // )),
            this: None,
            edit: TextEdit::new(ctrl_ctx.text_input_util(), &ctrl_ctx),
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
            ControlCapability::Focus => true,
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
        let (x, y, w, h) = self.geometry.into();
        self.edit.set_geometry(x, y, w, h);
        self.edit.render(painter);
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

    fn add_child(&mut self, child: WeakHandle<dyn Control>) -> anyhow::Result<IdType> {
        Err(Error::msg("InputBox cannot have children"))
    }

    fn remove_child(&mut self, child: WeakHandle<dyn Control>) {}

    fn remove_child_by_id(&mut self, child_id: IdType) {}

    fn get_children(&mut self) {}

    fn destroy_children(&mut self) {}

    fn on_init(&mut self, event: &WinInitEvent) {}

    fn receives_event(&self, _event_type: TypeId) -> bool {
        true
    }

    fn process_event(&mut self, event: Box<dyn Event>) -> bool {
        let type_id = event.get_type_id();
        if type_id == TypeId::of::<CtrlResizeEvent>() {
            let event = event.downcast_ref::<CtrlResizeEvent>().unwrap();
            let (w, h) = event.new_size;
            self.edit.set_size(w, h);
            true
        } else if type_id == TypeId::of::<ImEditEvent>() {
            let event = event.downcast_ref::<ImEditEvent>().unwrap();
            // TODO
            true
        } else if type_id == TypeId::of::<TextEditEvent>() {
            let event = event.downcast_ref::<TextEditEvent>().unwrap();
            self.edit.insert_text(&event.text);
            true
        } else if type_id == TypeId::of::<GainFocusEvent>() {
            self.edit.gain_focus().is_ok()
        } else if type_id == TypeId::of::<LoseFocusEvent>() {
            self.edit.lose_focus().is_ok()
        } else if type_id == TypeId::of::<KeyboardEvent>() {
            let event = event.downcast_ref::<KeyboardEvent>().unwrap();
            if event.state == KeyState::Pressed {
                match event.keycode {
                    Keycode::Backspace => self.edit.action(Action::Backspace),
                    Keycode::Delete => self.edit.action(Action::Delete),
                    Keycode::Return => self.edit.action(Action::Enter),
                    Keycode::Left => self.edit.action(Action::Motion(CosmicMotion::Left)),
                    Keycode::Right => self.edit.action(Action::Motion(CosmicMotion::Right)),
                    Keycode::Up => self.edit.action(Action::Motion(CosmicMotion::Up)),
                    Keycode::Down => self.edit.action(Action::Motion(CosmicMotion::Down)),
                    Keycode::Home => self.edit.action(Action::Motion(CosmicMotion::Home)),
                    Keycode::End => self.edit.action(Action::Motion(CosmicMotion::End)),
                    Keycode::PageUp => self.edit.action(Action::Motion(CosmicMotion::PageUp)),
                    Keycode::PageDown => self.edit.action(Action::Motion(CosmicMotion::PageDown)),
                    _ => return false,
                }
                true
            } else {
                false
            }
            // }else if type_id ==  {
        } else {
            false
        }
    }

    fn insert_tree(&self, focus_mgr: &mut FocusMgr) {
        focus_mgr.insert(self.this.clone().unwrap().into_untyped());
    }

    fn attach_window(&mut self, win: Weak<RefCell<WindowDirector>>) {
        self.edit.attach_window(win);
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
