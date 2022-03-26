extern crate crossterm;
extern crate tui;

use std::io;
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, read},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::Stdout;

fn main() -> Result<(), io::Error> {
    let mut terminal = setup_terminal()?;
    event_loop(&mut terminal).expect("Error executing event loop");

    restore_terminal(&mut terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), io::Error> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

fn event_loop<T: tui::backend::Backend>(terminal: &mut Terminal<T>) -> crossterm::Result<()> {
    loop {
        // `read()` blocks until an `Event` is available
        match read()? {
            Event::Key(event) => {
                println!("{:?}", event);
                if event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL {
                    break;
                }
            },
            Event::Mouse(_event) => {},
            Event::Resize(_, _) => {
                terminal.draw(|f| {
                    let size = f.size();
                    let block = Block::default()
                        .title("Block")
                        .borders(Borders::ALL);
                    f.render_widget(block, size);
                })?;
            },
        }
    }

    Ok(())
}