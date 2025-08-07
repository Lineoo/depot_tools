pub(crate) struct IdManager {
    next_id: u64,
    unused_ids: Vec<u64>,
}

impl IdManager {
    pub fn new() -> Self {
        IdManager {
            next_id: 0,
            unused_ids: Vec::new(),
        }
    }

    pub fn get_id(&mut self) -> u64 {
        if let Some(id) = self.unused_ids.pop() {
            id
        } else {
            let id = self.next_id;
            self.next_id += 1;
            id
        }
    }
    
    pub fn release_id(&mut self, id: u64) {
        if id < self.next_id {
            self.unused_ids.push(id);
        }
    }
}
