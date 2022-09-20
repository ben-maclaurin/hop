#![feature(allocator_api)]
use config::Config;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use directories::BaseDirs;
use std::collections::HashMap;
use std::fs;
use std::str;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};

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

/// This struct holds the current state of the app. In particular, it has the `items` field which is a wrapper
/// around `ListState`. Keeping track of the items state let us render the associated widget with its state
/// and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.
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
    // ///////////////////////////
    // Read settings in
    // ///////////////////////////
    let base_dirs = BaseDirs::new().unwrap();

    let home_dir = base_dirs.home_dir();

    let settings = Config::builder()
        .add_source(config::File::with_name(&(String::from(home_dir.to_str().unwrap()) + "/.config/jump/jump.yml")))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let settings = settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();

    let mut projects_dir = "";
    let mut launch_command = "";

    for (setting, value) in &settings {
        if setting == "projects_dir" {
            projects_dir = value;
        } else {
            launch_command = value;
        }
    }


    // ///////////////////////////
    // Dir management
    // ///////////////////////////
    let base_dirs = BaseDirs::new().unwrap();

    let home_dir = base_dirs.home_dir();

    let home_dir = home_dir.to_str().unwrap().to_owned() + "/" + projects_dir;
    let target_dir = Path::new(&home_dir);

    // println!("target dir is {:?}", target_dir);

    let entries = fs::read_dir(target_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(250);
    let app = App::new(entries);


    let res = run_app(&mut terminal, app, tick_rate, target_dir, launch_command);

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
    target_dir: &Path,
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
                            .args([target_dir.to_str().unwrap().to_owned()
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

    // Reset,
    // Black,
    // Red,
    // Green,
    // Yellow,
    // Blue,
    // Magenta,
    // Cyan,
    // Gray,
    // DarkGray,
    // LightRed,
    // LightGreen,
    // LightYellow,
    // LightBlue,
    // LightMagenta,
    // LightCyan,
    // White,
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
        .block(Block::default().borders(Borders::ALL).title("Projects"))
        .highlight_style(
            Style::default()
                .bg(Color::Cyan)
                .fg(Color::Black),
        );


    f.render_stateful_widget(items, chunks[0], &mut app.items.state);
}
