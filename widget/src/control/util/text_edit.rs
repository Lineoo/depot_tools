use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use cosmic_text::{
    Action as CosmicAction, Attrs, Buffer, Color as CosmicColor, Edit, Editor, Font as CosmicFont,
    Metrics,
    rustybuzz::{self, UnicodeBuffer},
};
use sdl3::{
    event::Event::{self as SdlEvent, TextEditing as SdlTextEditing, TextInput as SdlTextInput},
    keyboard::TextInputUtil,
    pixels::Color,
    rect::Rect as SdlRect,
};

use crate::{
    control::{
        ctrl_ctx::CtrlCtx,
        font::{Font, FontMgr},
    },
    paint::{
        cosmic_color_into_sdl_color, painter::Painter, sdl_color_into_cosmic_color, shapes::Rect,
    },
    window::{Window, WindowDirector},
};

/// Helper struct to process events and render texts for an input box.
///
/// You can simply make your own input box control put a TextEdit in it and forward events to it.
/// All rendering work can be automatically done, along with shortcuts and CJK support,  and no
/// other decorations would be painted other than texts, cursor, and selection.
///
/// Animation is now not supported, but will be added in the future.
///
/// If you just want an input box, use `InputBox` instead, which provides better integration and
/// can adapt to themes automatically.
pub struct TextEdit {
    input_util: Rc<RefCell<TextInputUtil>>,
    win: Weak<RefCell<WindowDirector>>,
    geometry: (i32, i32, u32, u32),
    font: Option<Rc<RefCell<Font>>>,
    font_mgr: Rc<RefCell<FontMgr>>,

    color: Color,

    editor: Editor<'static>,
}

impl TextEdit {
    pub fn new(input_util: Rc<RefCell<TextInputUtil>>, ctrl_ctx: &CtrlCtx) -> Self {
        let buffer = Buffer::new(
            &mut ctrl_ctx.font_mgr().borrow().ctx.fs.borrow_mut(),
            Metrics::new(14.0, 20.0),
        );
        TextEdit {
            input_util,
            win: Weak::new(),
            geometry: (0, 0, 0, 0),
            font: None,
            font_mgr: ctrl_ctx.font_mgr().clone(),

            color: Color::WHITE,

            editor: Editor::new(buffer),
        }
    }

    pub fn attach_window(&mut self, win: Weak<RefCell<WindowDirector>>) {
        self.win = win;
    }

    pub(crate) fn action(&mut self, action: CosmicAction) {
        if self.is_active() {
            self.editor
                .action(&mut self.font_mgr.borrow().ctx.fs.borrow_mut(), action);
        }
    }

    pub fn is_active(&self) -> bool {
        let win = self.win.upgrade().unwrap();
        let win = win.borrow();
        self.input_util.borrow().is_active(unsafe { win.raw() })
    }

    pub fn insert_text(&mut self, text: &str) {
        self.editor.insert_string(text, None);
    }

    pub fn text(&self) -> String {
        match *self.editor.buffer_ref() {
            cosmic_text::BufferRef::Owned(ref buffer) => buffer
                .lines
                .iter()
                .map(|line| line.text())
                .collect::<Vec<_>>()
                .concat(),
            cosmic_text::BufferRef::Borrowed(ref buffer) => buffer
                .lines
                .iter()
                .map(|line| line.text())
                .collect::<Vec<_>>()
                .concat(),
            cosmic_text::BufferRef::Arc(ref buffer) => buffer
                .lines
                .iter()
                .map(|line| line.text())
                .collect::<Vec<_>>()
                .concat(),
        }
    }

    pub fn cursor_pos(&self) -> usize {
        self.editor.cursor().index
    }

    pub fn geometry(&self) -> (i32, i32, u32, u32) {
        self.geometry
    }

    pub fn set_geometry(&mut self, x: i32, y: i32, w: u32, h: u32) {
        self.geometry = (x, y, w, h);
    }

    pub fn set_size(&mut self, w: u32, h: u32) {
        self.geometry.2 = w;
        self.geometry.3 = h;
    }

    pub fn render(&mut self, painter: &mut Painter) {
        let font_mgr = &self.font_mgr.borrow();
        let fs = &mut font_mgr.ctx.fs.borrow_mut();
        let color = sdl_color_into_cosmic_color(self.color);
        self.editor.with_buffer_mut(|buffer| {
            buffer.set_size(
                fs,
                Some(self.geometry.2 as f32),
                Some(self.geometry.3 as f32),
            );
        });
        self.editor.shape_as_needed(fs, true);
        self.editor.draw(
            fs,
            &mut font_mgr.ctx.sc.borrow_mut(),
            color,
            color,
            CosmicColor::rgb(200, 200, 200),
            CosmicColor::rgb(10, 10, 10),
            |x, y, w, h, color| {
                painter.set_color(cosmic_color_into_sdl_color(color));
                painter.rect(Rect { w, h, x, y });
            },
        );
    }

    pub fn set_font(&mut self, font: Rc<RefCell<Font>>) {
        self.font = Some(font);
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn gain_focus(&mut self) -> Result<(), TextEditWindowDestroyedErr> {
        if let Some(win) = self.win.upgrade()
            && let Ok(win) = win.try_borrow_mut()
        {
            let (x, y) = win.pos();
            self.geometry.0 = x;
            self.geometry.1 = y;
            self.input_util
                .borrow_mut()
                .set_rect(unsafe { win.raw() }, self.geometry.into(), 0);
            self.input_util.borrow_mut().start(unsafe { win.raw() });
            Ok(())
        } else {
            Err(TextEditWindowDestroyedErr)
        }
    }

    pub fn lose_focus(&mut self) -> Result<(), TextEditWindowDestroyedErr> {
        if let Some(win) = self.win.upgrade()
            && let Ok(win) = win.try_borrow_mut()
        {
            self.input_util.borrow_mut().stop(unsafe { win.raw() });
            Ok(())
        } else {
            Err(TextEditWindowDestroyedErr)
        }
    }

    fn text_pos_at(&self, x: u32) -> usize {
        let mut buffer = UnicodeBuffer::new();
        buffer.push_str(&self.text());
        let pos_vec = rustybuzz::shape(
            self.font
                .as_ref()
                .unwrap()
                .borrow()
                .data
                .borrow()
                .rustybuzz(),
            &[],
            buffer,
        )
        .glyph_positions()
        .iter()
        .map(|pos| (pos.x_offset + pos.x_advance / 2) as u32)
        .collect::<Vec<_>>();
        if pos_vec[pos_vec.len() - 1] <= x {
            return pos_vec.len();
        } else if pos_vec[0] >= x {
            return 0;
        }
        let idx = pos_vec.iter().position(|p| *p > x).unwrap(); // TODO: binary search
        if pos_vec[idx] - x > x - pos_vec[idx - 1] {
            idx - 1
        } else {
            idx
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TextEditWindowDestroyedErr;
