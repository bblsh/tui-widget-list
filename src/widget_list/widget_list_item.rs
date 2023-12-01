use ratatui::prelude::*;
use ratatui::widgets::*;

#[derive(Debug, Clone)]
pub struct WidgetListItem<'a> {
    item: Paragraph<'a>,
    width: usize,
    height: usize,
}

impl<'a> Widget for WidgetListItem<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.item.render(area, buf);
    }
}

impl<'a> WidgetListItem<'a> {
    pub fn new(widget: Paragraph<'a>, width: usize, height: usize) -> WidgetListItem<'a> {
        WidgetListItem {
            item: widget,
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
}
