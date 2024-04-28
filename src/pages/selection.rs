use ratatui::{widgets::ListState, Frame};

use crate::{app::App, generic_components::list, traits::Page};

pub struct SelectionPage;

impl Page for SelectionPage {
    fn render(app: &mut App, frame: &mut Frame) {
        let mut state: ListState = ListState::default();
        state.select(Some(app.selected_action));
        let list = list(app.actions.0.to_vec(), "Actions");
        frame.render_stateful_widget(list, frame.size(), &mut state);
    }
}
