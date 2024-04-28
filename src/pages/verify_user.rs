use ratatui::{widgets::ListState, Frame};

use crate::{app::App, generic_components::list, traits::Page};

pub struct VerifyUserPage;
//CLIENT ID: ab243ac2-be75-4cc1-b5d4-e4ff136392fa
//CLIENT SECRET: CFUlrvsi3CGLM8As9Yw54Viv2Z_Fd5ggJ7lnRBb55_TocrgJ

impl Page for VerifyUserPage {
    fn render(app: &mut App, frame: &mut Frame) {
        let list = list(app.new_user_vec.to_vec(), "Verify a user...");
        let mut state = ListState::default();

        state.select(Some(app.selected_action));

        frame.render_stateful_widget(list, frame.size(), &mut state);
    }
}
