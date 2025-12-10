use std::{cell::RefCell, rc::Rc};

use cosmic_text::{
    Family, Font as CosmicFont, FontSystem, Stretch, Style, SwashCache, Weight, fontdb::Query,
};
use fontconfig::Fontconfig;

pub enum DefaultFamily {
    SansSerif,
    Serif,
    Monospace,

    UiFont,
    CodeFont,
    TerminalFont,
}

pub(crate) struct CosmicContext {
    pub(crate) fs: Rc<RefCell<FontSystem>>,
    pub(crate) sc: Rc<RefCell<SwashCache>>,
}

pub struct FontMgr {
    fc: Fontconfig,
    pub(crate) ctx: CosmicContext,
}

impl FontMgr {
    pub(crate) fn new() -> Option<Self> {
        Some(Self {
            fc: Fontconfig::new()?,
            ctx: CosmicContext {
                fs: Rc::new(RefCell::new(FontSystem::new())),
                sc: Rc::new(RefCell::new(SwashCache::new())),
            },
        })
    }

    pub fn load_local_family(&mut self, name: &str) -> Result<(), FontFamilyNotExist> {
        if let Err(err) = self.ctx.fs.borrow_mut().db_mut().load_font_file(name) {
            Err(FontFamilyNotExist)
        } else {
            Ok(())
        }
    }

    pub fn load_system_family(&mut self, name: &str) -> Result<(), FontFamilyNotExist> {
        if let Some(font) = self.fc.find(name, None) {
            self.load_local_family(&font.path.to_str().unwrap())
        } else {
            Err(FontFamilyNotExist)
        }
    }

    pub fn load_default_family(&mut self, family_type: DefaultFamily) -> Self {
        todo!()
    }

    pub fn get_font(&self, name: &str, weight: u16) -> Option<Font> {
        let id = self.ctx.fs.borrow().db().query(&Query {
            families: &[Family::Name(name)],
            weight: Weight(weight),
            stretch: Stretch::Normal,
            style: Style::Normal,
        })?;
        Some(Font {
            data: CosmicFont::new(self.ctx.fs.borrow().db(), id)?,
        })
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct FontFamilyNotExist;

pub struct Font {
    pub(crate) data: CosmicFont,
}

impl Font {
    pub fn name(&self) -> String {
        self.data
            .rustybuzz()
            .names()
            .get(0)
            .unwrap()
            .to_string()
            .unwrap()
    }

    pub fn size(&self) -> u16 {
        self.data.rustybuzz().weight().to_number()
    }
}
