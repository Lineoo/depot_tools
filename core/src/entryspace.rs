use std::sync::Arc;

use log::*;
use nucleo::{
    Config, Nucleo, Utf32String,
    pattern::{CaseMatching, Normalization},
};
use uuid::Uuid;

use crate::entry::*;

struct EntryFetch {
    activate: fn() -> Invoke,
    description: Description,
    uuid: Uuid,
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
    fn fetch(&self, index: usize) -> Option<&EntryFetch> {
        self.engine
            .snapshot()
            .get_matched_item(index.try_into().unwrap())
            .map(|fetch| fetch.data)
    }
}
impl ActiveEntry for EntrySpace {
    fn push(&mut self, args: EntryArgs) {
        // TODO: auto-trigger
        // TODO: supports `append` param in reparse(..) for better performance
        self.engine
            .pattern
            .reparse(0, &args, CaseMatching::Ignore, Normalization::Smart, false);
        self.engine.tick(10);
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
            .map(|item| (item.data.activate)())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn entryspace() {
        let mut entryspace = test_entries();

        entryspace.push("ubutu".into());
        assert_eq!(
            entryspace.fetch(0).map(|x| x.uuid),
            Some(Uuid::from_u128(0x1e1e3d3d_b1b2_c1c2_d1d2d_3d4d5d6d7d8)),
        );

        entryspace.push("fami".into());
        assert_eq!(entryspace.read(0).map(|x| x.title), Some("Windows".into()));
    }

    fn test_entries() -> EntrySpace {
        EntrySpace::new(vec![
            (
                "linux debian".into(),
                EntryFetch {
                    activate: || {
                        println!("debian");
                        Invoke::Exit
                    },
                    description: Description {
                        title: "Debian".into(),
                        information: "Boot Debian".into(),
                    },
                    uuid: Uuid::from_u128(0x1e1e2d2d_b1b2_c1c2_d1d2d_3d4d5d6d7d8),
                },
            ),
            (
                "ubuntu linux debian".into(),
                EntryFetch {
                    activate: || {
                        println!("ubuntu");
                        Invoke::Exit
                    },
                    description: Description {
                        title: "Ubuntu".into(),
                        information: "Boot Ubuntu".into(),
                    },
                    uuid: Uuid::from_u128(0x1e1e3d3d_b1b2_c1c2_d1d2d_3d4d5d6d7d8),
                },
            ),
            (
                "windows 11 family".into(),
                EntryFetch {
                    activate: || {
                        println!("windows");
                        Invoke::Exit
                    },
                    description: Description {
                        title: "Windows".into(),
                        information: "Boot Windows 11 Family Edition".into(),
                    },
                    uuid: Uuid::from_u128(0x1e1e4d4d_b1b2_c1c2_d1d2d_3d4d5d6d7d8),
                },
            ),
        ])
    }
}
