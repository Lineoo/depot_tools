use std::{cell::RefCell, rc::Rc};

use crate::{id_manager::IdManager, ui_control::ctrl_mgr::CtrlMgr};

pub struct CtrlCtx {
    id_mgr: Rc<RefCell<IdManager>>,
    ctrl_mgr: Rc<RefCell<CtrlMgr>>,
}

impl CtrlCtx {
    pub(crate) fn new(id_mgr: Rc<RefCell<IdManager>>, ctrl_mgr: Rc<RefCell<CtrlMgr>>) -> Self {
        CtrlCtx { id_mgr, ctrl_mgr }
    }

    pub fn id_mgr(&self) -> Rc<RefCell<IdManager>> {
        self.id_mgr.clone()
    }

    pub fn ctrl_mgr(&self) -> Rc<RefCell<CtrlMgr>> {
        self.ctrl_mgr.clone()
    }
}
