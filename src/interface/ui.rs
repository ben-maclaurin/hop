use crate::interface::theme;
use std::path::PathBuf;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
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

    pub fn first(&mut self) {
        self.state.select(Some(0));
    }

    pub fn last(&mut self) {
        self.state.select(Some(self.items.len() - 1));
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
    pub items: StatefulList<(String, Color, theme::Icon)>,
}

impl App {
    pub fn new(entries: Vec<PathBuf>) -> App {
        println!("Detecting languages in project directory ... Please wait ...");

        let mut items = Vec::<(String, Color, theme::Icon)>::new();

        for entry in entries {
            if entry.is_dir() {
                items.push(theme::apply(entry.to_str().unwrap().to_owned()));
            }
        }

        print!("{}[2J", 27 as char);

        App {
            items: StatefulList::with_items(items),
        }
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, title: &String) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100), Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|i| {
            let (path, color, icon) = i;

            ListItem::new(vec![Spans::from(vec![Span::styled(
                icon.to_string()
                    + &path
                        .split("/")
                        .collect::<Vec<_>>()
                        .last()
                        .unwrap()
                        .to_string(),
                Style::default().fg(color.to_owned()),
            )])])
        })
        .collect();

    let items = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title.to_owned()),
        )
        .highlight_style(Style::default().bg(Color::Rgb(22, 22, 29)));

    f.render_stateful_widget(items, chunks[0], &mut app.items.state);
}