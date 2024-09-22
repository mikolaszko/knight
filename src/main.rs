use ratatui::{
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Constraint, Direction, Layout},
    prelude::{Backend, CrosstermBackend},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{error::Error, io, vec};

use sysinfo::{System, Users};

pub struct App {
    system: System,
    users: Users,
}

impl App {
    pub fn new() -> App {
        App {
            system: System::new_all(),
            users: Users::new_with_refreshed_list(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    let _ = run(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        app.system.refresh_memory();
        terminal.draw(|f| {
            let main_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ])
                .split(f.area());
            f.render_widget(
                Paragraph::new("mid").block(Block::new().borders(Borders::ALL)),
                main_layout[1],
            );
            f.render_widget(
                Paragraph::new("bottom").block(Block::new().borders(Borders::ALL)),
                main_layout[2],
            );

            let used_cpu_sec = Line::from(vec![Span::raw(app.system.used_memory().to_string())]);

            let used_cpu_sec = Line::from(vec![Span::raw(app.system.used_memory().to_string())]);

            f.render_widget(used_cpu_sec, main_layout[1])
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL)
    }
}
