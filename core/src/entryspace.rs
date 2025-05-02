use std::{str::FromStr, sync::Arc};

use hashbrown::HashMap;
use log::*;
use nucleo::{
    Config, Nucleo, Utf32String,
    pattern::{CaseMatching, Normalization},
};
use uuid::Uuid;

use crate::entry::*;

/// Single fetch of an entry.
#[derive(Clone)]
pub struct Entry {
    pub invoke: fn() -> Invoke,
    pub read: Read,
    pub uuid: Uuid,
}

/// The main search engine
/// # Initialize
/// See [`EntrySpace::from_toml`] for details.
/// # Features
/// ### fuzzy search
/// Use [`Nucleo`] as the fuzzy search engine.
/// ### auto trigger
/// You can register an `trigger` in config file like this:
/// ```toml
/// [[entry]]
/// auto_trigger = ":calc"
/// crate-type = "dylib"
/// path = "plugins/calc_helper"
/// ```
/// And when typed `:calc 2 + 1`, entryspace will automatically
/// call calculator with arguments `2 + 1`
pub struct EntrySpace {
    /// The core fuzzy search engine used by depot
    engine: Nucleo<Entry>,
    triggers: HashMap<String, Entry>,
    active_trigger: Option<(Uuid, BoxedEntry)>,
}
impl EntrySpace {
    // pub fn new(list: Vec<(String, EntryFetch)>) -> Self {
    //     let config = Config::DEFAULT;
    //     let engine = Nucleo::new(config, Arc::new(|| ()), None, 1);
    //     let entries = Vec::new();
    //     let injector = engine.injector();
    //     for (key, fetch) in list {
    //         if !key.is_ascii() {
    //             warn!(
    //                 "Non-ascii key: \"{}\"! Only valid ascii strings are accepted.",
    //                 key
    //             );
    //             continue;
    //         }
    //         injector.push(fetch, |_, slice| {
    //             slice[0] = Utf32String::Ascii(key.into());
    //         });
    //     }
    //     Self {
    //         engine,
    //         entries: todo!(),
    //         triggers: todo!(),
    //         active_trigger: todo!(),
    //     }
    // }

    /// Return [`EntrySpace`] from toml-formatted string.
    /// # Example
    /// ``` toml
    /// # all entries are stored in array `entry`
    /// [[entry]]
    /// # The `search name` used by engine
    /// name = "fruit orange"
    /// # Title to display
    /// title = "Orange"
    /// # The description, usually next to title
    /// description = "This is an orange. It's orange."
    /// # Trigger prefix
    /// trigger = ":og"
    /// # Type of this entry
    /// entry-type = "shell"
    /// # Other arguments required by `entry-type`
    /// exec = "orange init"
    /// ```
    pub fn from_toml(text: String) -> Option<Self> {
        let config = Config::DEFAULT;
        let engine = Nucleo::new(config, Arc::new(|| ()), None, 1);
        let mut triggers = HashMap::new();

        // Init toml
        let toml = toml::Table::from_str(&text).ok()?;

        // Load [[entry]] array
        // Builtin key: name, title, description, crate-type, trigger
        let injector = engine.injector();
        for each in toml.get("entry").and_then(|x| x.as_array())? {
            let entry = Entry {
                invoke: || Invoke::Update("TODO".into()),
                read: Read {
                    title: each
                        .get("title")
                        .and_then(|x| x.as_str())
                        .unwrap_or("<No title>")
                        .into(),
                    description: each
                        .get("description")
                        .and_then(|x| x.as_str())
                        .unwrap_or_default()
                        .into(),
                },
                uuid: Uuid::new_v4(),
            };

            // Auto Trigger
            if let Some(prefix) = each.get("trigger").and_then(|x| x.as_str()) {
                if prefix.contains(' ') {
                    warn!(
                        "Trigger prefix \"{prefix}\" contains whitespace, \
                        which will prevent it from normally functioning."
                    )
                }
                if triggers.insert(prefix.to_string(), entry.clone()).is_some() {
                    warn!("An trigger conflict occurs on: {prefix}")
                }
            }

            // Normal entry that displays
            if let Some(name) = each.get("name").and_then(|x| x.as_str()) {
                if !name.is_ascii() {
                    warn!(
                        "Non-ascii key: \"{name}\"! \
                        Only valid ascii strings are accepted for entry name.",
                    );
                } else {
                    injector.push(entry, |_, slice| {
                        slice[0] = Utf32String::Ascii(name.into());
                    });
                }
            };
        }

        Some(Self {
            engine,
            triggers,
            active_trigger: None,
        })
    }
}
impl ActiveEntry for EntrySpace {
    fn push(&mut self, args: EntryArgs) {
        // Auto Trigger
        if let Some((prefix, args)) = args.split_once(' ') {
            if let Some(trigger) = self.triggers.get(prefix) {
                match self.active_trigger.as_mut() {
                    // The same trigger
                    Some(active) if active.0 == trigger.uuid => {
                        active.1.push(args.to_string());
                    }
                    // A different trigger or no trigger
                    Some(_) | None => {
                        let active = match (trigger.invoke)() {
                            Invoke::Raise(active) => active,
                            _ => Box::new(
                                "[ERR] This trigger is registered on an entry \
                                that doesn't accept arguments",
                            ),
                        };
                        self.active_trigger.replace((trigger.uuid, active));
                    }
                }
                return;
            } else {
                self.active_trigger.take();
            }
        }

        // TODO: supports `append` param in reparse(..) for better performance
        self.engine
            .pattern
            .reparse(0, &args, CaseMatching::Ignore, Normalization::Smart, false);
        self.engine.tick(10);
    }
    fn read(&self, index: usize) -> Option<Read> {
        if let Some(active) = &self.active_trigger {
            return active.1.read(index);
        }

        self.engine
            .snapshot()
            .get_matched_item(index.try_into().unwrap())
            .map(|fetch| fetch.data.read.clone())
    }
    fn call(&self, index: usize) -> Option<Invoke> {
        if let Some(active) = &self.active_trigger {
            return active.1.call(index);
        }

        self.engine
            .snapshot()
            .get_matched_item(index.try_into().unwrap())
            .map(|item| (item.data.invoke)())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basis() {
        const TEST_STR: &str = r#"
[[entry]]
name = "orange fruit"
title = "Orange"
[[entry]]
name = "apple red fruit"
title = "Apple"
"#;
        let mut entryspace = EntrySpace::from_toml(String::from(TEST_STR)).unwrap();

        entryspace.push("orn".into());
        assert_eq!(entryspace.read(0).map(|x| x.title), Some("Orange".into()));

        entryspace.push("apl".into());
        assert_eq!(entryspace.read(0).map(|x| x.title), Some("Apple".into()));
    }
}
