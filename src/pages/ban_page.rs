use ratatui::{widgets::ListState, Frame};

use crate::{app::App, generic_components::list, traits::Page};

pub struct BanPage;

impl Page for BanPage {
    fn render(app: &mut App, frame: &mut Frame) {
        let list = list(app.verified_user_vec.to_vec(), "Ban a user...");
        let mut state = ListState::default();

        state.select(Some(app.selected_action));

        frame.render_stateful_widget(list, frame.size(), &mut state);
    }
}
