use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use cosmic_text::{
    Font as CosmicFont,
    rustybuzz::{self, UnicodeBuffer},
};
use sdl3::{
    event::Event::{self as SdlEvent, TextEditing as SdlTextEditing, TextInput as SdlTextInput},
    keyboard::TextInputUtil,
    pixels::Color,
    rect::Rect,
    ttf::Font as SdlFont,
};

use crate::{paint::painter::Painter, ui_control::ctrl_ctx::CtrlCtx, window::Window};

#[derive(Clone, Debug, Default)]
struct InputArea {
    text: String,
    cursor: usize,
    selecting: bool,
    selection_start: usize,
    selection_end: usize,
    composing: bool,
    composition_start: usize,
    composition_end: usize,
}

impl InputArea {
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
        match event {
            InputAreaEvent::TextEditing {
                text,
                start,
                length,
                ..
            } => {
                self.composing = true;
                self.composition_start = start as usize;
                self.composition_end = (start + length) as usize;
                // Replace the composing text
                self.text
                    .replace_range(self.composition_start..self.composition_end, &text);
                self.cursor = self.composition_end;
            }
            InputAreaEvent::TextInput { text, .. } => {
                // Insert the text at the cursor position
                self.text.insert_str(self.cursor, &text);
                self.cursor += text.len();
                self.composing = false;
            }
            InputAreaEvent::CursorMove { offset } => {
                let new_pos = (self.cursor as i32 + offset).max(0) as usize;
                self.cursor = new_pos.min(self.text.len());
                if self.selecting {
                    self.selection_end = self.cursor;
                    if self.selection_start > self.selection_end {
                        std::mem::swap(&mut self.selection_start, &mut self.selection_end);
                    }
                }
            }
            InputAreaEvent::StartSelection => {
                self.selecting = true;
                self.selection_start = self.cursor;
                self.selection_end = self.cursor;
            }
            InputAreaEvent::EndSelection => {
                self.selecting = false;
            }
        }
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
}

enum InputAreaEvent {
    TextEditing {
        text: String,
        start: i32,
        length: i32,
    },
    TextInput {
        text: String,
    },
    CursorMove {
        offset: i32,
    },
    StartSelection,
    EndSelection,
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
                text,
                start,
                length,
            }),
            SdlTextInput {
                window_id, text, ..
            } => Ok(InputAreaEvent::TextInput { text }),
            _ => Err(InputAreaEvtNotSupport),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) struct InputAreaEvtNotSupport;

const CLICK_MOVE_THRESHOLD_PX: u32 = 5;

pub struct TextEdit {
    input_util: Rc<RefCell<TextInputUtil>>,
    win: Weak<RefCell<Window>>,
    area: InputArea,
    geometry: (u32, u32, u32, u32),
    cursor_offset: i32,
    color: Color,
    font: Option<Rc<RefCell<CosmicFont>>>,

    click_start_pos: Option<(u32, u32)>,
    is_dragging: bool,
}

impl TextEdit {
    pub fn new(input_util: Rc<RefCell<TextInputUtil>>, ctrl_ctx: CtrlCtx) -> Self {
        TextEdit {
            input_util,
            win: Weak::new(),
            area: InputArea::new(),
            geometry: (0, 0, 0, 0),
            cursor_offset: 0,
            color: Color::BLACK,
            font: None,

            click_start_pos: None,
            is_dragging: false,
        }
    }

    pub fn attach_window(&mut self, win: Weak<RefCell<Window>>) {
        self.win = win;
    }

    pub(crate) fn event(&mut self, event: TextEditEvt) {
        match event {
            TextEditEvt::MouseDown { x, y } => {
                self.click_start_pos = Some((x, y));
                todo!()
            }
            TextEditEvt::MouseUp { x: _, y: _ } => {
                self.is_dragging = false;
                self.click_start_pos = None;
                todo!()
            }
            TextEditEvt::MouseMove { x, y } => {
                let (start_x, start_y) = self.click_start_pos.unwrap();
                let dx2 = x.abs_diff(start_x).pow(2);
                let dy2 = y.abs_diff(start_y).pow(2);
                if dx2 + dy2 > CLICK_MOVE_THRESHOLD_PX.pow(2) {
                    self.is_dragging = true;
                }
                todo!()
            }
            TextEditEvt::TextInput { text } => {
                self.area.event(InputAreaEvent::TextInput { text });
            }
            TextEditEvt::TextEditing {
                text,
                start,
                length,
            } => {
                self.area.event(InputAreaEvent::TextEditing {
                    text,
                    start: start as i32,
                    length: length as i32,
                });
            }
        }
        todo!()
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

    pub fn geometry(&self) -> (u32, u32, u32, u32) {
        self.geometry
    }

    pub fn set_geometry(&mut self, x: u32, y: u32, w: u32, h: u32) {
        self.geometry = (x, y, w, h);
    }

    pub fn render(&mut self, painter: &mut Painter) {
        painter.set_color(self.color);
        let (x, y, _, _) = self.geometry;
        painter.text(self.text(), x, y, self.font.clone().unwrap());
        // TODO: render cursor, selection, composition
    }

    pub fn set_font(&mut self, font: Rc<RefCell<SdlFont<'static>>>) {
        self.font = Some(font);
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
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

    fn text_pos_at(&self, x: u32) -> usize {
        let mut buffer = UnicodeBuffer::new();
        buffer.push_str(self.text());
        let pos_vec = rustybuzz::shape(
            self.font.as_ref().unwrap().borrow().rustybuzz(),
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

pub(crate) enum TextEditEvt {
    MouseDown {
        x: u32,
        y: u32,
    },
    MouseUp {
        x: u32,
        y: u32,
    },
    MouseMove {
        x: u32,
        y: u32,
    },
    TextInput {
        text: String,
    },
    TextEditing {
        text: String,
        start: u32,
        length: u32,
    },
}
