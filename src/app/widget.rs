use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::text::{Line, Text};
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::{Block, Paragraph, Widget};

use super::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Counter App Tutorial ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(instructions.alignment(Alignment::Center).position(Position::Bottom))
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text).centered().block(block).render(area, buf);
    }
}
