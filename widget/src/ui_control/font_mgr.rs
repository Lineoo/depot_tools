use std::{cell::RefCell, collections::HashMap, path::Path, rc::Rc};

use sdl3::ttf::{Font, Sdl3TtfContext};

pub struct FontMgr {
    fonts: HashMap<String, HashMap<u32, Rc<RefCell<Font<'static>>>>>,
    ctx: Rc<RefCell<Sdl3TtfContext>>,
}

impl FontMgr {
    pub fn new(ctx: Rc<RefCell<Sdl3TtfContext>>) -> Self {
        FontMgr {
            fonts: HashMap::new(),
            ctx,
        }
    }

    pub fn load_local_font(&mut self, path: &Path, size: u32) -> Result<(), sdl3::Error> {
        let font = self.ctx.borrow_mut().load_font(path, size as f32)?;
        let rc_font = Rc::new(RefCell::new(font));
        let font_name = path.file_name().unwrap().to_str().unwrap();
        let name = font_name.strip_suffix(".ttf").unwrap().to_string();
        let family = self.fonts.entry(name).or_default();
        family.insert(size, rc_font);
        Ok(())
    }

    pub fn load_system_font(&self, name: &str, size: f32) -> Result<(), String> {
        todo!();
    }

    pub fn has_family(&self, name: &str) -> bool {
        self.fonts.contains_key(name)
    }

    pub fn get_family_sizes(&self, name: &str) -> Vec<u32> {
        self.fonts
            .get(name)
            .map(|font| font.keys().cloned().collect())
            .unwrap_or_default()
    }

    pub fn get_family_names(&self) -> Vec<String> {
        self.fonts.keys().cloned().collect()
    }

    pub fn clear_fonts(&mut self) {
        self.fonts.clear();
    }

    pub fn remove_family(&mut self, name: &str) {
        self.fonts.remove(name);
    }

    pub fn remove_font(&mut self, name: &str, size: f32) {
        if let Some(family) = self.fonts.get_mut(name) {
            family.remove(&(size as u32));
        }
    }

    pub fn get_font(&self, name: &str, size: u32) -> Option<Rc<RefCell<Font<'static>>>> {
        self.fonts.get(name).and_then(|f| f.get(&size).cloned())
    }
}
