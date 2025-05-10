use depot_core::{
    entryspace::EntrySpace,
    stack::{Stack, StackCall},
};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Position},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState, Paragraph},
};
use std::io;

fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

struct App {
    input: String,
    stack: Stack,

    character_index: usize,
    state: ListState,
}

impl App {
    fn new() -> Self {
        Self {
            input: String::new(),
            stack: Stack::new(Box::new(
                EntrySpace::from_toml(String::from(include_str!("config.toml"))).unwrap(),
            )),
            character_index: 0,
            state: ListState::default(),
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn move_selector_up(&mut self) {
        self.state.select_previous();
    }

    fn move_selector_down(&mut self) {
        self.state.select_next();
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
        self.write_args()
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
        self.write_args()
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn write_args(&mut self) {
        self.stack.write(self.input.clone());
    }

    fn call(&mut self) -> bool {
        if let Some(index) = self.state.selected() {
            let call = self.stack.call(index);
            if let StackCall::With(args) = call {
                self.input = args;
                self.character_index = self.input.chars().count();
                false
            } else {
                true
            }
        } else {
            false
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Enter => {
                            if self.call() {
                                return Ok(());
                            }
                        }
                        KeyCode::Char(to_insert) => self.enter_char(to_insert),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Up => self.move_selector_up(),
                        KeyCode::Down => self.move_selector_down(),
                        KeyCode::Esc => return Ok(()),
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical = Layout::vertical([Constraint::Length(3), Constraint::Min(1)]);
        let [input_area, results_area] = vertical.areas(frame.area());

        let input = Paragraph::new(self.input.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::bordered().title("Input"));
        frame.render_widget(input, input_area);
        frame.set_cursor_position(Position::new(
            input_area.x + self.character_index as u16 + 1,
            input_area.y + 1,
        ));

        let results: Vec<ListItem> = self
            .stack
            .iter()
            .map(|m| {
                ListItem::new(Line::from(vec![
                    Span::raw(m.title),
                    Span::raw("    "),
                    Span::raw(m.description),
                ]))
            })
            .collect();
        let results = List::new(results)
            .block(Block::bordered().title("Results"))
            .highlight_style(Style::new().on_light_yellow().black());
        frame.render_stateful_widget(results, results_area, &mut self.state);
    }
}
