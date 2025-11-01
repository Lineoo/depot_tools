use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    id_manager::IdManager,
    ui_control::{ctrl_mgr::CtrlMgr, font::FontMgr, util::text_edit},
};

pub struct CtrlCtx {
    id_mgr: Rc<RefCell<IdManager>>,
    ctrl_mgr: Rc<RefCell<CtrlMgr>>,
    font_mgr: Rc<RefCell<FontMgr>>,
    active_text_edit: Rc<RefCell<Option<Weak<RefCell<text_edit::TextEdit>>>>>,
}

impl CtrlCtx {
    pub(crate) fn new(
        id_mgr: Rc<RefCell<IdManager>>,
        ctrl_mgr: Rc<RefCell<CtrlMgr>>,
        font_mgr: Rc<RefCell<FontMgr>>,
        active_text_edit: Rc<RefCell<Option<Weak<RefCell<text_edit::TextEdit>>>>>,
    ) -> Self {
        CtrlCtx {
            id_mgr,
            ctrl_mgr,
            font_mgr,
            active_text_edit,
        }
    }

    pub fn id_mgr(&self) -> Rc<RefCell<IdManager>> {
        self.id_mgr.clone()
    }

    pub fn ctrl_mgr(&self) -> Rc<RefCell<CtrlMgr>> {
        self.ctrl_mgr.clone()
    }

    pub fn font_mgr(&self) -> Rc<RefCell<FontMgr>> {
        self.font_mgr.clone()
    }

    pub(crate) fn active_text_edit(
        &self,
    ) -> Rc<RefCell<Option<Weak<RefCell<text_edit::TextEdit>>>>> {
        self.active_text_edit.clone()
    }
}
