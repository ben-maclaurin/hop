use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::process::Command;
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod backend;
mod interface;
use backend::configuration::Configuration;
use backend::project::get_project_paths;
use interface::ui::{ui, App};
use std::env;

pub enum InputMode {
    Normal,
    Editing,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let mut force_deep_sync: bool = false;

    if args.len() > 1 && args[1] == "-s" {
        force_deep_sync = true;
    }

    let mut jump_config = Configuration::default();

    jump_config.init();

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(250);
    let app = App::new(
        get_project_paths(&jump_config).unwrap(),
        &jump_config,
        force_deep_sync,
    );
    let res = run_app(&mut terminal, app, tick_rate, jump_config);

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
    config: Configuration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app, &config.title))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Left => app.items.unselect(),
                        KeyCode::Char('j') => app.items.next(),
                        KeyCode::Char('k') => app.items.previous(),
                        KeyCode::Char('g') => app.items.first(),
                        KeyCode::Char('G') => app.items.last(),
                        KeyCode::Tab => {
                            app.input_mode = InputMode::Editing;
                        }
                        KeyCode::Enter => {
                            Command::new(config.editor)
                                .args([app.items.items[app.items.state.selected().unwrap()]
                                    .path
                                    .to_string()])
                                .spawn()?
                                .wait()
                                .unwrap();

                            return Ok(());
                        }
                        _ => {},
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Char(c) => {
                            app.input.push(c);
                            app.items.filter(&app.input);
                        },
                        KeyCode::Backspace => {
                            app.input.pop();
                            app.items.filter(&app.input);
                        },
                        KeyCode::Enter => {
                            app.items.first();
                            Command::new(config.editor)
                                .args([app.items.items[app.items.state.selected().unwrap()]
                                    .path
                                    .to_string()])
                                .spawn()?
                                .wait()
                                .unwrap();

                            return Ok(());
                        },
                        KeyCode::Tab => {
                            app.input_mode = InputMode::Normal;
                            app.items.first();
                        },
                        _ => {},
                    },
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}
