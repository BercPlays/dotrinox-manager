use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Paragraph, Wrap,
    },
    Frame,
};

use crate::{app::App, traits::Page};

pub struct ErrorPage;

impl Page for ErrorPage {
    fn render(app: &mut App, frame: &mut Frame) {
        let instructions = Title::from(Line::from(vec![
            " Back ".red().into(),
            "<Enter> ".light_red().bold(),
        ]));

        let p = Paragraph::new(app.error_string.as_str())
            .block(
                Block::new().title(" Error ").borders(Borders::ALL).title(
                    instructions
                        .alignment(Alignment::Left)
                        .position(Position::Bottom),
                ),
            )
            .style(Style::new().red().on_black())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        frame.render_widget(p, frame.size());
    }
}
