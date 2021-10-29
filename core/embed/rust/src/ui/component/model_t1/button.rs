use crate::ui::{
    component::{
        model::{HidEvent, T1Button},
        Component, Event, EventCtx,
    },
    display::{self, Color, Font},
    geometry::{Offset, Point, Rect},
};

use super::theme;

pub enum ButtonMsg {
    Clicked,
}

pub enum ButtonPos {
    Left,
    Right,
}

impl ButtonPos {
    fn hit(&self, b: &T1Button) -> bool {
        match (self, b) {
            (Self::Left, T1Button::Left) => true,
            (Self::Right, T1Button::Right) => true,
            _ => false,
        }
    }
}

pub struct Button<T> {
    pos: ButtonPos,
    content: ButtonContent<T>,
    styles: ButtonStyleSheet,
    state: State,
}

impl<T: AsRef<[u8]>> Button<T> {
    pub fn new(pos: ButtonPos, content: ButtonContent<T>, styles: ButtonStyleSheet) -> Self {
        Self {
            pos,
            content,
            styles,
            state: State::Initial,
        }
    }

    pub fn with_text(pos: ButtonPos, text: T, styles: ButtonStyleSheet) -> Self {
        Self::new(pos, ButtonContent::Text(text), styles)
    }

    pub fn with_icon(pos: ButtonPos, image: &'static [u8], styles: ButtonStyleSheet) -> Self {
        Self::new(pos, ButtonContent::Icon(image), styles)
    }

    pub fn enable(&mut self, ctx: &mut EventCtx) {
        self.set(ctx, State::Initial)
    }

    pub fn disable(&mut self, ctx: &mut EventCtx) {
        self.set(ctx, State::Disabled)
    }

    pub fn is_enabled(&self) -> bool {
        matches!(
            self.state,
            State::Initial | State::Pressed | State::Released
        )
    }

    pub fn is_disabled(&self) -> bool {
        matches!(self.state, State::Disabled)
    }

    pub fn content(&self) -> &ButtonContent<T> {
        &self.content
    }

    fn style(&self) -> &ButtonStyle {
        match self.state {
            State::Initial | State::Released => self.styles.normal,
            State::Pressed => self.styles.active,
            State::Disabled => self.styles.disabled,
        }
    }

    fn set(&mut self, ctx: &mut EventCtx, state: State) {
        if self.state != state {
            self.state = state;
            ctx.request_paint();
        }
    }
}

impl<T: AsRef<[u8]>> Component for Button<T> {
    type Msg = ButtonMsg;

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        match event {
            Event::HumanInput(HidEvent::ButtonPressed(which)) => {
                match self.state {
                    State::Disabled => {
                        // Do nothing.
                    }
                    _ => {
                        if self.pos.hit(&which) {
                            // Touch started in our area, transform to `Pressed` state.
                            self.set(ctx, State::Pressed);
                        }
                    }
                }
            }
            Event::HumanInput(HidEvent::ButtonReleased(which)) => {
                match self.state {
                    State::Pressed if self.pos.hit(&which) => {
                        // Touch finished in our area, we got clicked.
                        self.set(ctx, State::Initial);

                        return Some(ButtonMsg::Clicked);
                    }
                    _ => {
                        // Do nothing.
                    }
                }
            }
            _ => {}
        };
        None
    }

    fn paint(&mut self) {
        let style = self.style();

        let button_h = 11;
        let button_y = display::height() - button_h;

        match &self.content {
            ButtonContent::Text(text) => {
                let width = display::text_width(text.as_ref(), style.font);
                let height = display::text_height();
                let button_x = match &self.pos {
                    ButtonPos::Left => 0,
                    ButtonPos::Right => display::width() - width + 1,
                };

                if style.border_horiz {
                    display::rounded_rect1(
                        Rect::from_top_left_and_size(
                            Point::new(button_x - 4, button_y),
                            Offset::new(width + 3, button_h),
                        ),
                        style.background_color,
                        theme::BG,
                    );
                } else {
                    display::rect(
                        Rect::from_top_left_and_size(
                            Point::new(button_x, button_y),
                            Offset::new(width - 1, button_h),
                        ),
                        style.background_color,
                    )
                }

                let h_border = if style.border_horiz { 2 } else { 0 };
                let start_of_baseline = match &self.pos {
                    ButtonPos::Left => Point::new(h_border, button_y + height + 1),
                    ButtonPos::Right => Point::new(
                        display::width() - h_border + 1 - width,
                        button_y + height + 1,
                    ),
                };
                display::text(
                    start_of_baseline,
                    text.as_ref(),
                    style.font,
                    style.text_color,
                    style.background_color,
                );
            }
            ButtonContent::Icon(_image) => {
                todo!();
            }
        }
    }
}

#[cfg(feature = "ui_debug")]
impl<T> crate::trace::Trace for Button<T>
where
    T: AsRef<[u8]> + crate::trace::Trace,
{
    fn trace(&self, t: &mut dyn crate::trace::Tracer) {
        t.open("Button");
        match &self.content {
            ButtonContent::Text(text) => t.field("text", text),
            ButtonContent::Icon(_) => t.symbol("icon"),
        }
        t.close();
    }
}

#[derive(PartialEq, Eq)]
enum State {
    Initial,
    Pressed,
    Released,
    Disabled,
}

pub enum ButtonContent<T> {
    Text(T),
    Icon(&'static [u8]),
}

pub struct ButtonStyleSheet {
    pub normal: &'static ButtonStyle,
    pub active: &'static ButtonStyle,
    pub disabled: &'static ButtonStyle,
}

pub struct ButtonStyle {
    pub font: Font,
    pub text_color: Color,
    pub background_color: Color,
    pub border_horiz: bool,
}
