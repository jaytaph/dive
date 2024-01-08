use std::rc::Rc;
use crate::dive::command_queue::{Command, CommandQueue};
use crate::dive::widget_manager::Drawable;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::{
    Block, Borders, Clear, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
    Wrap,
};
use ratatui::Frame;
use crate::dive::gosub_logger::GosubLogger;

pub struct LogWidget {
    pub vertical_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub vertical_scroll_max: usize,
    pub logger: Rc<GosubLogger>,
}

impl LogWidget {
    pub fn new(logger: Rc<GosubLogger>) -> Self {
        Self {
            vertical_scroll_state: ScrollbarState::default(),
            vertical_scroll: 0,
            vertical_scroll_max: logger.logs().len(),
            logger,
        }
    }
}

impl Drawable for LogWidget {
    fn on_show(&mut self) {}
    fn on_hide(&mut self) {}

    fn render(&mut self, f: &mut Frame) {
        let size = f.size();
        let margins = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(size);

        let log_block_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(margins[1])[1];

        let log_block = Block::default()
            .title(" Logging ")
            .borders(Borders::ALL)
            .padding(Padding::uniform(1));

        let mut lines = vec![];
        for log in self.logger.logs() {
            lines.push(Line::from(log));
        }

        let log_paragraph = Paragraph::new(Text::from(lines))
            .block(log_block)
            .wrap(Wrap { trim: false })
            .scroll((self.vertical_scroll as u16, 0));

        f.render_widget(Clear, log_block_area);
        f.render_widget(log_paragraph, log_block_area);

        f.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            log_block_area,
            &mut self.vertical_scroll_state,
        );
    }

    fn event_handler(
        &mut self,
        queue: &mut CommandQueue,
        key: KeyEvent,
    ) -> anyhow::Result<Option<KeyEvent>> {
        match key.code {
            KeyCode::Esc => {
                queue.push(Command::DestroyWidget { id: "log".into() });
            }
            KeyCode::Down => {
                self.vertical_scroll = self
                    .vertical_scroll
                    .saturating_add(1)
                    .clamp(0, self.vertical_scroll_max - 1);
                self.vertical_scroll_state =
                    self.vertical_scroll_state.position(self.vertical_scroll);
            }
            KeyCode::Up => {
                self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
                self.vertical_scroll_state =
                    self.vertical_scroll_state.position(self.vertical_scroll);
            }
            _ => {}
        }

        Ok(Some(key))
    }
}
