use std::{cell::RefCell, rc::Rc};

use crate::{id_manager::IdManager, ui_control::ctrl_mgr::CtrlMgr};

pub struct CtrlCreator {
    id_mgr: Rc<RefCell<IdManager>>,
    ctrl_mgr: Rc<CtrlMgr>,
}

impl CtrlCreator {
    pub(crate) fn new(id_mgr: Rc<RefCell<IdManager>>, ctrl_mgr: Rc<CtrlMgr>) -> Self {
        CtrlCreator { id_mgr, ctrl_mgr }
    }

    pub fn id_mgr(&self) -> Rc<RefCell<IdManager>> {
        self.id_mgr.clone()
    }

    pub fn ctrl_mgr(&self) -> Rc<CtrlMgr> {
        self.ctrl_mgr.clone()
    }
}
