use std::path::PathBuf;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
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

pub struct App {
    pub items: StatefulList<String>,
}

impl App {
    pub fn new(entries: Vec<PathBuf>) -> App {
        let mut items = Vec::<String>::new();
        for entry in entries {
            items.push(
                entry
                    .to_str()
                    .unwrap()
                    .to_owned()
                    .split("/")
                    .last()
                    .unwrap()
                    .to_owned(),
            );
        }

        App {
            items: StatefulList::with_items(items),
        }
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, title: &String) {
    let chunks = Layout::default()
        .margin(2)
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100), Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|i| {
            ListItem::new(vec![Spans::from(i.to_string())])
                .style(Style::default().fg(Color::White).bg(Color::Reset))
        })
        .collect();

    let items = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title.to_owned()),
        )
        .highlight_style(Style::default().bg(Color::Cyan).fg(Color::Black));

    f.render_stateful_widget(items, chunks[0], &mut app.items.state);
}
