use clap::Parser;
use raffle::app::{App, AppResult};
use raffle::event::{Event, EventHandler};
use raffle::handler::handle_key_events;

use raffle::tui::Tui;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::path::{PathBuf};

const TICK_RATE: u64 = 100;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    participants_file: Option<PathBuf>,
}

fn main() -> AppResult<()> {
    // Create an application.
    let args = Args::parse();
    let mut app = match args.participants_file {
        Some(path) => App::new(&path),
        None => App::default(),
    };

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(TICK_RATE);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
