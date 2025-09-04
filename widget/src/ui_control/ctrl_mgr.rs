use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    application::IdType,
    id_manager::IdManager,
    ui_control::control::{Control, Insertable},
};

pub struct CtrlMgr {
    ctrls: HashMap<IdType, Rc<RefCell<dyn Control>>>,
    pub id_mgr: Rc<RefCell<IdManager>>,
}

impl CtrlMgr {
    pub fn new(id_mgr: Rc<RefCell<IdManager>>) -> Self {
        CtrlMgr {
            ctrls: HashMap::new(),
            id_mgr,
        }
    }

    pub fn add_ctrl<C: Insertable>(&mut self, mut ctrl: C) {
        ctrl.insert_tree(self);
    }

    pub fn insert_item(&mut self, ctrl: Rc<RefCell<dyn Control>>) {
        let id = ctrl.borrow().id();
        self.ctrls.insert(id, ctrl);
    }
}
