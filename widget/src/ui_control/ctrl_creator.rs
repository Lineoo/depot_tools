use std::{cell::RefCell, rc::Rc};

use crate::id_manager::IdManager;

pub struct CtrlCreator(Rc<RefCell<IdManager>>);

impl CtrlCreator {
    pub(crate) fn new(id_mgr: Rc<RefCell<IdManager>>) -> Self {
        CtrlCreator(id_mgr)
    }

    pub fn id_mgr(&self) -> Rc<RefCell<IdManager>> {
        self.0.clone()
    }
}
