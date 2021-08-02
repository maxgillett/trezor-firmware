use core::convert::TryInto;

use crate::{error, ui::geometry::Point};

#[cfg(feature = "model_t1")]
pub use super::model_t1::constants;
#[cfg(feature = "model_t1")]
pub use super::model_t1::theme;

#[cfg(feature = "model_tt")]
pub use super::model_tt::constants;
#[cfg(feature = "model_tt")]
pub use super::model_tt::theme;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum T1Button {
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum T1HidEvent {
    ButtonPressed(T1Button),
    ButtonReleased(T1Button),
}

impl T1HidEvent {
    pub fn new(event: u32, button: u32, _unused: u32) -> Result<Self, error::Error> {
        let button = match button {
            0 => T1Button::Left,
            1 => T1Button::Right,
            _ => return Err(error::Error::OutOfRange),
        };
        let result = match event {
            1 => Self::ButtonPressed(button),
            2 => Self::ButtonReleased(button),
            _ => return Err(error::Error::OutOfRange),
        };
        Ok(result)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TTHidEvent {
    TouchStart(Point),
    TouchMove(Point),
    TouchEnd(Point),
}

impl TTHidEvent {
    pub fn new(event: u32, x: u32, y: u32) -> Result<Self, error::Error> {
        let point = Point::new(x.try_into()?, y.try_into()?);
        let result = match event {
            1 => Self::TouchStart(point),
            2 => Self::TouchMove(point),
            4 => Self::TouchEnd(point),
            _ => return Err(error::Error::OutOfRange),
        };
        Ok(result)
    }
}

#[cfg(feature = "model_t1")]
pub type HidEvent = T1HidEvent;

#[cfg(feature = "model_tt")]
pub type HidEvent = TTHidEvent;
