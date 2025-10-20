use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use sdl3::{
    event::Event::{self as SdlEvent, TextEditing as SdlTextEditing, TextInput as SdlTextInput},
    rect::Rect,
};
use sdl3::{keyboard::TextInputUtil, pixels::Color};

use crate::{paint::painter::Painter, window::Window};

#[derive(Clone, Debug, Default)]
struct TextArea {
    text: String,
    cursor: usize,
    selecting: bool,
    selection_start: usize,
    selection_end: usize,
    composing: bool,
    composition_start: usize,
    composition_end: usize,
}

impl TextArea {
    fn new() -> Self {
        Self {
            text: String::new(),
            cursor: 0,
            selecting: false,
            selection_start: 0,
            selection_end: 0,
            composing: false,
            composition_start: 0,
            composition_end: 0,
        }
    }

    fn event(&mut self, event: InputAreaEvent) {
        todo!()
    }

    fn text(&self) -> &str {
        self.text.as_str()
    }

    fn cursor_pos(&self) -> usize {
        self.cursor
    }

    fn selection(&self) -> Option<(usize, usize)> {
        if self.selecting {
            Some((self.selection_start, self.selection_end))
        } else {
            None
        }
    }

    fn composing(&self) -> Option<(usize, usize)> {
        if self.composing {
            Some((self.composition_start, self.composition_end))
        } else {
            None
        }
    }

    fn select_start(&mut self, pos: usize) {
        self.selecting = true;
        self.selection_start = pos;
        self.selection_end = pos;
    }

    fn selecting(&mut self, pos: usize) {
        if self.selecting {
            self.selection_end = pos;
        }
    }

    fn select_end(&mut self) {
        self.selecting = false;
    }
}

pub(crate) enum InputAreaEvent {
    TextEditing {
        win_id: u32,
        text: String,
        start: i32,
        length: i32,
    },
    TextInput {
        win_id: u32,
        text: String,
    },
}

impl TryFrom<SdlEvent> for InputAreaEvent {
    type Error = InputAreaEvtNotSupport;

    fn try_from(value: SdlEvent) -> Result<Self, Self::Error> {
        match value {
            SdlTextEditing {
                window_id,
                text,
                start,
                length,
                ..
            } => Ok(InputAreaEvent::TextEditing {
                win_id: window_id,
                text,
                start,
                length,
            }),
            SdlTextInput {
                window_id, text, ..
            } => Ok(InputAreaEvent::TextInput {
                win_id: window_id,
                text,
            }),
            _ => Err(InputAreaEvtNotSupport),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) struct InputAreaEvtNotSupport;

pub struct TextEdit {
    input_util: Rc<RefCell<TextInputUtil>>,
    win: Weak<RefCell<Window>>,
    area: TextArea,
    geometry: (u32, u32, u32, u32),
    cursor_offset: i32,
}

impl TextEdit {
    pub fn new(input_util: Rc<RefCell<TextInputUtil>>) -> Self {
        TextEdit {
            input_util,
            win: Weak::new(),
            area: TextArea::new(),
            geometry: (0, 0, 0, 0),
            cursor_offset: 0,
        }
    }

    pub fn attach_window(&mut self, win: Weak<RefCell<Window>>) {
        self.win = win;
    }

    pub fn event(&mut self, event: InputAreaEvent) {
        self.area.event(event);
    }

    pub fn text(&self) -> &str {
        self.area.text()
    }

    pub fn cursor_pos(&self) -> usize {
        self.area.cursor_pos()
    }

    pub fn selection(&self) -> Option<(usize, usize)> {
        self.area.selection()
    }

    pub fn composing(&self) -> Option<(usize, usize)> {
        self.area.composing()
    }

    pub fn set_geometry(&mut self, x: u32, y: u32, w: u32, h: u32) {
        self.geometry = (x, y, w, h);
    }

    pub fn render(&mut self, painter: &mut Painter) {
        todo!()
    }

    pub fn set_line_height(&mut self, line_height: i32) {
        todo!()
    }

    pub fn set_font(&mut self, font: sdl3::ttf::Font) {
        todo!()
    }

    pub fn set_color(&mut self, color: Color) {
        todo!()
    }

    pub fn gain_focus(&mut self) -> Result<(), TextEditWindowDestroyedErr> {
        if let Some(win) = self.win.upgrade()
            && let Ok(win) = win.try_borrow_mut()
        {
            let (x, y, w, h) = self.geometry;
            self.input_util.borrow_mut().set_rect(
                unsafe { win.raw() },
                Rect::new(x as i32, y as i32, w, h),
                0,
            );
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
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TextEditWindowDestroyedErr;
