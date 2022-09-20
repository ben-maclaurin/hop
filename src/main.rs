use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::process::Command;
use std::str;
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::ListState,Terminal,
};

mod configuration;
mod ui;
mod directory_manager;
use configuration::Configuration;
use directory_manager::{get_entries, get_home_dir};
use ui::{ui, App};

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

    let projects_dir =
        get_home_dir().to_str().unwrap().to_owned() + "/" + &jump_config.projects_dir;
    let res = run_app(
        &mut terminal,
        app,
        tick_rate,
        projects_dir,
        &jump_config.launch_command,
        &jump_config.title,
    );

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
    title: &String,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app, title))?;

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
                            .spawn()?
                            .wait();

                        return Ok(());
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

