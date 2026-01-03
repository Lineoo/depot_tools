use std::{cell::RefCell, rc::Rc};

use sdl3::{render::TextureCreator, video::WindowContext};

use crate::{control::font::FontMgr, paint::painter::Painter};

#[derive(Clone)]
pub struct PainterCreator {
    texture_creator: Rc<RefCell<TextureCreator<WindowContext>>>,
    font_mgr: Rc<RefCell<FontMgr>>,
}

impl PainterCreator {
    pub(crate) fn new(
        texture_creator: Rc<RefCell<TextureCreator<WindowContext>>>,
        font_mgr: Rc<RefCell<FontMgr>>,
    ) -> Self {
        Self {
            texture_creator,
            font_mgr,
        }
    }
    pub fn create(&self, size: (u32, u32)) -> Painter {
        Painter::new(size, self.texture_creator.clone(), self.font_mgr.clone())
    }
}
