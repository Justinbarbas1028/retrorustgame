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

const BOARD_SIZE: usize = 10;
const TOTAL_MINES: usize = 15;

#[derive(Clone, Copy, Debug)]
struct Cell {
    is_mine: bool,
    is_revealed: bool,
    is_flagged: bool,
    adjacent_mines: u8,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            is_mine: false,
            is_revealed: false,
            is_flagged: false,
            adjacent_mines: 0,
        }
    }
}

pub struct MinesweeperGame {
    board: [[Cell; BOARD_SIZE]; BOARD_SIZE],
    cursor_x: usize,
    cursor_y: usize,
    first_click: bool,
    game_over: bool,
    won: bool,
    paused: bool,
    score: u32,
}

impl Default for MinesweeperGame {
    fn default() -> Self {
        Self::new()
    }
}

impl MinesweeperGame {
    pub fn new() -> Self {
        Self {
            board: [[Cell::default(); BOARD_SIZE]; BOARD_SIZE],
            cursor_x: BOARD_SIZE / 2,
            cursor_y: BOARD_SIZE / 2,
            first_click: true,
            game_over: false,
            won: false,
            paused: false,
            score: 0,
        }
    }

    fn generate_mines(&mut self, safe_x: usize, safe_y: usize) {
        let mut rng = rand::thread_rng();
        let mut mines_placed = 0;

        while mines_placed < TOTAL_MINES {
            let rx = rng.gen_range(0..BOARD_SIZE);
            let ry = rng.gen_range(0..BOARD_SIZE);

            // Avoid placing on the safe cell (first click) or its immediate neighbors for a nice starting pocket
            let is_safe_zone = (rx as i32 - safe_x as i32).abs() <= 1 && (ry as i32 - safe_y as i32).abs() <= 1;

            if !self.board[ry][rx].is_mine && !is_safe_zone {
                self.board[ry][rx].is_mine = true;
                mines_placed += 1;
            }
        }

        // Calculate adjacent numbers
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                if !self.board[r][c].is_mine {
                    self.board[r][c].adjacent_mines = self.count_adjacent_mines(c, r);
                }
            }
        }
    }

    fn count_adjacent_mines(&self, cx: usize, cy: usize) -> u8 {
        let mut count = 0;
        for r_offset in -1..=1 {
            for c_offset in -1..=1 {
                let r = cy as i32 + r_offset;
                let c = cx as i32 + c_offset;
                if r >= 0 && r < BOARD_SIZE as i32 && c >= 0 && c < BOARD_SIZE as i32 {
                    if self.board[r as usize][c as usize].is_mine {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn reveal_cell(&mut self, cx: usize, cy: usize) {
        if self.board[cy][cx].is_revealed || self.board[cy][cx].is_flagged {
            return;
        }

        if self.first_click {
            self.first_click = false;
            self.generate_mines(cx, cy);
        }

        self.board[cy][cx].is_revealed = true;

        if self.board[cy][cx].is_mine {
            self.game_over = true;
            // Reveal all mines on death
            self.reveal_all_mines();
            return;
        }

        // Increment score
        self.score += 10;

        if self.board[cy][cx].adjacent_mines == 0 {
            // Cascade reveal empty pocket
            for r_offset in -1..=1 {
                for c_offset in -1..=1 {
                    let r = cy as i32 + r_offset;
                    let c = cx as i32 + c_offset;
                    if r >= 0 && r < BOARD_SIZE as i32 && c >= 0 && c < BOARD_SIZE as i32 {
                        self.reveal_cell(c as usize, r as usize);
                    }
                }
            }
        }

        self.check_win_condition();
    }

    fn reveal_all_mines(&mut self) {
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                if self.board[r][c].is_mine {
                    self.board[r][c].is_revealed = true;
                }
            }
        }
    }

    fn toggle_flag(&mut self, cx: usize, cy: usize) {
        if self.board[cy][cx].is_revealed {
            return;
        }
        self.board[cy][cx].is_flagged = !self.board[cy][cx].is_flagged;
    }

    fn check_win_condition(&mut self) {
        let mut cells_to_reveal = 0;
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                if !self.board[r][c].is_mine && !self.board[r][c].is_revealed {
                    cells_to_reveal += 1;
                }
            }
        }

        if cells_to_reveal == 0 {
            self.won = true;
            self.game_over = true;
            self.score += 1000; // Big win bonus!
        }
    }

    fn get_flagged_count(&self) -> usize {
        let mut count = 0;
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                if self.board[r][c].is_flagged {
                    count += 1;
                }
            }
        }
        count
    }

    fn get_cell_style_and_char(&self, x: usize, y: usize) -> (Style, &'static str) {
        let cell = self.board[y][x];
        
        if cell.is_revealed {
            if cell.is_mine {
                (Style::default().fg(Color::Red).add_modifier(Modifier::BOLD), "💣")
            } else if cell.adjacent_mines == 0 {
                (Style::default().fg(Color::DarkGray), "  ")
            } else {
                let color = match cell.adjacent_mines {
                    1 => Color::Blue,
                    2 => Color::Green,
                    3 => Color::Red,
                    4 => Color::Rgb(0, 0, 128), // Navy
                    5 => Color::Rgb(128, 0, 0), // Maroon
                    6 => Color::Rgb(0, 128, 128), // Teal
                    7 => Color::Black,
                    _ => Color::DarkGray,
                };
                let text = match cell.adjacent_mines {
                    1 => "1 ", 2 => "2 ", 3 => "3 ", 4 => "4 ",
                    5 => "5 ", 6 => "6 ", 7 => "7 ", _ => "8 ",
                };
                (Style::default().fg(color).add_modifier(Modifier::BOLD), text)
            }
        } else if cell.is_flagged {
            (Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD), "⚑ ")
        } else {
            (Style::default().fg(Color::Rgb(80, 80, 80)), "■ ")
        }
    }
}

impl Game for MinesweeperGame {
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
            KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
                if self.cursor_x < BOARD_SIZE - 1 {
                    self.cursor_x += 1;
                }
            }
            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                if self.cursor_y < BOARD_SIZE - 1 {
                    self.cursor_y += 1;
                }
            }
            KeyCode::Char(' ') | KeyCode::Enter => {
                self.reveal_cell(self.cursor_x, self.cursor_y);
            }
            KeyCode::Char('f') | KeyCode::Char('F') => {
                self.toggle_flag(self.cursor_x, self.cursor_y);
            }
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                return GameCommand::Exit;
            }
            _ => {}
        }

        GameCommand::None
    }

    fn draw(&self, frame: &mut Frame, area: Rect) {
        let outer_block = Block::default()
            .title(" MINESWEEPER CABINET ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD));

        frame.render_widget(outer_block, area);

        let inner_area = area.inner(&ratatui::layout::Margin { horizontal: 2, vertical: 1 });
        
        let layouts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(24), // Mine Board (10cols * 2chars = 20width + 2 border)
                Constraint::Min(12),    // Stats & controls
            ])
            .split(inner_area);

        let board_area = layouts[0];
        let side_area = layouts[1];

        // Draw Board Border
        let board_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Gray));
        
        let board_inner = board_block.inner(board_area);
        frame.render_widget(board_block, board_area);

        // Build Minesweeper rendering lines
        let mut rows = Vec::new();
        for r in 0..BOARD_SIZE {
            let mut line_spans = Vec::new();
            for c in 0..BOARD_SIZE {
                let (style, symbol) = self.get_cell_style_and_char(c, r);
                
                // Highlight cursor position
                if r == self.cursor_y && c == self.cursor_x {
                    line_spans.push(Span::styled(
                        symbol,
                        style.bg(Color::Rgb(0, 100, 100)).add_modifier(Modifier::UNDERLINED)
                    ));
                } else {
                    line_spans.push(Span::styled(symbol, style));
                }
            }
            rows.push(Line::from(line_spans));
        }

        let board_paragraph = Paragraph::new(rows);
        frame.render_widget(board_paragraph, board_inner);

        // Draw Sidebar Stats & Keys
        let side_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(7), // Score & mines count
                Constraint::Min(6),    // Keys help
            ])
            .split(side_area);

        let flagged = self.get_flagged_count();
        let remaining_mines = (TOTAL_MINES as i32 - flagged as i32).max(0);

        let stats_content = vec![
            Line::from(vec![
                Span::styled(" SCORE ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(format!("  {:05}", self.score), Style::default().fg(Color::White)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" MINES ", Style::default().fg(Color::Red)),
                Span::styled(format!("  {:02}", TOTAL_MINES), Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled(" FLAGS ", Style::default().fg(Color::Yellow)),
                Span::styled(format!("  {:02}", flagged), Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled(" UNUSED", Style::default().fg(Color::Green)),
                Span::styled(format!("  {:02}", remaining_mines), Style::default().fg(Color::White)),
            ]),
        ];

        let stats_paragraph = Paragraph::new(stats_content)
            .block(Block::default().borders(Borders::ALL).title("STATS"));
        frame.render_widget(stats_paragraph, side_layout[0]);

        let instruct_content = vec![
            Line::from(Span::styled("  [Arrows/WASD] Move", Style::default().fg(Color::Gray))),
            Line::from(Span::styled("  [Enter/Spc]  Dig", Style::default().fg(Color::White))),
            Line::from(Span::styled("  [F]          Flag", Style::default().fg(Color::Yellow))),
            Line::from(""),
            Line::from(Span::styled("  [P]          Pause", Style::default().fg(Color::Gray))),
            Line::from(Span::styled("  [Esc]        Quit", Style::default().fg(Color::Gray))),
        ];

        let instruct_paragraph = Paragraph::new(instruct_content)
            .block(Block::default().borders(Borders::ALL).title("KEYS"));
        frame.render_widget(instruct_paragraph, side_layout[1]);

        // Overlays
        if self.paused {
            let pause_area = Rect {
                x: board_inner.x + 1,
                y: board_inner.y + 3,
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
                x: board_inner.x,
                y: board_inner.y + 2,
                width: 20,
                height: 7,
            };
            frame.render_widget(Clear, go_area);
            
            let message = if self.won {
                vec![
                    Line::from(""),
                    Line::from(Span::styled(" YOU CLEAR IT! ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))),
                    Line::from(format!("Score: {}", self.score)),
                    Line::from(""),
                    Line::from(Span::styled("Press [R] to replay", Style::default().fg(Color::Green))),
                    Line::from(Span::styled("Press [Esc] to exit", Style::default().fg(Color::DarkGray))),
                ]
            } else {
                vec![
                    Line::from(""),
                    Line::from(Span::styled(" BOOM! DETONATED ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))),
                    Line::from(format!("Score: {}", self.score)),
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
