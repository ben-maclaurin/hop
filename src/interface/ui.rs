use crate::{
    backend::{
        configuration::Configuration,
        project::{get_projects, Project},
    },
    interface::theme::*,
};
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
    pub items: StatefulList<Project>,
}

impl App {
    pub fn new(mut entries: Vec<PathBuf>, config: &Configuration, force_deep_sync: bool) -> App {
        println!("Generating icons for projects directory ...");

        if !config.include_files {
            entries = entries.into_iter().filter(|entry| entry.is_dir()).collect();
        }

        print!("{}[2J", 27 as char);

        App {
            items: StatefulList::with_items(get_projects(entries, force_deep_sync, config)), // items used here
        }
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, title: &String) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(5)
        .constraints([Constraint::Percentage(100), Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let items: Vec<ListItem> = app
        .items
        .items
        .iter().rev()
        .map(|project| {
            ListItem::new(vec![Spans::from(vec![Span::styled(
                project.theme.icon.to_string()
                    + &project
                        .path
                        .split("/")
                        .collect::<Vec<_>>()
                        .last()
                        .unwrap()
                        .to_string(),
                Style::default().fg(Color::Rgb(
                    project.theme.color.0,
                    project.theme.color.1,
                    project.theme.color.2,
                )),
            )])])
        })
        .collect();

    let items = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(WHITE.0, WHITE.1, WHITE.2)))
                .title(title.to_owned()),
        )
        .highlight_style(Style::default().bg(Color::Rgb(34, 50, 73)));

    f.render_stateful_widget(items, chunks[0], &mut app.items.state);
}
