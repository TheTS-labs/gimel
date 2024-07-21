use std::io;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;

use super::App;
use crate::tui::Terminal;

impl App {
    pub fn run(&mut self, terminal: &mut Terminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) { frame.render_widget(self, frame.size()); }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self.handle_key_event(key_event),
            _ => {},
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {},
        }
    }

    fn exit(&mut self) { self.exit = true; }

    fn increment_counter(&mut self) { self.counter += 1; }

    fn decrement_counter(&mut self) { self.counter -= 1; }
}
