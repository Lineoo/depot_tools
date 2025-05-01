use std::sync::Arc;

use log::*;
use nucleo::{
    Config, Nucleo, Utf32String,
    pattern::{CaseMatching, Normalization},
};

use crate::entry::*;

pub struct EntryFetch {
    pub activate: fn() -> Invoke,
    pub description: Description,
}

/// The main search engine
pub struct EntrySpace {
    engine: Nucleo<EntryFetch>,
}
impl EntrySpace {
    pub fn new(list: Vec<(String, EntryFetch)>) -> Self {
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
    pub fn from_toml(text: String) -> Option<Self> {
        let config = Config::DEFAULT;
        let engine = Nucleo::new(config, Arc::new(|| ()), None, 1);

        let toml = toml::Table::try_from(text).ok()?;
        let entries = toml.get("entry").and_then(|x| x.as_array())?;

        let injector = engine.injector();
        for each in entries {
            let Some(key) = each.get("name").and_then(|x| x.as_str()) else {
                continue;
            };
            if !key.is_ascii() {
                warn!(
                    "Non-ascii key: \"{}\"! Only valid ascii strings are accepted.",
                    key
                );
                continue;
            }
            let fetch = EntryFetch {
                activate: || todo!(),
                description: Description {
                    title: each
                        .get("title")
                        .and_then(|x| x.as_str())
                        .unwrap_or("<No title>")
                        .into(),
                    information: each
                        .get("information")
                        .and_then(|x| x.as_str())
                        .unwrap_or_default()
                        .into(),
                }
            };
            injector.push(fetch, |_, slice| {
                slice[0] = Utf32String::Ascii(key.into());
            });
        }
        Some(Self { engine })
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
        assert_eq!(entryspace.read(0).map(|x| x.title), Some("Ubuntu".into()));

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
                    }
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
                    }
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
                    }
                },
            ),
        ])
    }
}
