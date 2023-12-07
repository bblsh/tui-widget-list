use ratatui::prelude::*;
use ratatui::widgets::*;
use std::fmt::Debug;

pub enum WidgetListItemType<'a> {
    Paragraph(Paragraph<'a>),
    Chart(Chart<'a>),
}

pub struct WidgetListItem<'a> {
    widget: Option<WidgetListItemType<'a>>,
    width: usize,
    height: usize,
}

impl<'a> WidgetListItem<'a> {
    pub fn new(widget: WidgetListItemType<'a>, width: usize, height: usize) -> Self {
        Self {
            widget: Some(widget),
            width,
            height,
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn render(&mut self, area: Rect, buffer: &mut Buffer) {
        if self.widget.is_some() {
            let widget = self.widget.take().unwrap();
            match widget {
                WidgetListItemType::Paragraph(p) => p.render(area, buffer),
                WidgetListItemType::Chart(c) => c.render(area, buffer),
            }
        }
    }
}

impl Debug for WidgetListItem<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WidgetListItem")
            .field("height", &self.height)
            .field("width", &self.width)
            .finish()
    }
}
