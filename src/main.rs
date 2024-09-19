use ratatui::{
    crossterm::{
        event::{
            self, DisableFocusChange, DisableMouseCapture, EnableFocusChange, EnableMouseCapture,
            KeyCode, KeyEventKind,
        },
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Constraint, Direction, Layout},
    prelude::{Backend, CrosstermBackend},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{error::Error, io, vec};

use sysinfo::System;

pub struct App {}

impl App {
    pub fn new() -> App {
        App {}
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode();
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    let _ = run(&mut terminal, &mut app, sys);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>, app: &mut App, mut sys: System) -> io::Result<()> {
    loop {
        sys.refresh_memory();
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

            let used_cpu_block = Block::default().title("Used Memory");
            let used_cpu_text = Paragraph::new(sys.used_memory().to_string()).block(used_cpu_block);
            f.render_widget(used_cpu_text, main_layout[1])
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL)
    }
}
