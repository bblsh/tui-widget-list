use crate::widget_list::widget_list::WidgetListState;

#[derive(Debug)]
pub struct StatefulWidgetList<T> {
    pub state: WidgetListState,
    pub items: Vec<T>,
    last_selected: Option<usize>,
}

impl<T> Default for StatefulWidgetList<T> {
    fn default() -> Self {
        Self {
            state: WidgetListState::default(),
            items: Vec::new(),
            last_selected: None,
        }
    }
}

impl<T> StatefulWidgetList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulWidgetList<T> {
        StatefulWidgetList {
            state: WidgetListState::default(),
            items,
            last_selected: None,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        let offset = self.state.offset();
        self.last_selected = self.state.selected();
        self.state.select(None);
        *self.state.offset_mut() = offset;
    }

    pub fn select_last(&mut self) {
        self.state.select(Some(self.items.len() - 1));
    }
}
