use std::path::Path;

use sdl3::{render::TextureCreator, surface::Surface};

pub struct Image {
    pub(crate) img_data: Surface<'static>,
}

impl Image {
    pub fn from_file(path: &Path) -> Option<Self> {
        use sdl3::image::*;
        let img_data = Surface::from_file(path).ok()?;
        Some(Image { img_data })
    }

    pub fn get_width(&self) -> u32 {
        self.img_data.width()
    }

    pub fn get_height(&self) -> u32 {
        self.img_data.height()
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.img_data.width(), self.img_data.height())
    }
}
