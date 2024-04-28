use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{
        block::{Position, Title},
        Block, Borders, HighlightSpacing, List, ListItem,
    },
};

pub fn list<'a, T>(items: T, title: &'a str) -> List<'a>
where
    T: IntoIterator,
    T::Item: Into<ListItem<'a>>,
{
    let instructions = Title::from(Line::from(vec![
        " Previous ".into(),
        "<↑>".blue().bold(),
        " Next ".into(),
        "<↓>".blue().bold(),
        " Quit ".into(),
        "<Q, Esc, Ctrl+C> ".blue().bold(),
    ]));
    let list = List::new(items)
        .block(
            Block::default()
                .title(Title::from(format!(" {} ", title)).alignment(Alignment::Left))
                .title(
                    instructions
                        .alignment(Alignment::Left)
                        .position(Position::Bottom),
                )
                .borders(Borders::ALL)
                .border_style(Style::new().light_green()),
        )
        .highlight_style(
            Style::new()
                .add_modifier(Modifier::REVERSED)
                .fg(Color::Green),
        )
        .highlight_symbol(">> ")
        .highlight_spacing(HighlightSpacing::WhenSelected)
        .repeat_highlight_symbol(true);
    list
}
