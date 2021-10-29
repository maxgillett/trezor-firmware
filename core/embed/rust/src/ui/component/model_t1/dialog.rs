use super::{
    button::{Button, ButtonMsg::Clicked},
    theme,
};
use crate::{
    micropython::buffer::Buffer,
    ui::{
        component::{Child, Component, Event, EventCtx},
        display,
        geometry::{Point, Rect},
    },
};

pub enum DialogMsg<T> {
    Content(T),
    LeftClicked,
    RightClicked,
}

pub struct Dialog<T, U> {
    header: Option<Buffer>,
    content: Child<T>,
    left_btn: Option<Child<Button<U>>>,
    right_btn: Option<Child<Button<U>>>,
}

impl<T: Component, U: AsRef<[u8]>> Dialog<T, U> {
    pub fn new(
        area: Rect,
        content: impl FnOnce(Rect) -> T,
        left: Option<impl FnOnce(Rect) -> Button<U>>,
        right: Option<impl FnOnce(Rect) -> Button<U>>,
        header: Option<Buffer>,
    ) -> Self {
        let button_h = 11;
        let header_h = 13;

        let (content_area, buttons) = area.hsplit(-button_h);
        let content_area = if header.is_none() {
            content_area
        } else {
            content_area.hsplit(header_h).1
        };
        let (left_rect, right_rect) = buttons.vsplit(buttons.width() / 2);

        let content = Child::new(content(content_area));
        let left_btn = left.map(|f| Child::new(f(left_rect)));
        let right_btn = right.map(|f| Child::new(f(right_rect)));
        Self {
            header: header,
            content: content,
            left_btn: left_btn,
            right_btn: right_btn,
        }
    }

    fn paint_header(&self) {
        if let Some(h) = &self.header {
            let line_height = theme::FONT_BOLD.line_height();
            display::text(
                Point::new(0, line_height - 2),
                h,
                theme::FONT_BOLD,
                theme::FG,
                theme::BG,
            );
            display::dotted_line(Point::new(0, line_height), theme::FG)
        }
    }
}

impl<T: Component, U: AsRef<[u8]>> Component for Dialog<T, U> {
    type Msg = DialogMsg<T::Msg>;

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        if let Some(msg) = self.content.event(ctx, event) {
            Some(DialogMsg::Content(msg))
        } else if let Some(Clicked) = self.left_btn.as_mut().and_then(|b| b.event(ctx, event)) {
            Some(DialogMsg::LeftClicked)
        } else if let Some(Clicked) = self.right_btn.as_mut().and_then(|b| b.event(ctx, event)) {
            Some(DialogMsg::RightClicked)
        } else {
            None
        }
    }

    fn paint(&mut self) {
        self.paint_header();
        self.content.paint();
        if let Some(b) = self.left_btn.as_mut() {
            b.paint();
        }
        if let Some(b) = self.right_btn.as_mut() {
            b.paint();
        }
    }
}

#[cfg(feature = "ui_debug")]
impl<T, U> crate::trace::Trace for Dialog<T, U>
where
    T: crate::trace::Trace,
    U: crate::trace::Trace + AsRef<[u8]>,
{
    fn trace(&self, t: &mut dyn crate::trace::Tracer) {
        t.open("Dialog");
        t.field("content", &self.content);
        if let Some(label) = &self.left_btn {
            t.field("left", label);
        }
        if let Some(label) = &self.right_btn {
            t.field("right", label);
        }
        t.close();
    }
}
