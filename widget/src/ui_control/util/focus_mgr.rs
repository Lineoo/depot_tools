use indexmap::IndexSet;

use crate::ui_control::control::{Control, WeakHandle};

pub struct FocusMgr {
    chain: IndexSet<WeakHandle<dyn Control>>,
    current_idx: usize,
}

impl FocusMgr {
    pub(crate) fn new() -> Self {
        Self {
            chain: IndexSet::new(),
            current_idx: 0,
        }
    }

    pub fn set_order(&mut self, prev: WeakHandle<dyn Control>, control: WeakHandle<dyn Control>) {
        if !self.chain.contains(&prev) {
            self.chain.insert(prev.clone());
        }
        self.insert_after(prev, control);
    }

    pub fn insert(&mut self, control: WeakHandle<dyn Control>) {
        self.chain.insert(control);
    }

    pub fn insert_after(
        &mut self,
        prev: WeakHandle<dyn Control>,
        control: WeakHandle<dyn Control>,
    ) -> Result<FocusMgrInsertState, FocusMgrInsertFailure> {
        if !self.chain.contains(&prev) {
            return Err(FocusMgrInsertFailure::PrevNotExist);
        }
        let (idx, r) = self
            .chain
            .insert_before(self.chain.get_index_of(&prev).unwrap() + 1, control);
        Ok(if r {
            if idx <= self.current_idx {
                self.current_idx += 1;
            }
            FocusMgrInsertState::InsertSuccess
        } else {
            FocusMgrInsertState::MoveOrder
        })
    }

    pub fn remove(&mut self, control: WeakHandle<dyn Control>) {
        if let Some(idx) = self.chain.get_index_of(&control)
            && idx < self.current_idx
        {
            self.current_idx -= 1;
        }
        self.chain.shift_remove(&control);
    }

    pub fn contains(&self, control: WeakHandle<dyn Control>) -> bool {
        self.chain.contains(&control)
    }

    pub fn query_prev(
        &self,
        control: WeakHandle<dyn Control>,
    ) -> Result<WeakHandle<dyn Control>, FocusMgrQueryFailure> {
        if !self.chain.contains(&control) {
            return Err(FocusMgrQueryFailure::NotExist);
        }
        let idx = self.chain.get_index_of(&control).unwrap();
        return Ok(self.chain[if idx == 0 {
            self.chain.len() - 1
        } else {
            idx - 1
        }]
        .clone());
    }

    pub fn query_next(
        &self,
        control: WeakHandle<dyn Control>,
    ) -> Result<WeakHandle<dyn Control>, FocusMgrQueryFailure> {
        if !self.chain.contains(&control) {
            return Err(FocusMgrQueryFailure::NotExist);
        }
        let idx = self.chain.get_index_of(&control).unwrap();
        return Ok(self.chain[if idx == self.chain.len() - 1 {
            0
        } else {
            idx + 1
        }]
        .clone());
    }

    pub fn prev(&mut self) {
        if self.current_idx == 0 {
            self.current_idx = self.chain.len() - 1;
        } else {
            self.current_idx -= 1;
        }
    }

    pub fn next(&mut self) {
        self.current_idx += 1;
        if self.current_idx == self.chain.len() {
            self.current_idx = 0;
        }
    }

    pub fn current(&self) -> WeakHandle<dyn Control> {
        self.chain[self.current_idx].clone()
    }
}

pub enum FocusMgrQueryFailure {
    NotExist,
}

pub enum FocusMgrInsertFailure {
    PrevNotExist,
}

pub enum FocusMgrInsertState {
    InsertSuccess,
    MoveOrder,
}
