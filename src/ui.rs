use crate::{
    app::{App, Menus},
    pages::{
        ban_page::BanPage, error_page::ErrorPage, selection::SelectionPage,
        verify_user::VerifyUserPage,
    },
    traits::Page,
};
use ratatui::Frame;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    match &app.current_menu {
        Menus::ActionPage => SelectionPage::render(app, frame),
        Menus::VerifyUserPage => {
            VerifyUserPage::render(app, frame);
        }
        Menus::ErrorPage => ErrorPage::render(app, frame),
        Menus::BanPage => BanPage::render(app, frame),

        _ => {}
    };
}
