use crate::widget_list::widget_list::WidgetListState;

#[derive(Debug)]
pub struct StatefulWidgetList<T> {
    pub state: WidgetListState,
    pub items: Vec<T>,
}

impl<T> Default for StatefulWidgetList<T> {
    fn default() -> Self {
        Self {
            state: WidgetListState::default(),
            items: Vec::new(),
        }
    }
}

impl<T> StatefulWidgetList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulWidgetList<T> {
        StatefulWidgetList {
            state: WidgetListState::default(),
            items,
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
            None => 0,
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
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
