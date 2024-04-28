use crate::app::{App, AppResult, Menus};
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
        // Counter handlers
        KeyCode::Up => {
            app.previous_action();
        }
        KeyCode::Down => {
            app.next_action();
        }
        KeyCode::Enter => match app.current_menu {
            Menus::ActionPage => app.select_action(),
            Menus::ErrorPage => app.current_menu = Menus::ActionPage,
            Menus::VerifyUserPage => app.select_user(app.selected_action),
            Menus::BanPage => app.select_user(app.selected_action),
            _ => {}
        },
        // Other handlers you could add here.
        _ => {}
    }

    Ok(())
}
