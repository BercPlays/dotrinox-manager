use ratatui::Frame;

use crate::app::App;

pub trait Page {
    fn render(app: &mut App, frame: &mut Frame) {}
}
