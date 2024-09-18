use std::io;

use sysinfo::System;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    text::{Line, Text},
    DefaultTerminal,
};

fn main() -> io::Result<()> {
    let sys = System::new_all();
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(terminal, sys);
    ratatui::restore();
    app_result
}

fn run(mut terminal: DefaultTerminal, sys: System) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let used_memory = sys.used_memory();
            let greeting = Text::from(Line::from(vec![
                "Knight!".into(),
                used_memory.to_string().black(),
            ]))
            .on_blue();
            frame.render_widget(greeting, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
