use std::{cell::RefCell, rc::Rc};

use sdl3::{
    pixels::{Color, PixelFormatEnum},
    rect::Rect as SdlRect,
    render::{Canvas, TextureCreator},
    surface::Surface,
    video::WindowContext,
};

use crate::paint::shapes::Rect;

pub struct Painter {
    pub(crate) canvas: Canvas<Surface<'static>>,
    creator: Rc<RefCell<TextureCreator<WindowContext>>>,
    size: (u32, u32),
}

impl Painter {
    pub fn new(size: (u32, u32), creator: Rc<RefCell<TextureCreator<WindowContext>>>) -> Self {
        let surface = Surface::new(size.0, size.1, PixelFormatEnum::RGBA8888.into())
            .expect("Could not create surface");
        Painter {
            canvas: surface.into_canvas().expect("Could not create canvas"),
            creator,
            size,
        }
    }

    pub fn rect(&mut self, rect: Rect) {
        let rect: SdlRect = rect.try_into().unwrap();
        self.canvas.fill_rect(Some(rect.into()));
    }

    pub fn text(&mut self, _text: &str, _x: u32, _y: u32) {
        // Implementation for painting text
    }

    pub fn set_color(&mut self, color: Color) {
        // Implementation for setting background color
        self.canvas.set_draw_color(color);
    }

    pub fn set_font_size(&mut self, w: f32, h: f32) {}

    pub fn copy(&mut self, source: Painter, source_area: Rect, dest_area: Rect) {
        let source_area: SdlRect = source_area.try_into().unwrap();
        let dest_area: SdlRect = dest_area.try_into().unwrap();
        let creator = self.creator.borrow_mut();
        let texture = creator
            .create_texture(
                Some(PixelFormatEnum::RGBA8888.into()),
                sdl3::render::TextureAccess::Static,
                source.size.0,
                source.size.1,
            )
            .unwrap();
        self.canvas.copy(&texture, source_area, dest_area);
    }
}
