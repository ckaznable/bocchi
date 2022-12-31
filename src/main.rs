use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block},
    Frame, Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    let size = f.size();
    const SHOW_HINT_WIDTH: u16 = 60;

    // Surrounding block
    let block = Block::default();
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);

    let mut constraints = vec![Constraint::Percentage(if size.width < SHOW_HINT_WIDTH {10} else {5}), Constraint::Percentage(30)];
    if size.width > SHOW_HINT_WIDTH {
        constraints.push(Constraint::Min(0));
    }

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints.as_ref())
        .split(chunks[0]);

    let block = Block::default()
        .style(Style::default().bg(Color::Rgb(249, 168, 186)));
    f.render_widget(block, top_chunks[1]);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(10), Constraint::Min(0)].as_ref())
        .vertical_margin(1)
        .split(top_chunks[0]);

    let block = Block::default()
        .style(Style::default().bg(Color::Rgb(95, 196, 229)));
    f.render_widget(block, left_chunks[0]);

    let block = Block::default()
        .style(Style::default().bg(Color::Rgb(210, 178, 73)));
    f.render_widget(block, left_chunks[1]);

    if size.width > SHOW_HINT_WIDTH {
        let block = Block::default()
            .title("Press <Q> exit");
        f.render_widget(block, top_chunks[2]);
    }
}