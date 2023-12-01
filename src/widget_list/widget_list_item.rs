use ratatui::prelude::*;
use ratatui::widgets::*;

#[derive(Debug, Clone, Copy)]
pub struct WidgetListItem<W>
where
    W: Widget,
{
    item: W,
    width: usize,
    height: usize,
}

impl<W> Widget for WidgetListItem<W>
where
    W: Widget,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.item.render(area, buf);
    }
}

impl<W> WidgetListItem<W>
where
    W: Widget,
{
    pub fn new(widget: W, width: usize, height: usize) -> WidgetListItem<W> {
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
