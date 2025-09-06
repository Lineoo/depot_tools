use std::{hash, num::TryFromIntError};

#[derive(Debug, Clone, Copy, hash::Hash)]
pub struct Rect {
    pub w: u32,
    pub h: u32,
    pub x: u32,
    pub y: u32,
}

impl Rect {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Rect { x, y, w, h }
    }

    pub fn size(&self) -> (u32, u32) {
        (self.w, self.h)
    }

    pub fn pos(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

impl TryFrom<sdl3::rect::Rect> for Rect {
    type Error = TryFromIntError;

    fn try_from(rect: sdl3::rect::Rect) -> Result<Self, TryFromIntError> {
        Ok(Rect {
            w: rect.w.try_into()?,
            h: rect.h.try_into()?,
            x: rect.x.try_into()?,
            y: rect.y.try_into()?,
        })
    }
}

impl TryInto<sdl3::rect::Rect> for Rect {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<sdl3::rect::Rect, TryFromIntError> {
        Ok(sdl3::rect::Rect::new(
            self.x.try_into()?,
            self.y.try_into()?,
            self.w,
            self.h,
        ))
    }
}
