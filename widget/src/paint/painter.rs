use std::{cell::RefCell, rc::Rc};

use cosmic_text::{Buffer as CosmicBuffer, Metrics as CosmicMetrics};
use sdl3::{
    pixels::{Color, PixelFormat},
    rect::Rect as SdlRect,
    render::{Canvas, TextureCreator},
    surface::Surface,
    video::WindowContext,
};

use crate::{
    paint::shapes::Rect,
    ui_control::font::{Font, FontMgr},
};

pub struct Painter {
    pub(crate) canvas: Canvas<Surface<'static>>,
    creator: Rc<RefCell<TextureCreator<WindowContext>>>,
    size: (u32, u32),
    color: Color,
    font_mgr: Rc<RefCell<FontMgr>>,
}

impl Painter {
    pub fn new(
        size: (u32, u32),
        creator: Rc<RefCell<TextureCreator<WindowContext>>>,
        font_mgr: Rc<RefCell<FontMgr>>,
    ) -> Self {
        let surface = Surface::new(size.0, size.1, PixelFormat::RGBA8888.into())
            .expect("Could not create surface");
        Painter {
            canvas: surface.into_canvas().expect("Could not create canvas"),
            creator,
            size,
            color: Color::WHITE,
            font_mgr,
        }
    }

    pub fn rect(&mut self, rect: Rect) {
        let rect: SdlRect = rect.try_into().unwrap();
        self.canvas.fill_rect(Some(rect.into()));
    }

    pub fn text(&mut self, text: &str, x: u32, y: u32, font: Rc<RefCell<Font>>) {
        if text.len() == 0 {
            return;
        }
        let metrics = CosmicMetrics::new(14.0, 20.0);
        let mut buffer = CosmicBuffer::new(&mut self.font_mgr.borrow_mut().ctx.fs, metrics);
        let mut buffer = buffer.borrow_with(&mut self.font_mgr.borrow_mut().ctx.fs);
        todo!()
    }

    pub fn set_color(&mut self, color: Color) {
        // Implementation for setting background color
        self.canvas.set_draw_color(color);
        self.color = color;
    }

    pub fn copy(&mut self, source: Painter, source_area: Rect, dest_area: Rect) {
        let source_area: SdlRect = source_area.try_into().unwrap();
        let dest_area: SdlRect = dest_area.try_into().unwrap();
        let creator = self.creator.borrow_mut();
        let texture = creator
            .create_texture(
                Some(PixelFormat::RGBA8888.into()),
                sdl3::render::TextureAccess::Static,
                source.size.0,
                source.size.1,
            )
            .unwrap();
        self.canvas.copy(&texture, source_area, dest_area);
    }

    pub fn present(&mut self) {
        unsafe {
            self.canvas.flush_renderer();
        }
        self.canvas.present();
    }
}
