use std::{process::Command, str::FromStr, sync::Arc};

use hashbrown::HashMap;
use log::*;

use crate::{dylib::DylibEntry, entry::*, search::SearchEngine};

type Ident = u64;

/// Single fetch of an entry.
#[derive(Clone)]
pub struct Entry {
    pub invoke: Arc<dyn Fn() -> Invoke + Send + Sync>,
    pub raise: Arc<dyn Fn() -> BoxedEntry + Send + Sync>,
    pub read: Read,
    pub ident: Ident,
}

/// The core component for the entries' access. EntrySpace allows storing a collection of [`Entry`]s
/// and specific their behavior for use, and contains a lot of utilities like searching and auto-trigger.
///
/// # Initialize
/// See [`EntrySpace::from_toml`] for details.
///
/// # Usage
/// ### toml configuration
/// Initialize `EntrySpace` from toml file with [`from_toml`](EntrySpace::from_toml) method.
/// See [`EntrySpace::from_toml`] for details.
///
/// ### fuzzy search
/// Use [`Nucleo`] as the fuzzy search engine.
///
/// ### auto trigger
/// You can register an `trigger` in config file like this:
/// ```toml
/// [[entry]]
/// trigger = ":calc"
/// crate-type = "dylib"
/// path = "plugins/calc_helper"
/// ```
/// And when typed `:calc 2 + 1`, entryspace will automatically
/// call calculator with arguments `2 + 1`
pub struct EntrySpace {
    /// The core fuzzy search engine used by depot
    engine: SearchEngine<Entry>,
    triggers: HashMap<String, Entry>,
    active_trigger: Option<(Ident, BoxedEntry)>,
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
    /// # `entry` contains all entries.
    /// [[entry]]               
    /// name = "fruit orange"   # The `search name` used by engine
    /// title = "Orange"        # Title to display
    /// description = "This is an orange. It's orange." # The description, typically next to title
    /// trigger = ":og"         # Trigger prefix
    /// entry-type = "shell"    # Type of this entry, a key defined in `loader`
    /// exec = "orange"         # Other arguments required by `entry-type`
    /// args = ["init"]
    ///
    /// # TODO: `loader` table defines custom loaders
    /// # builtin loader: dylib, *cdylib, shell, lua
    /// [loader.fruit]
    /// base = "rust"           # Available variables: *rust, *lua, *bin
    /// path = "fruit.dll"      # Path to loader library
    /// ```
    pub fn from_toml(text: String) -> Option<Self> {
        let mut engine = SearchEngine::new();
        let mut triggers = HashMap::new();

        // Init toml
        let toml = toml::Table::from_str(&text).ok()?;

        // Load [[entry]] array
        // Builtin key: name, title, description, crate-type, trigger
        for each in toml.get("entry").and_then(|x| x.as_array())? {
            // Prepare the loader
            let invoke: Arc<dyn Fn() -> Invoke + Send + Sync>;
            let raise: Arc<dyn Fn() -> BoxedEntry + Send + Sync>;
            match each.get("crate-type").and_then(|x| x.as_str()) {
                Some("shell") => {
                    let exec = each.get("exec").and_then(|x| x.as_str());
                    let args = each.get("args").and_then(|x| x.as_array());
                    if let Some(exec) = exec {
                        let exec = exec.to_string();
                        let args = args
                            .into_iter()
                            .flat_map(|x| {
                                x.iter().filter_map(|x| x.as_str()).map(|x| x.to_string())
                            })
                            .collect::<Vec<_>>();
                        invoke = Arc::new(move || {
                            Command::new(&exec)
                                .args(args.iter())
                                .spawn()
                                .expect("failed to start shell command")
                                .wait()
                                .expect("error");
                            Invoke::Exit
                        });
                        raise = Arc::new(|| Box::new("No raise-support for shell entry"));
                    } else {
                        invoke =
                            Arc::new(|| Invoke::Raise(Box::new("Shell entry lack of `exec` key!")));
                        raise = Arc::new(|| Box::new("Shell entry lack of `exec` key!"));
                    }
                }
                Some("dylib") => {
                    if let Some(path) = each.get("path").and_then(|x| x.as_str()) {
                        // loading lib
                        let path = path.to_string();
                        let action = move || -> Result<BoxedEntry, Box<dyn std::error::Error>> {
                            use libloading::*;

                            // TODO: User Confirmation
                            // Safety: No. No safety at all. That depends on users.
                            unsafe {
                                let lib = Library::new(&path)?;
                                let init = lib.get::<unsafe extern "C" fn()>(b"init")?;
                                init();
                                Ok(Box::new(DylibEntry { lib }))
                            }
                        };
                        // error handing
                        let action = move || match action() {
                            Ok(entry) => entry,
                            Err(e) => Box::new(e.to_string()),
                        };
                        let action = Arc::new(action);
                        raise = action.clone();
                        invoke = Arc::new(move || Invoke::Raise(action()));
                    } else {
                        invoke =
                            Arc::new(|| Invoke::Raise(Box::new("Dylib entry lack of `path` key!")));
                        raise = Arc::new(|| Box::new("Dylib entry lack of `path` key!"));
                    }
                }
                Some("lua") => {
                    if let Some(path) = each.get("path").and_then(|x| x.as_str()) {
                        // loading lib
                        let path = path.to_string();

                        let action = move || -> Result<BoxedEntry, Box<dyn std::error::Error>> {
                            // TODO: User Confirmation
                            // TODO: Share lua Vms
                            let vm = mlua::Lua::new();
                            vm.load("").exec();

                            todo!()
                        };
                        // error handing
                        let action = move || match action() {
                            Ok(entry) => entry,
                            Err(e) => Box::new(e.to_string()),
                        };
                        let action = Arc::new(action);
                        raise = action.clone();
                        invoke = Arc::new(move || Invoke::Raise(action()));
                    } else {
                        invoke =
                            Arc::new(|| Invoke::Raise(Box::new("Lua entry lack of `path` key!")));
                        raise = Arc::new(|| Box::new("Lua entry lack of `path` key!"));
                    }
                }
                Some(_) => {
                    invoke = Arc::new(|| Invoke::Raise(Box::new("Unrecognized crate-type!")));
                    raise = Arc::new(|| Box::new("Unrecognized crate-type!"));
                }
                None => {
                    invoke = Arc::new(|| Invoke::Raise(Box::new("No valid crate-type key!")));
                    raise = Arc::new(|| Box::new("No valid crate-type key!"));
                }
            };

            let entry = Entry {
                invoke,
                raise,
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
                ident: getrandom::u64().ok()?,
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
                    engine.push(name.to_string(), entry);
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
        if let Some((prefix, args)) = args.split_once(' ')
            && let Some(trigger) = self.triggers.get(prefix)
        {
            match self.active_trigger.as_mut() {
                // The same trigger
                Some(active) if active.0 == trigger.ident => {
                    active.1.push(args.to_string());
                }
                // A different trigger or no trigger
                Some(_) | None => {
                    let active = (trigger.raise)();
                    self.active_trigger.replace((trigger.ident, active));
                }
            }
            return;
        }

        self.active_trigger.take();
        self.engine.search(&args);
    }
    fn read(&self, index: usize) -> Option<Read> {
        if let Some(active) = &self.active_trigger {
            return active.1.read(index);
        }

        self.engine.get(index).map(|fetch| fetch.read.clone())
    }
    fn call(&self, index: usize) -> Option<Invoke> {
        if let Some(active) = &self.active_trigger {
            return active.1.call(index);
        }

        self.engine.get(index).map(|item| (item.invoke)())
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
