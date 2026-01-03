use std::{cell::RefCell, rc::Rc};

use cosmic_text::{
    Family, Font as CosmicFont, FontSystem, Stretch, Style, SwashCache, Weight, fontdb::Query,
};
use fontconfig::{Fontconfig, Pattern};

pub enum DefaultFamily {
    SansSerif,
    Serif,
    Monospace,

    UiFont,
    CodeFont,
    TerminalFont,
}

impl DefaultFamily {
    fn get_fc_family_name(&self) -> &'static str {
        match self {
            Self::SansSerif | Self::UiFont => "sans-serif",
            Self::Serif => "serif",
            Self::Monospace | Self::CodeFont | Self::TerminalFont => "monospace",
        }
    }
}

pub(crate) struct CosmicContext {
    pub(crate) fs: Rc<RefCell<FontSystem>>,
    pub(crate) sc: Rc<RefCell<SwashCache>>,
}

pub struct FontMgr {
    fc: Fontconfig,
    pub(crate) ctx: CosmicContext,
    default_table: DefaultFontTable,
}

impl FontMgr {
    pub(crate) fn new() -> Option<Self> {
        Some(Self {
            fc: Fontconfig::new()?,
            ctx: CosmicContext {
                fs: Rc::new(RefCell::new(FontSystem::new())),
                sc: Rc::new(RefCell::new(SwashCache::new())),
            },
            default_table: DefaultFontTable::default(),
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

    pub fn load_default_family(mut self, family_type: DefaultFamily) -> Self {
        let font_name;
        {
            let font_list = fontconfig::list_fonts(&Pattern::new(&self.fc), None);
            let font = &font_list
                .iter()
                .find(|p| {
                    p.get_string(c"genericfamily").unwrap() == family_type.get_fc_family_name()
                })
                .unwrap();
            font_name = font.name().unwrap().to_string();
        }
        self.load_system_family(&font_name).unwrap();
        self
    }

    pub fn get_font_by_name(&self, name: &str, weight: u16) -> Option<Font> {
        let id = self.ctx.fs.borrow().db().query(&Query {
            families: &[Family::Name(name)],
            weight: Weight(weight),
            stretch: Stretch::Normal,
            style: Style::Normal,
        })?;
        Some(Font {
            data: Rc::new(RefCell::new(CosmicFont::new(
                self.ctx.fs.borrow().db(),
                id,
            )?)),
        })
    }

    pub fn set_default_family(
        &mut self,
        family_type: DefaultFamily,
        family: String,
    ) -> Option<String> {
        let family_ref = match family_type {
            DefaultFamily::SansSerif => &mut self.default_table.sans_serif,
            DefaultFamily::Serif => &mut self.default_table.serif,
            DefaultFamily::Monospace => &mut self.default_table.monospace,
            DefaultFamily::UiFont => &mut self.default_table.ui_font,
            DefaultFamily::CodeFont => &mut self.default_table.code_font,
            DefaultFamily::TerminalFont => &mut self.default_table.terminal_font,
        };
        let result = family_ref.clone();
        *family_ref = Some(family);
        result
    }

    pub fn get_default_font(&self, family: DefaultFamily, weight: u16) -> Option<Font> {
        self.get_font_by_name(
            match family {
                DefaultFamily::SansSerif => self.default_table.sans_serif.as_ref(),
                DefaultFamily::Serif => self.default_table.serif.as_ref(),
                DefaultFamily::Monospace => self.default_table.monospace.as_ref(),
                DefaultFamily::UiFont => self.default_table.ui_font.as_ref(),
                DefaultFamily::CodeFont => self.default_table.code_font.as_ref(),
                DefaultFamily::TerminalFont => self.default_table.terminal_font.as_ref(),
            }?,
            weight,
        )
    }
}

#[derive(Clone, Debug, Default)]
struct DefaultFontTable {
    sans_serif: Option<String>,
    serif: Option<String>,
    monospace: Option<String>,

    ui_font: Option<String>,
    code_font: Option<String>,
    terminal_font: Option<String>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct FontFamilyNotExist;

#[derive(Clone, Debug)]
pub struct Font {
    pub(crate) data: Rc<RefCell<CosmicFont>>,
}

impl Font {
    pub fn name(&self) -> String {
        self.data
            .borrow()
            .rustybuzz()
            .names()
            .get(0)
            .unwrap()
            .to_string()
            .unwrap()
    }

    pub fn size(&self) -> u16 {
        self.data.borrow().rustybuzz().weight().to_number()
    }
}
