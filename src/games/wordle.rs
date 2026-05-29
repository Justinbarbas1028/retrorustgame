use std::time::Duration;
use rand::Rng;
use ratatui::{
    Frame,
    layout::{Rect, Layout, Constraint, Direction, Alignment},
    style::{Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Paragraph, Clear},
};
use crossterm::event::KeyCode;
use crate::settings::ThemePalette;
use super::{Game, GameCommand};

const WORD_BANK: &[&str] = &[
    "RUSTY", "BOARD", "FLOAT", "CODES", "INDEX", "FRAME", "LOOPS", "STACK",
    "ARRAY", "MATCH", "SCORE", "PIXEL", "INPUT", "MOUSE", "LOGIC", "PRIME",
    "SHIPS", "CLONE", "SNAKE", "ROBOT", "WUMPUS", "CHESS", "GRAIN", "FLAME",
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum LetterState {
    Empty,
    Inputting,
    Correct,     // Green
    Present,     // Yellow
    Absent,      // Gray
}

#[derive(Clone, Copy, Debug)]
struct Cell {
    char_val: char,
    state: LetterState,
}

pub struct WordleGame {
    secret_word: String,
    grid: [[Cell; 5]; 6],
    current_attempt: usize,
    cursor_col: usize,
    game_over: bool,
    won: bool,
    paused: bool,
    score: u32,
    keyboard_states: std::collections::HashMap<char, LetterState>,
}

impl Default for WordleGame {
    fn default() -> Self {
        Self::new()
    }
}

impl WordleGame {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..WORD_BANK.len());
        let secret = WORD_BANK[idx].to_string();

        let mut game = Self {
            secret_word: secret,
            grid: [[Cell { char_val: ' ', state: LetterState::Empty }; 5]; 6],
            current_attempt: 0,
            cursor_col: 0,
            game_over: false,
            won: false,
            paused: false,
            score: 0,
            keyboard_states: std::collections::HashMap::new(),
        };

        // Initialize keyboard character tracking
        for c in 'A'..='Z' {
            game.keyboard_states.insert(c, LetterState::Empty);
        }

        game
    }

    fn submit_guess(&mut self) {
        if self.cursor_col < 5 {
            return; // Needs 5 characters
        }

        let attempt = self.current_attempt;
        let mut guess = String::new();
        for col in 0..5 {
            guess.push(self.grid[attempt][col].char_val);
        }

        let secret = self.secret_word.clone();
        let secret_chars: Vec<char> = secret.chars().collect();
        let mut guess_cells = self.grid[attempt];

        // 1st Pass: Check exact matches (Green)
        let mut matched_secret = vec![false; 5];
        let mut matched_guess = vec![false; 5];

        for col in 0..5 {
            if guess_cells[col].char_val == secret_chars[col] {
                guess_cells[col].state = LetterState::Correct;
                matched_secret[col] = true;
                matched_guess[col] = true;

                // Update keyboard
                self.keyboard_states.insert(guess_cells[col].char_val, LetterState::Correct);
            }
        }

        // 2nd Pass: Check partial matches (Yellow)
        for col in 0..5 {
            if matched_guess[col] {
                continue;
            }
            let char_val = guess_cells[col].char_val;
            let mut found = false;

            for secret_col in 0..5 {
                if !matched_secret[secret_col] && secret_chars[secret_col] == char_val {
                    guess_cells[col].state = LetterState::Present;
                    matched_secret[secret_col] = true;
                    matched_guess[col] = true;
                    found = true;

                    // Update keyboard unless it's already marked Correct
                    if self.keyboard_states.get(&char_val) != Some(&LetterState::Correct) {
                        self.keyboard_states.insert(char_val, LetterState::Present);
                    }
                    break;
                }
            }

            if !found {
                guess_cells[col].state = LetterState::Absent;
                // Update keyboard unless already Correct or Present
                let current_k = self.keyboard_states.get(&char_val);
                if current_k != Some(&LetterState::Correct) && current_k != Some(&LetterState::Present) {
                    self.keyboard_states.insert(char_val, LetterState::Absent);
                }
            }
        }

        self.grid[attempt] = guess_cells;

        if guess == secret {
            self.won = true;
            self.game_over = true;
            self.score = 1000 - (attempt as u32 * 150); // Speed/Attempt bonus
        } else if self.current_attempt >= 5 {
            self.game_over = true;
            self.won = false;
        } else {
            self.current_attempt += 1;
            self.cursor_col = 0;
        }
    }
}

impl Game for WordleGame {
    fn update(&mut self, _delta: Duration) {}

    fn handle_input(&mut self, key: KeyCode) -> GameCommand {
        if self.game_over {
            match key {
                KeyCode::Char('r') | KeyCode::Char('R') => return GameCommand::Restart,
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => return GameCommand::Exit,
                _ => return GameCommand::None,
            }
        }

        if key == KeyCode::Tab {
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
                if c.is_ascii_alphabetic() && self.cursor_col < 5 {
                    let uppercase_c = c.to_ascii_uppercase();
                    let attempt = self.current_attempt;
                    self.grid[attempt][self.cursor_col] = Cell {
                        char_val: uppercase_c,
                        state: LetterState::Inputting,
                    };
                    self.cursor_col += 1;
                }
            }
            KeyCode::Backspace => {
                if self.cursor_col > 0 {
                    self.cursor_col -= 1;
                    let attempt = self.current_attempt;
                    self.grid[attempt][self.cursor_col] = Cell {
                        char_val: ' ',
                        state: LetterState::Empty,
                    };
                }
            }
            KeyCode::Enter => {
                self.submit_guess();
            }
            KeyCode::Esc => {
                return GameCommand::Exit;
            }
            _ => {}
        }

        GameCommand::None
    }

    fn draw(&self, frame: &mut Frame, area: Rect, palette: &ThemePalette) {
        let outer_block = Block::default()
            .title(" WORDLE CABINET ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(palette.accent_alt).add_modifier(Modifier::BOLD));

        frame.render_widget(outer_block, area);

        let inner_area = area.inner(&ratatui::layout::Margin { horizontal: 2, vertical: 1 });

        let layouts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(32), // Guess Grid area (5 cols * 6 chars = 30 + borders)
                Constraint::Min(12),    // Keyboard tracker & sidebar
            ])
            .split(inner_area);

        let grid_area = layouts[0];
        let side_area = layouts[1];

        // Draw Guess grid borders
        let grid_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(palette.muted));
        let grid_inner = grid_block.inner(grid_area);
        frame.render_widget(grid_block, grid_area);

        // Build Wordle grid lines
        let mut rows = Vec::new();
        rows.push(Line::from(""));
        for attempt in 0..6 {
            let mut spans = Vec::new();
            spans.push(Span::raw("   ")); // padding
            for col in 0..5 {
                let cell = self.grid[attempt][col];
                let val_str = format!(" {} ", cell.char_val);
                
                let (fg, bg, modifier) = match cell.state {
                    LetterState::Empty => (palette.muted, palette.muted, Modifier::empty()),
                    LetterState::Inputting => {
                        // Highlight input cell under the cursor
                        if attempt == self.current_attempt && col == self.cursor_col.saturating_sub(1) {
                            (palette.accent_alt, palette.muted, Modifier::UNDERLINED | Modifier::BOLD)
                        } else {
                            (palette.body, palette.muted, Modifier::empty())
                        }
                    }
                    LetterState::Correct => (palette.muted, palette.accent, Modifier::BOLD),
                    LetterState::Present => (palette.muted, palette.accent_alt, Modifier::BOLD),
                    LetterState::Absent => (palette.body, palette.muted, Modifier::empty()),
                };

                spans.push(Span::styled(
                    val_str,
                    Style::default().fg(fg).bg(bg).add_modifier(modifier)
                ));
                spans.push(Span::raw(" ")); // spacer
            }
            rows.push(Line::from(spans));
            rows.push(Line::from("")); // vertical spacer
        }

        let grid_paragraph = Paragraph::new(rows);
        frame.render_widget(grid_paragraph, grid_inner);

        // Sidebar keyboard panel
        let side_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8), // Stats
                Constraint::Min(6),    // Keyboard tracker
            ])
            .split(side_area);

        let stats_content = vec![
            Line::from(vec![
                Span::styled(" ATTEMPTS", Style::default().fg(palette.accent)),
                Span::styled(format!("  {}/6", self.current_attempt + 1), Style::default().fg(palette.body).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" WORD BANK", Style::default().fg(palette.accent)),
                Span::styled(format!("  {} items", WORD_BANK.len()), Style::default().fg(palette.body)),
            ]),
        ];
        let stats_paragraph = Paragraph::new(stats_content)
            .block(Block::default().borders(Borders::ALL).title("STATS"));
        frame.render_widget(stats_paragraph, side_layout[0]);

        // Keyboard grid tracker rendering (split A-Z into lines)
        let mut keyboard_rows = Vec::new();
        keyboard_rows.push(Line::from("  ALPHABET KEYMAP:"));
        keyboard_rows.push(Line::from(""));

        let key_rows_chars = [
            &['Q','W','E','R','T','Y','U','I','O','P'][..],
            &['A','S','D','F','G','H','J','K','L'][..],
            &['Z','X','C','V','B','N','M'][..],
        ];

        for &row_chars in &key_rows_chars {
            let mut key_spans = Vec::new();
            key_spans.push(Span::raw("  "));
            for &c in row_chars {
                let state = self.keyboard_states.get(&c).copied().unwrap_or(LetterState::Empty);
                let (fg, bg) = match state {
                    LetterState::Correct => (palette.muted, palette.accent),
                    LetterState::Present => (palette.muted, palette.accent_alt),
                    LetterState::Absent => (palette.muted, palette.muted),
                    _ => (palette.body, palette.muted),
                };
                key_spans.push(Span::styled(
                    format!(" {} ", c),
                    Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD)
                ));
                key_spans.push(Span::raw(" "));
            }
            keyboard_rows.push(Line::from(key_spans));
            keyboard_rows.push(Line::from(""));
        }

        let keyboard_paragraph = Paragraph::new(keyboard_rows)
            .block(Block::default().borders(Borders::ALL).title("ELIMINATIONS"));
        frame.render_widget(keyboard_paragraph, side_layout[1]);

        // Overlays
        if self.paused {
            let pause_area = Rect {
                x: grid_inner.x + (30 - 18) / 2,
                y: grid_inner.y + 5,
                width: 18,
                height: 5,
            };
            frame.render_widget(Clear, pause_area);
            let pause_widget = Paragraph::new(vec![
                Line::from(""),
                Line::from(Span::styled(" PAUSED ", Style::default().fg(palette.accent_alt).add_modifier(Modifier::BOLD))),
                Line::from(Span::styled("Press [Tab] to resume", Style::default().fg(palette.muted))),
            ])
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(palette.accent_alt)));
            frame.render_widget(pause_widget, pause_area);
        } else if self.game_over {
            let go_area = Rect {
                x: grid_inner.x + (30 - 22) / 2,
                y: grid_inner.y + 4,
                width: 22,
                height: 8,
            };
            frame.render_widget(Clear, go_area);
            
            let message = if self.won {
                vec![
                    Line::from(""),
                    Line::from(Span::styled(" WORD REVEALED! ", Style::default().fg(palette.accent).add_modifier(Modifier::BOLD))),
                    Line::from(format!("Score: {}", self.score)),
                    Line::from(""),
                    Line::from(Span::styled("Press [R] to retry", Style::default().fg(palette.accent))),
                    Line::from(Span::styled("Press [Esc] to exit", Style::default().fg(palette.muted))),
                ]
            } else {
                vec![
                    Line::from(""),
                    Line::from(Span::styled(" FAILED GUESS! ", Style::default().fg(palette.danger).add_modifier(Modifier::BOLD))),
                    Line::from(format!("Word: {}", self.secret_word)),
                    Line::from(""),
                    Line::from(Span::styled("Press [R] to retry", Style::default().fg(palette.accent))),
                    Line::from(Span::styled("Press [Esc] to exit", Style::default().fg(palette.muted))),
                ]
            };

            let go_widget = Paragraph::new(message)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(if self.won { palette.accent } else { palette.danger })));
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
