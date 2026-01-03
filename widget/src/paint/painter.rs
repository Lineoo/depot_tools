use std::{cell::RefCell, rc::Rc};

use cosmic_text::{Attrs, Buffer as CosmicBuffer, Color as CosmicColor, Metrics as CosmicMetrics};
use sdl3::{
    pixels::{Color, PixelFormat},
    rect::Rect as SdlRect,
    render::{BlendMode, Canvas, Texture, TextureCreator},
    surface::{Surface, SurfaceContext},
    video::WindowContext,
};

use crate::{
    control::font::{Font, FontMgr},
    paint::shapes::Rect,
};

pub struct Painter {
    pub(crate) canvas: Option<Canvas<Surface<'static>>>,
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
        let surface =
            Surface::new(size.0, size.1, PixelFormat::RGBA8888).expect("Could not create surface");
        Painter {
            canvas: Some(surface.into_canvas().expect("Could not create canvas")),
            creator,
            size,
            color: Color::WHITE,
            font_mgr,
        }
    }

    pub fn rect(&mut self, rect: Rect) {
        let rect: SdlRect = rect.try_into().unwrap();
        self.canvas.as_mut().unwrap().fill_rect(Some(rect.into()));
    }

    pub fn text(&mut self, text: &str, x: i32, y: i32, font: Option<Rc<RefCell<Font>>>) {
        if text.is_empty() {
            return;
        }
        let font_mgr = self.font_mgr.borrow();
        let mut fs = font_mgr.ctx.fs.borrow_mut();
        let metrics = CosmicMetrics::new(14.0, 20.0);
        let mut buffer = CosmicBuffer::new(&mut fs, metrics);
        let mut buffer = buffer.borrow_with(&mut fs);
        let attrs = Attrs::new();
        buffer.set_text(text, &attrs, cosmic_text::Shaping::Advanced);
        let Color { r, g, b, a } = self.color;
        let text_color = CosmicColor::rgba(r, g, b, a);

        self.canvas
            .as_mut()
            .unwrap()
            .set_blend_mode(BlendMode::Blend);

        buffer.draw(
            &mut font_mgr.ctx.sc.borrow_mut(),
            text_color,
            |xx, yy, ww, hh, color| {
                self.canvas.as_mut().unwrap().set_draw_color(Color {
                    r: color.r(),
                    g: color.g(),
                    b: color.b(),
                    a: color.a(),
                });
                let rect = SdlRect::new(x + xx, y + yy, ww, hh);
                self.canvas.as_mut().unwrap().fill_rect(Some(rect.into()));
            },
        );
    }

    pub fn set_color(&mut self, color: Color) {
        // Implementation for setting background color
        self.canvas.as_mut().unwrap().set_draw_color(color);
        self.color = color;
    }

    pub fn copy(&mut self, source: &mut Painter, source_area: Rect, dest_area: Rect) {
        let source_area: SdlRect = source_area.try_into().unwrap();
        let dest_area: SdlRect = dest_area.try_into().unwrap();
        let creator = self.canvas.as_ref().unwrap().texture_creator();
        let texture = source.get_texture(&creator);
        let result = self
            .canvas
            .as_mut()
            .unwrap()
            .copy(&texture, source_area, dest_area);
        if let Err(err) = result {
            eprintln!("Error copying texture: {}", err);
        }
    }

    pub fn present(&mut self) {
        unsafe {
            self.canvas.as_mut().unwrap().flush_renderer();
        }
        self.canvas.as_mut().unwrap().present();
    }

    fn get_texture<'c>(&mut self, creator: &'c TextureCreator<SurfaceContext>) -> Texture<'c> {
        let surface = self.canvas.take().unwrap().into_surface();
        let texture = surface.as_texture(creator).unwrap();
        self.canvas = Some(surface.into_canvas().unwrap());
        texture
    }
}
