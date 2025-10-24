use std::{cell::RefCell, rc::Rc};

use sdl3::{
    pixels::{Color, PixelFormat},
    rect::Rect as SdlRect,
    render::{Canvas, TextureCreator},
    surface::Surface,
    ttf::Font,
    video::WindowContext,
};

use crate::paint::shapes::Rect;

pub struct Painter {
    pub(crate) canvas: Canvas<Surface<'static>>,
    creator: Rc<RefCell<TextureCreator<WindowContext>>>,
    size: (u32, u32),
    color: Color,
}

impl Painter {
    pub fn new(size: (u32, u32), creator: Rc<RefCell<TextureCreator<WindowContext>>>) -> Self {
        let surface = Surface::new(size.0, size.1, PixelFormat::RGBA8888.into())
            .expect("Could not create surface");
        Painter {
            canvas: surface.into_canvas().expect("Could not create canvas"),
            creator,
            size,
            color: Color::WHITE,
        }
    }

    pub fn rect(&mut self, rect: Rect) {
        let rect: SdlRect = rect.try_into().unwrap();
        self.canvas.fill_rect(Some(rect.into()));
    }

    pub fn text(&mut self, text: &str, x: u32, y: u32, font: Rc<RefCell<Font<'static>>>) {
        if text.len() == 0 {
            return;
        }
        let r = font.borrow_mut().render(text).blended(self.color).unwrap();
        let rect = r.rect();
        self.canvas.copy(
            &r.as_texture(&self.canvas.texture_creator()).unwrap(),
            rect,
            SdlRect::new(
                x.try_into().unwrap(),
                y.try_into().unwrap(),
                rect.width(),
                rect.height(),
            ),
        );
    }

    pub fn set_color(&mut self, color: Color) {
        // Implementation for setting background color
        self.canvas.set_draw_color(color);
        self.color = color;
    }

    pub fn set_font_size(&mut self, w: f32, h: f32) {}

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
