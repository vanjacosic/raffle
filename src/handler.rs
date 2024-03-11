use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }
    match key_event.code {
        // Exit application on `q`
        KeyCode::Char('q') => {
            app.quit();
        }

        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }

        // Tabs
        KeyCode::Tab => app.tabs.next_tab(),

        // Spin actions
        KeyCode::Char('s') => app.start_spin(),

        KeyCode::Char('r') => app.reset_spin(),

        // List handlers
        KeyCode::Up => {
            app.all_participants.previous();
        }

        KeyCode::Down => {
            app.all_participants.next();
        }

        KeyCode::Esc => {
            app.all_participants.unselect();
        }

        KeyCode::Backspace => {
            app.all_participants.remove();
        }

        _ => {}
    }
    Ok(())
}
