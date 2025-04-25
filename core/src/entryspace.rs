use std::sync::Arc;

use log::*;
use nucleo::{
    Config, Nucleo, Utf32String,
    pattern::{CaseMatching, Normalization},
};

use crate::entry::*;

struct EntryFetch {
    activate: fn() -> BoxedEntry,
    description: Description,
}

/// The main search engine
pub struct EntrySpace {
    engine: Nucleo<EntryFetch>,
}
impl EntrySpace {
    fn new(list: Vec<(String, EntryFetch)>) -> Self {
        let config = Config::DEFAULT;
        let engine = Nucleo::new(config, Arc::new(|| ()), None, 1);

        let injector = engine.injector();
        for (key, fetch) in list {
            if !key.is_ascii() {
                warn!(
                    "Non-ascii key: \"{}\"! Only valid ascii strings are accepted.",
                    key
                );
                continue;
            }
            injector.push(fetch, |_, slice| {
                slice[0] = Utf32String::Ascii(key.into());
            });
        }

        Self { engine }
    }
}
impl ActiveEntry for EntrySpace {
    fn push(&mut self, args: EntryArgs) {
        // TODO: auto-trigger
        // TODO: supports `append` param in reparse(..) for better performance
        self.engine
            .pattern
            .reparse(0, &args, CaseMatching::Ignore, Normalization::Smart, false);
    }
    fn read(&self, index: usize) -> Option<Description> {
        self.engine
            .snapshot()
            .get_matched_item(index.try_into().unwrap())
            .map(|fetch| fetch.data.description.clone())
    }
    fn call(&self, index: usize) -> Option<Invoke> {
        self.engine
            .snapshot()
            .get_matched_item(index.try_into().unwrap())
            .map(|item| Invoke::Raise((item.data.activate)()))
    }
}
