use indexmap::IndexSet;

use crate::ui_control::control::{self, Control, WeakHandle};

pub struct FocusMgr {
    chain: IndexSet<WeakHandle<dyn Control>>,
}

impl FocusMgr {
    pub(crate) fn new() -> Self {
        Self {
            chain: IndexSet::new(),
        }
    }

    pub fn set_order(&mut self, former: WeakHandle<dyn Control>, control: WeakHandle<dyn Control>) {
        if (!self.chain.contains(&former)) {
            self.chain.insert(former);
        }
    }

    pub fn insert(control: WeakHandle<dyn Control>) {}

    pub fn insert_after(former: WeakHandle<dyn Control>, control: WeakHandle<dyn Control>) {}

    pub fn remove(control: WeakHandle<dyn Control>) {}

    pub fn contains(control: WeakHandle<dyn Control>) -> bool {}

    pub fn query_former(
        control: WeakHandle<dyn Control>,
    ) -> Result<(WeakHandle<dyn Control>), FocusMgrQueryFailure> {
    }

    pub fn query_latter(
        control: WeakHandle<dyn Control>,
    ) -> Result<(WeakHandle<dyn Control>), FocusMgrQueryFailure> {
    }
}

pub enum FocusMgrQueryFailure {
    NotExist,
    NoTargetHandle,
}
