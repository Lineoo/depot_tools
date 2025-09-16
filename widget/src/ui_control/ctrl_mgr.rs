use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    application::IdType,
    id_manager::IdManager,
    ui_control::{
        control::{Control, Handle, Insertable},
        ctrl_creator::CtrlCreator,
    },
};

pub struct CtrlMgr {
    ctrls: HashMap<IdType, Rc<RefCell<dyn Control>>>,
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
        let id = ctrl.borrow().id();
        self.ctrls.insert(id, ctrl);
    }

    pub fn is_valid_id(&self, id: IdType) -> bool {
        self.ctrls.contains_key(&id)
    }

    pub fn get_ctrl(&self, id: IdType) -> Option<Handle<dyn Control>> {
        Some(self.ctrls.get(&id)?.clone())
    }
}

pub(crate) fn make_creator(
    id_mgr: Rc<RefCell<IdManager>>,
    ctrl_mgr: Handle<CtrlMgr>,
) -> CtrlCreator {
    CtrlCreator::new(id_mgr, ctrl_mgr)
}
