use std::time::Duration;
use rand::Rng;
use ratatui::{
    Frame,
    layout::{Rect, Layout, Constraint, Direction, Alignment},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Paragraph, Clear},
};
use crossterm::event::KeyCode;
use super::{Game, GameCommand};

const WORD_BANK: &[&str] = &[
    "TERMINAL", "ARCADE", "RUSTACEAN", "COMPILER", "DOUBLEBUFFER",
    "RETROWAVE", "KEYBOARD", "MONITOR", "DUNGEON", "SPACESHIP",
    "MINESWEEPER", "TETROMINO", "SHIELDS", "BATTLESHIP", "HANGMAN",
];

pub struct HangmanGame {
    secret_word: String,
    guessed_chars: Vec<char>,
    wrong_guesses: usize,
    game_over: bool,
    won: bool,
    paused: bool,
    score: u32,
    warning_msg: Option<String>,
}

impl Default for HangmanGame {
    fn default() -> Self {
        Self::new()
    }
}

impl HangmanGame {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..WORD_BANK.len());
        let secret = WORD_BANK[idx].to_string();

        Self {
            secret_word: secret,
            guessed_chars: Vec::new(),
            wrong_guesses: 0,
            game_over: false,
            won: false,
            paused: false,
            score: 0,
            warning_msg: None,
        }
    }

    fn make_guess(&mut self, c: char) {
        self.warning_msg = None;

        if self.guessed_chars.contains(&c) {
            self.warning_msg = Some(format!("Already guessed letter '{}'!", c));
            return;
        }

        self.guessed_chars.push(c);

        if !self.secret_word.contains(c) {
            self.wrong_guesses += 1;
            if self.wrong_guesses >= 6 {
                self.game_over = true;
                self.won = false;
            }
        } else {
            // Check victory
            let mut completed = true;
            for sc in self.secret_word.chars() {
                if !self.guessed_chars.contains(&sc) {
                    completed = false;
                    break;
                }
            }
            if completed {
                self.won = true;
                self.game_over = true;
                self.score = 600 - (self.wrong_guesses as u32 * 100);
            }
        }
    }

    fn get_gallows_ascii(&self) -> Vec<&'static str> {
        match self.wrong_guesses {
            0 => vec![
                "   +---+  ",
                "   |   |  ",
                "       |  ",
                "       |  ",
                "       |  ",
                "       |  ",
                "  ========="
            ],
            1 => vec![
                "   +---+  ",
                "   |   |  ",
                "   O   |  ",
                "       |  ",
                "       |  ",
                "       |  ",
                "  ========="
            ],
            2 => vec![
                "   +---+  ",
                "   |   |  ",
                "   O   |  ",
                "   |   |  ",
                "       |  ",
                "       |  ",
                "  ========="
            ],
            3 => vec![
                "   +---+  ",
                "   |   |  ",
                "   O   |  ",
                "  /|   |  ",
                "       |  ",
                "       |  ",
                "  ========="
            ],
            4 => vec![
                "   +---+  ",
                "   |   |  ",
                "   O   |  ",
                "  /|\\  |  ",
                "       |  ",
                "       |  ",
                "  ========="
            ],
            5 => vec![
                "   +---+  ",
                "   |   |  ",
                "   O   |  ",
                "  /|\\  |  ",
                "  /    |  ",
                "       |  ",
                "  ========="
            ],
            _ => vec![
                "   +---+  ",
                "   |   |  ",
                "   O   |  ",
                "  /|\\  |  ",
                "  / \\  |  ",
                "       |  ",
                "  ========="
            ],
        }
    }
}

impl Game for HangmanGame {
    fn update(&mut self, _delta: Duration) {}

    fn handle_input(&mut self, key: KeyCode) -> GameCommand {
        if self.game_over {
            match key {
                KeyCode::Char('r') | KeyCode::Char('R') => return GameCommand::Restart,
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => return GameCommand::Exit,
                _ => return GameCommand::None,
            }
        }

        if key == KeyCode::Char('p') || key == KeyCode::Char('P') {
            self.paused = !self.paused;
            return GameCommand::None;
        }

        if self.paused {
            if key == KeyCode::Esc || key == KeyCode::Char('q') || key == KeyCode::Char('Q') {
                return GameCommand::Exit;
            }
            return GameCommand::None;
        }

        match key {
            KeyCode::Char(c) => {
                if c.is_ascii_alphabetic() {
                    let uppercase_c = c.to_ascii_uppercase();
                    self.make_guess(uppercase_c);
                }
            }
            KeyCode::Esc => {
                return GameCommand::Exit;
            }
            _ => {}
        }

        GameCommand::None
    }

    fn draw(&self, frame: &mut Frame, area: Rect) {
        let outer_block = Block::default()
            .title(" HANGMAN CABINET ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD));

        frame.render_widget(outer_block, area);

        let inner_area = area.inner(&ratatui::layout::Margin { horizontal: 2, vertical: 1 });

        let layouts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(28), // Gallows area
                Constraint::Min(12),    // Secret Word & Letters
            ])
            .split(inner_area);

        let gallow_area = layouts[0];
        let side_area = layouts[1];

        // 1. Draw Gallows
        let gallow_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Gray));
        let gallow_inner = gallow_block.inner(gallow_area);
        frame.render_widget(gallow_block, gallow_area);

        let ascii = self.get_gallows_ascii();
        let mut gallow_lines = Vec::new();
        gallow_lines.push(Line::from(""));
        for row in ascii {
            gallow_lines.push(Line::from(Span::styled(
                format!("  {}", row),
                Style::default().fg(Color::Rgb(150, 100, 50)).add_modifier(Modifier::BOLD)
            )));
        }
        let gallow_paragraph = Paragraph::new(gallow_lines);
        frame.render_widget(gallow_paragraph, gallow_inner);

        // 2. Draw Side Board Panels
        let side_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6), // Secret Word Display
                Constraint::Min(6),    // Letters & Warnings
            ])
            .split(side_area);

        // Secret Word Display
        let mut secret_spans = Vec::new();
        secret_spans.push(Span::raw("   ")); // spacing
        for sc in self.secret_word.chars() {
            if self.guessed_chars.contains(&sc) || self.game_over {
                secret_spans.push(Span::styled(
                    format!("{} ", sc),
                    Style::default().fg(if self.guessed_chars.contains(&sc) { Color::Green } else { Color::Red }).add_modifier(Modifier::BOLD)
                ));
            } else {
                secret_spans.push(Span::styled("_ ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
            }
        }
        let word_content = vec![
            Line::from(""),
            Line::from(Span::styled("   GUESS SECRET WORD:", Style::default().fg(Color::DarkGray))),
            Line::from(""),
            Line::from(secret_spans),
        ];
        let word_paragraph = Paragraph::new(word_content)
            .block(Block::default().borders(Borders::ALL).title("WORD"));
        frame.render_widget(word_paragraph, side_layout[0]);

        // Letters Guessing Tracker
        let mut guess_content = Vec::new();
        guess_content.push(Line::from(""));
        
        let mut guessed_line_spans = vec![Span::styled("   Guessed: ", Style::default().fg(Color::DarkGray))];
        if self.guessed_chars.is_empty() {
            guessed_line_spans.push(Span::styled("None", Style::default().fg(Color::DarkGray)));
        } else {
            for &c in &self.guessed_chars {
                let correct = self.secret_word.contains(c);
                guessed_line_spans.push(Span::styled(
                    format!("{} ", c),
                    Style::default().fg(if correct { Color::Green } else { Color::Red }).add_modifier(Modifier::BOLD)
                ));
            }
        }
        guess_content.push(Line::from(guessed_line_spans));
        guess_content.push(Line::from(""));

        // Warning or Action instruction
        if let Some(ref warn) = self.warning_msg {
            guess_content.push(Line::from(Span::styled(format!("   ⚠ {}", warn), Style::default().fg(Color::Yellow))));
        } else {
            guess_content.push(Line::from(Span::styled("   Type any letter [A-Z] to guess!", Style::default().fg(Color::Gray))));
        }
        guess_content.push(Line::from(""));

        let lives_left = 6u32.saturating_sub(self.wrong_guesses as u32);
        let mut heart_spans = vec![Span::styled("   LIVES: ", Style::default().fg(Color::DarkGray))];
        for _ in 0..lives_left {
            heart_spans.push(Span::styled("♥ ", Style::default().fg(Color::Red)));
        }
        for _ in 0..(6 - lives_left) {
            heart_spans.push(Span::styled(". ", Style::default().fg(Color::Rgb(50, 50, 50))));
        }
        guess_content.push(Line::from(heart_spans));

        let guess_paragraph = Paragraph::new(guess_content)
            .block(Block::default().borders(Borders::ALL).title("TACTICAL DATA"));
        frame.render_widget(guess_paragraph, side_layout[1]);

        // Overlays
        if self.paused {
            let pause_area = Rect {
                x: gallow_inner.x + 3,
                y: gallow_inner.y + 1,
                width: 18,
                height: 5,
            };
            frame.render_widget(Clear, pause_area);
            let pause_widget = Paragraph::new(vec![
                Line::from(""),
                Line::from(Span::styled(" PAUSED ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
                Line::from(Span::styled("Press 'P' to resume", Style::default().fg(Color::DarkGray))),
            ])
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Yellow)));
            frame.render_widget(pause_widget, pause_area);
        } else if self.game_over {
            let go_area = Rect {
                x: gallow_inner.x + 2,
                y: gallow_inner.y + 1,
                width: 22,
                height: 8,
            };
            frame.render_widget(Clear, go_area);
            
            let message = if self.won {
                vec![
                    Line::from(""),
                    Line::from(Span::styled(" FREEDOM! WINNER ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))),
                    Line::from(format!("Score: {}", self.score)),
                    Line::from(""),
                    Line::from(Span::styled("Press [R] to retry", Style::default().fg(Color::Green))),
                    Line::from(Span::styled("Press [Esc] to exit", Style::default().fg(Color::DarkGray))),
                ]
            } else {
                vec![
                    Line::from(""),
                    Line::from(Span::styled(" EXECUTED... DEFEAT ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))),
                    Line::from(format!("Word: {}", self.secret_word)),
                    Line::from(""),
                    Line::from(Span::styled("Press [R] to retry", Style::default().fg(Color::Green))),
                    Line::from(Span::styled("Press [Esc] to exit", Style::default().fg(Color::DarkGray))),
                ]
            };

            let go_widget = Paragraph::new(message)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(if self.won { Color::Green } else { Color::Red })));
            frame.render_widget(go_widget, go_area);
        }
    }

    fn get_score(&self) -> u32 {
        self.score
    }

    fn is_game_over(&self) -> bool {
        self.game_over
    }
}
