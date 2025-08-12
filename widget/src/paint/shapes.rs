use std::{hash, num::TryFromIntError};

#[derive(Debug, Clone, Copy, hash::Hash)]
pub struct Rect {
    pub w: u32,
    pub h: u32,
    pub x: u32,
    pub y: u32,
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

impl TryFrom<cosmic_text::ttf_parser::Rect> for Rect {
    type Error = TryFromIntError;

    fn try_from(rect: cosmic_text::ttf_parser::Rect) -> Result<Self, TryFromIntError> {
        Ok(Rect {
            w: rect.width().try_into()?,
            h: rect.height().try_into()?,
            x: rect.x_min.try_into()?,
            y: rect.y_min.try_into()?,
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

impl TryInto<cosmic_text::ttf_parser::Rect> for Rect {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<cosmic_text::ttf_parser::Rect, TryFromIntError> {
        let x = self.x.try_into()?;
        let y = self.y.try_into()?;
        Ok(cosmic_text::ttf_parser::Rect {
            x_min: x,
            y_min: y,
            x_max: std::convert::TryInto::<i16>::try_into(self.w)? + x,
            y_max: std::convert::TryInto::<i16>::try_into(self.h)? + y,
        })
    }
}
