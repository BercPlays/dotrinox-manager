use directories::ProjectDirs;
use dotrinox_manager::app::{App, AppResult};
use dotrinox_manager::event::{Event, EventHandler};
use dotrinox_manager::files::{
    config_file_path, create_directory, data_file_path, file_exists, project_directory, read_file,
};
use dotrinox_manager::handler::handle_key_events;
use dotrinox_manager::panel_api_wrapper::OAuth2TokenRequestPayload;
use dotrinox_manager::panel_types::ConfigJson;
use dotrinox_manager::tui::Tui;

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use serde_json::{from_str, to_string_pretty};

use std::fs::File;
use std::io::{self, Write};

fn main() -> AppResult<()> {
    create_directory(project_directory());

    //the uglies thing you will ever see
    if (!file_exists(config_file_path())) {
        File::create(config_file_path())
            .unwrap()
            .write_all(
                to_string_pretty::<ConfigJson>(&ConfigJson::default())
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap()
    };

    // Create an application.
    let mut app = App::new();
    let config = from_str::<ConfigJson>(read_file(config_file_path()).as_str()).unwrap();
    app.panel_wrapper.0 .1 =
        OAuth2TokenRequestPayload::new(config.client_id, config.client_secret, config.domain);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(30);
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
