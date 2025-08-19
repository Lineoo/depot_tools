use std::sync::Arc;

use depot_core::{entryspace::EntrySpace, stack::Stack};
use parking_lot::Mutex;

pub struct KitCore {
    stack: Stack,
    selector: usize,
}
impl KitCore {
    pub fn new() -> Arc<Mutex<KitCore>> {
        let config = String::from(include_str!("../examples/config.toml"));
        let entryspace = EntrySpace::from_toml(config).unwrap();

        Arc::new(Mutex::new(KitCore {
            stack: Stack::new(Box::new(entryspace)),
            selector: 0,
        }))
    }
    pub fn selector_up(&mut self) {
        self.selector = self.selector.saturating_add(1);
    }
    pub fn selector_down(&mut self) {
        self.selector = self.selector.saturating_sub(1);
    }
    pub fn stack(&mut self) -> &mut Stack {
        &mut self.stack
    }
}
