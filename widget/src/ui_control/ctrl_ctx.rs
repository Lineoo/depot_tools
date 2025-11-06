use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    sync::{Arc, Mutex},
};

use crate::{
    id_manager::IdManager,
    ui_control::{
        ctrl_mgr::CtrlMgr,
        font::FontMgr,
        util::{focus_mgr::FocusMgr, text_edit},
    },
};

pub struct CtrlCtx {
    id_mgr: Rc<RefCell<IdManager>>,
    ctrl_mgr: Rc<RefCell<CtrlMgr>>,
    font_mgr: Rc<RefCell<FontMgr>>,
    focus_mgr: Arc<Mutex<FocusMgr>>,
}

impl CtrlCtx {
    pub(crate) fn new(
        id_mgr: Rc<RefCell<IdManager>>,
        ctrl_mgr: Rc<RefCell<CtrlMgr>>,
        font_mgr: Rc<RefCell<FontMgr>>,
        focus_mgr: Arc<Mutex<FocusMgr>>,
    ) -> Self {
        CtrlCtx {
            id_mgr,
            ctrl_mgr,
            font_mgr,
            focus_mgr,
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
}
