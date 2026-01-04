use crate::entry::{ActiveEntry, EntryArgs, Invoke, Read};
use mlua::{Error as LuaError, Lua, Table, Value};
use std::{fs, path::Path};

/// A single Lua-backed entry. It owns a Lua VM and the entry table/function
/// result. Use `from_script` to create it from script source.
pub struct LuaEntry {
    lua: Lua,
    table: Table,
}

impl LuaEntry {
    /// Create a `LuaEntry` from Lua script source.
    ///
    /// The script is executed in a fresh `Lua` VM. The function expects the
    /// script to populate a global `module` table with an `entry` field
    /// (either a table or a function returning a table).
    pub fn from_script(script: &str) -> mlua::Result<Self> {
        let lua = Lua::new();
        lua.load(script).exec()?;

        let globals = lua.globals();
        let val: Value = globals.get("module")?;

        // module should be a table. From that table, get `entry` field which
        // may be a table or a function returning a table.
        let Value::Table(module_table) = val else {
            return Err(LuaError::RuntimeError(
                "no global module table found".into(),
            ));
        };

        let entry_val: Value = module_table.get("entry")?;
        let table = match entry_val {
            Value::Table(t) => t,
            Value::Function(func) => match func.call(())? {
                Value::Table(t) => t,
                _ => {
                    return Err(LuaError::RuntimeError(
                        "module.entry() did not return a table".into(),
                    ));
                }
            },
            _ => {
                return Err(LuaError::RuntimeError(
                    "module.entry not found or invalid".into(),
                ));
            }
        };

        Ok(Self { lua, table })
    }

    /// Convenience: create from a file path.
    pub fn from_script_file(path: &Path) -> mlua::Result<Self> {
        let s = fs::read_to_string(path).map_err(LuaError::external)?;
        Self::from_script(&s)
    }
}

impl ActiveEntry for LuaEntry {
    fn push(&mut self, args: EntryArgs) {
        if let Ok(Value::Function(f)) = self.table.get::<Value>("push") {
            let _ = f.call::<()>(args);
        }
    }

    fn read(&self, index: usize) -> Option<Read> {
        if let Ok(Value::Function(f)) = self.table.get::<Value>("read") {
            match f.call::<Value>(index) {
                Ok(Value::Table(t)) => {
                    let title = t.get("title").unwrap_or_default();
                    let description = t.get("description").unwrap_or_default();
                    Some(Read { title, description })
                }
                Ok(Value::String(s)) => {
                    let title = s.to_str().map(|s| s.to_string()).unwrap_or_default();
                    let description = "".into();
                    Some(Read { title, description })
                }
                _ => None,
            }
        } else {
            None
        }
    }

    fn call(&self, index: usize) -> Option<Invoke> {
        if let Ok(Value::Function(f)) = self.table.get::<Value>("call") {
            match f.call::<Value>(index) {
                Ok(Value::String(s)) => match &*s.to_str().ok()? {
                    "leave" => Some(Invoke::Leave),
                    "exit" => Some(Invoke::Exit),
                    "remain" => Some(Invoke::Remain),
                    _ => Some(Invoke::Remain),
                },
                Ok(Value::Table(t)) => Some(Invoke::Raise(Box::new(LuaEntry {
                    lua: self.lua.clone(),
                    table: t.clone(),
                }))),
                _ => None,
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_example() {
        let entry = LuaEntry::from_script(include_str!("../../examples/lua_example.lua"))
            .expect("from_script_file");
        let read = entry.read(1).expect("read returned");
        assert_eq!(read.title, "Hello, World!");
    }
}
