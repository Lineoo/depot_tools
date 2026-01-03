use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
    sync::{Arc, Mutex},
};

use sdl3::keyboard::TextInputUtil;

use crate::{
    application::IdType,
    id_manager::IdManager,
    ui_control::{
        control::{Control, Handle, Insertable},
        ctrl_ctx::CtrlCtx,
        font::FontMgr,
        util::{focus_mgr::FocusMgr, text_edit},
    },
};

pub struct CtrlMgr {
    ctrls: HashMap<IdType, Handle<dyn Control>>,
}

impl CtrlMgr {
    pub fn new() -> Self {
        CtrlMgr {
            ctrls: HashMap::new(),
        }
    }

    pub fn add_ctrl<C: Insertable>(&mut self, mut ctrl: C) {
        ctrl.insert_tree(self);
    }

    pub fn insert_item(&mut self, ctrl: Handle<dyn Control>) {
        let id = (*ctrl.borrow()).id();
        self.ctrls.insert(id, ctrl);
    }

    pub fn is_valid_id(&self, id: IdType) -> bool {
        self.ctrls.contains_key(&id)
    }

    pub fn get_ctrl(&self, id: IdType) -> Option<Handle<dyn Control>> {
        Some(self.ctrls.get(&id)?.clone())
    }
}

pub(crate) fn make_ctrl_ctx(
    id_mgr: Rc<RefCell<IdManager>>,
    ctrl_mgr: Rc<RefCell<CtrlMgr>>,
    font_mgr: Rc<RefCell<FontMgr>>,
    focus_mgr: Arc<Mutex<FocusMgr>>,
    text_input_util: Rc<RefCell<TextInputUtil>>,
) -> CtrlCtx {
    CtrlCtx::new(id_mgr, ctrl_mgr, font_mgr, focus_mgr, text_input_util)
}
