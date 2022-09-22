use crate::{
    backend::{
        configuration::Configuration,
        project::{get_projects, Project},
    },
    interface::theme::*, InputMode,
};
use std::path::PathBuf;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub struct StatefulList {
    pub state: ListState,
    pub items: Vec<Project>,
    pub projects: Vec<Project>,
}

impl StatefulList {
    pub fn with_items(items: Vec<Project>, projects: Vec<Project>) -> StatefulList {
        StatefulList {
            state: ListState::default(),
            items,
            projects,
        }
    }

    pub fn first(&mut self) {
        self.state.select(Some(0));
    }

    pub fn last(&mut self) {
        self.state.select(Some(self.items.len() - 1));
    }

    pub fn filter(&mut self, input: &String) {
        let filtered: Vec<Project> = self.projects.clone().into_iter().filter(|x| {
            x.path.to_lowercase().as_str().contains(input.to_lowercase().as_str())
        }).collect();

        self.items = filtered;

        self.first();
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
    pub items: StatefulList,
    pub input: String,
    pub input_mode: InputMode,
}

impl App {
    pub fn new(mut entries: Vec<PathBuf>, config: &Configuration, force_deep_sync: bool) -> App {
        println!("Generating icons for projects directory ...");

        if !config.include_files {
            entries = entries.into_iter().filter(|entry| entry.is_dir()).collect();
        }

        print!("{}[2J", 27 as char);

        App {
            items: StatefulList::with_items(get_projects(entries.clone(), force_deep_sync, config), get_projects(entries, force_deep_sync, config)), // items used here
            input: String::new(),
            input_mode: InputMode::Editing,
        }
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, title: &String) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(25)
        .constraints(
            [
                Constraint::Percentage(90),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
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
                .title_alignment(tui::layout::Alignment::Center)
                .border_type(tui::widgets::BorderType::Rounded)
                .title(title.to_owned()),
        )
        .highlight_style(Style::default().bg(Color::Rgb(34, 50, 73)));

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Search").title_alignment(tui::layout::Alignment::Center));

    f.render_widget(input, chunks[1]);


    f.render_stateful_widget(items, chunks[0], &mut app.items.state);
}
