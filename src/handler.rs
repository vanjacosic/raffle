use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }

        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }

        // Spin
        KeyCode::Char('s') => app.start(),

        // Tabs
        KeyCode::Tab => app.tabs.next_tab(),

        // List handlers
        KeyCode::Up => {
            app.list.previous();
        }

        KeyCode::Down => {
            app.list.next();
        }

        KeyCode::Left => {
            app.list.unselect();
        }

        KeyCode::Backspace => {
            app.list.remove();
        }

        _ => {}
    }
    Ok(())
}
