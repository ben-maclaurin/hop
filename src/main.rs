use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::str;
use std::path::PathBuf;
use std::process::Command;
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};

mod configuration;
mod directory_manager;
use configuration::Configuration;
use directory_manager::{get_entries, get_home_dir};

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
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

    fn previous(&mut self) {
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

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

struct App {
    items: StatefulList<String>,
}

impl App {
    fn new(entries: Vec<PathBuf>) -> App {
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

fn main() -> Result<(), Box<dyn Error>> {
    let mut jump_config = Configuration::default();

    jump_config.init();

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(250);

    let app = App::new(get_entries(jump_config.clone()).unwrap());

    let projects_dir = get_home_dir().to_str().unwrap().to_owned() + "/" + &jump_config.projects_dir;
    let res = run_app(&mut terminal, app, tick_rate, projects_dir, &jump_config.launch_command);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
    projects_dir: String,
    launch_command: &str,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Left => app.items.unselect(),
                    KeyCode::Char('j') => app.items.next(),
                    KeyCode::Char('k') => app.items.previous(),
                    KeyCode::Enter => {
                        println!(
                            "selected is: {:?}",
                            app.items.items[app.items.state.selected().unwrap()]
                        );

                        Command::new(launch_command)
                            .args([projects_dir
                                + "/"
                                + &app.items.items[app.items.state.selected().unwrap()]
                                + "/"])
                            .spawn()?.wait();

                        return Ok(())
                    }
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
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
        .block(Block::default().borders(Borders::ALL).title("Project directories"))
        .highlight_style(
            Style::default()
                .bg(Color::Cyan)
                .fg(Color::Black),
        );


    f.render_stateful_widget(items, chunks[0], &mut app.items.state);
}
