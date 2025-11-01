use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::{
    application::IdType,
    id_manager::IdManager,
    ui_control::{
        control::{Control, Handle, Insertable},
        ctrl_ctx::CtrlCtx,
        font::FontMgr,
        util::text_edit,
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
    active_text_edit: Rc<RefCell<Option<Weak<RefCell<text_edit::TextEdit>>>>>,
) -> CtrlCtx {
    CtrlCtx::new(id_mgr, ctrl_mgr, font_mgr, active_text_edit)
}
