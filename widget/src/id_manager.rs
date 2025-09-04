use std::num::NonZero;

use crate::application::IdType;

pub(crate) struct IdManager {
    next_id: IdType,
    unused_ids: Vec<IdType>,
}

impl IdManager {
    pub fn new() -> Self {
        IdManager {
            next_id: NonZero::new(1).unwrap(),
            unused_ids: Vec::new(),
        }
    }

    pub fn get_id(&mut self) -> IdType {
        if let Some(id) = self.unused_ids.pop() {
            id
        } else {
            let id = self.next_id;
            self.next_id = self.next_id.checked_add(1).expect("Control id overflow!");
            id
        }
    }

    pub fn release_id(&mut self, id: IdType) {
        if id < self.next_id {
            self.unused_ids.push(id);
        }
    }
}
