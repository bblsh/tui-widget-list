use crate::widget_list::widget_list_item::WidgetListItem;
use ratatui::prelude::*;
use ratatui::widgets::*;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone)]
pub enum WidgetListEnd {
    Top,
    Bottom,
}

pub struct WidgetList<'a> {
    pub items: Vec<WidgetListItem<'a>>,
    block: Option<Block<'a>>,
    style: Style,
    start_corner: Corner,
    /// Style used to render selected item
    highlight_style: Style,
    /// Symbol in front of the selected item (Shift all items to the right)
    highlight_symbol: Option<&'a str>,
    /// Whether to repeat the highlight symbol for each line of the selected item
    repeat_highlight_symbol: bool,
    follow_end: WidgetListEnd,
}

impl<'a> Default for WidgetList<'a> {
    fn default() -> Self {
        WidgetList {
            block: None,
            style: Style::default(),
            items: Vec::new(),
            start_corner: Corner::TopLeft,
            highlight_symbol: None,
            highlight_style: Style::default(),
            repeat_highlight_symbol: false,
            follow_end: WidgetListEnd::Top,
        }
    }
}

impl<'a> WidgetList<'a> {
    pub fn block(mut self, block: Block<'a>) -> WidgetList<'a> {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> WidgetList<'a> {
        self.style = style;
        self
    }

    pub fn highlight_symbol(mut self, highlight_symbol: &'a str) -> WidgetList<'a> {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_style(mut self, style: Style) -> WidgetList<'a> {
        self.highlight_style = style;
        self
    }

    pub fn repeat_highlight_symbol(mut self, repeat: bool) -> WidgetList<'a> {
        self.repeat_highlight_symbol = repeat;
        self
    }

    pub fn start_corner(mut self, corner: Corner) -> WidgetList<'a> {
        self.start_corner = corner;
        self
    }

    pub fn follow(mut self, end: WidgetListEnd) -> WidgetList<'a> {
        self.follow_end = end;
        self
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn get_items_bounds(
        &self,
        selected: Option<usize>,
        offset: usize,
        max_height: usize,
    ) -> (usize, usize) {
        let offset = offset.min(self.items.len().saturating_sub(1));
        let mut start = offset;
        let mut end = offset;
        let mut height = 0;
        for item in self.items.iter().skip(offset) {
            if height + item.height() > max_height {
                break;
            }
            height += item.height();
            end += 1;
        }

        let selected = selected.unwrap_or(0).min(self.items.len() - 1);
        while selected >= end {
            height = height.saturating_add(self.items[end].height());
            end += 1;
            while height > max_height {
                height = height.saturating_sub(self.items[start].height());
                start += 1;
            }
        }
        while selected < start {
            start -= 1;
            height = height.saturating_add(self.items[start].height());
            while height > max_height {
                end -= 1;
                height = height.saturating_sub(self.items[end].height());
            }
        }
        (start, end)
    }
}

impl<'a> Widget for WidgetList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = WidgetListState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}

impl<'a> StatefulWidget for WidgetList<'a> {
    type State = WidgetListState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);
        let list_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if list_area.width < 1 || list_area.height < 1 {
            return;
        }

        if self.items.is_empty() {
            return;
        }

        let list_height = list_area.height as usize;

        let (start, end) = self.get_items_bounds(state.selected, state.offset, list_height);
        state.offset = start;

        let highlight_symbol = self.highlight_symbol.unwrap_or("");

        let mut current_height = 0;
        let has_selection = state.selected.is_some();

        for (i, item) in self
            .items
            .into_iter()
            .enumerate()
            .skip(state.offset)
            .take(end - start)
        {
            let (x, y) = if self.start_corner == Corner::BottomLeft {
                current_height += item.height() as u16;
                (list_area.left(), list_area.bottom() - current_height)
            } else {
                let pos = (list_area.left(), list_area.top() + current_height);
                current_height += item.height() as u16;
                pos
            };

            let area = Rect {
                x,
                y,
                width: list_area.width,
                height: item.height() as u16,
            };

            let is_selected = state.selected.map_or(false, |s| s == i);

            if is_selected {
                let count = if self.repeat_highlight_symbol {
                    item.height()
                } else {
                    1
                };
                for n in 0..count as u16 {
                    buf.set_string(x, y + n, highlight_symbol, Style::default());
                }
            }

            let symbol_width = if has_selection {
                self.highlight_symbol.map_or(0, |s| s.width()) as u16
            } else {
                0
            };

            let item_area = Rect {
                x: area.x + symbol_width,
                width: area.width - symbol_width,
                ..area
            };

            // Render the WidgetList background style first
            // This way individual widgets can display their own styles
            if is_selected {
                buf.set_style(area, self.highlight_style);
            }

            // Finally, render the widget in this area
            item.render(item_area, buf);
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct WidgetListState {
    offset: usize,
    selected: Option<usize>,
}

impl WidgetListState {
    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn offset_mut(&mut self) -> &mut usize {
        &mut self.offset
    }

    pub fn with_selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
        if index.is_none() {
            self.offset = 0;
        }
    }
}

impl<'a> From<Vec<WidgetListItem<'a>>> for WidgetList<'a> {
    fn from(value: Vec<WidgetListItem<'a>>) -> WidgetList<'a> {
        WidgetList {
            block: None,
            style: Style::default(),
            items: value,
            start_corner: Corner::TopLeft,
            highlight_symbol: None,
            highlight_style: Style::default(),
            repeat_highlight_symbol: false,
            follow_end: WidgetListEnd::Top,
        }
    }
}
