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

pub struct Game2048 {
    grid: [[u32; 4]; 4],
    score: u32,
    game_over: bool,
    paused: bool,
}

impl Default for Game2048 {
    fn default() -> Self {
        Self::new()
    }
}

impl Game2048 {
    pub fn new() -> Self {
        let mut game = Self {
            grid: [[0; 4]; 4],
            score: 0,
            game_over: false,
            paused: false,
        };
        game.spawn_tile();
        game.spawn_tile();
        game
    }

    fn spawn_tile(&mut self) {
        let mut empty_cells = Vec::new();
        for r in 0..4 {
            for c in 0..4 {
                if self.grid[r][c] == 0 {
                    empty_cells.push((r, c));
                }
            }
        }

        if !empty_cells.is_empty() {
            let mut rng = rand::thread_rng();
            let idx = rng.gen_range(0..empty_cells.len());
            let (r, c) = empty_cells[idx];
            // 90% chance of spawning 2, 10% chance of spawning 4
            self.grid[r][c] = if rng.gen_bool(0.9) { 2 } else { 4 };
        }
    }

    fn check_game_over(&self) -> bool {
        // Check for empty cells
        for r in 0..4 {
            for c in 0..4 {
                if self.grid[r][c] == 0 {
                    return false;
                }
            }
        }

        // Check for horizontal merges
        for r in 0..4 {
            for c in 0..3 {
                if self.grid[r][c] == self.grid[r][c + 1] {
                    return false;
                }
            }
        }

        // Check for vertical merges
        for r in 0..3 {
            for c in 0..4 {
                if self.grid[r][c] == self.grid[r + 1][c] {
                    return false;
                }
            }
        }

        true
    }

    fn transpose(grid: &[[u32; 4]; 4]) -> [[u32; 4]; 4] {
        let mut temp = [[0; 4]; 4];
        for r in 0..4 {
            for c in 0..4 {
                temp[c][r] = grid[r][c];
            }
        }
        temp
    }

    fn slide_left(&mut self) -> bool {
        let mut changed = false;
        let mut new_grid = [[0; 4]; 4];
        
        for r in 0..4 {
            let mut row_score = 0;
            let (new_row, row_changed) = self.slide_row_left(&self.grid[r], &mut row_score);
            new_grid[r] = new_row;
            if row_changed {
                changed = true;
            }
            self.score += row_score;
        }

        self.grid = new_grid;
        changed
    }

    fn slide_row_left(&self, row: &[u32; 4], score_acc: &mut u32) -> ([u32; 4], bool) {
        let cleaned: Vec<u32> = row.iter().copied().filter(|&x| x != 0).collect();
        let mut merged = Vec::new();
        let mut i = 0;
        let mut points = 0;

        while i < cleaned.len() {
            if i + 1 < cleaned.len() && cleaned[i] == cleaned[i + 1] {
                let double = cleaned[i] * 2;
                merged.push(double);
                points += double;
                i += 2;
            } else {
                merged.push(cleaned[i]);
                i += 1;
            }
        }

        *score_acc += points;
        while merged.len() < 4 {
            merged.push(0);
        }

        let mut out = [0; 4];
        out.copy_from_slice(&merged[0..4]);
        (out, out != *row)
    }

    fn slide_right(&mut self) -> bool {
        let mut changed = false;
        let mut new_grid = [[0; 4]; 4];

        for r in 0..4 {
            let mut row = self.grid[r];
            row.reverse();
            let mut row_score = 0;
            let (mut new_row, row_changed) = self.slide_row_left(&row, &mut row_score);
            new_row.reverse();
            new_grid[r] = new_row;
            if row_changed {
                changed = true;
            }
            self.score += row_score;
        }

        self.grid = new_grid;
        changed
    }

    fn slide_up(&mut self) -> bool {
        let transposed = Self::transpose(&self.grid);
        let mut changed = false;
        let mut new_transposed = [[0; 4]; 4];

        for r in 0..4 {
            let mut row_score = 0;
            let (new_row, row_changed) = self.slide_row_left(&transposed[r], &mut row_score);
            new_transposed[r] = new_row;
            if row_changed {
                changed = true;
            }
            self.score += row_score;
        }

        self.grid = Self::transpose(&new_transposed);
        changed
    }

    fn slide_down(&mut self) -> bool {
        let transposed = Self::transpose(&self.grid);
        let mut changed = false;
        let mut new_transposed = [[0; 4]; 4];

        for r in 0..4 {
            let mut row = transposed[r];
            row.reverse();
            let mut row_score = 0;
            let (mut new_row, row_changed) = self.slide_row_left(&row, &mut row_score);
            new_row.reverse();
            new_transposed[r] = new_row;
            if row_changed {
                changed = true;
            }
            self.score += row_score;
        }

        self.grid = Self::transpose(&new_transposed);
        changed
    }

    fn get_tile_colors(val: u32) -> (Color, Color) {
        // Returns (Foreground/Text, Background)
        match val {
            0 => (Color::DarkGray, Color::Rgb(30, 30, 30)),
            2 => (Color::White, Color::Rgb(70, 70, 70)),
            4 => (Color::White, Color::Rgb(95, 95, 95)),
            8 => (Color::White, Color::Rgb(242, 177, 121)),
            16 => (Color::White, Color::Rgb(245, 149, 99)),
            32 => (Color::White, Color::Rgb(246, 124, 95)),
            64 => (Color::White, Color::Rgb(246, 94, 59)),
            128 => (Color::Black, Color::Rgb(237, 207, 114)),
            256 => (Color::Black, Color::Rgb(237, 204, 97)),
            512 => (Color::Black, Color::Rgb(237, 200, 80)),
            1024 => (Color::Black, Color::Rgb(237, 197, 63)),
            2048 => (Color::White, Color::Rgb(237, 194, 46)),
            _ => (Color::Magenta, Color::Rgb(128, 0, 128)),
        }
    }
}

impl Game for Game2048 {
    fn update(&mut self, _delta: Duration) {
        // 2048 is strictly turn-based, so update is passive.
    }

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

        let moved = match key {
            KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => self.slide_left(),
            KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => self.slide_right(),
            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => self.slide_up(),
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => self.slide_down(),
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => return GameCommand::Exit,
            _ => false,
        };

        if moved {
            self.spawn_tile();
            if self.check_game_over() {
                self.game_over = true;
            }
        }

        GameCommand::None
    }

    fn draw(&self, frame: &mut Frame, area: Rect) {
        let outer_block = Block::default()
            .title(" 2048 SLIDER ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

        frame.render_widget(outer_block, area);

        let inner_area = area.inner(&ratatui::layout::Margin { horizontal: 2, vertical: 1 });
        
        let layouts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(60), // Game Board Grid
                Constraint::Percentage(40), // Side Stats & Help
            ])
            .split(inner_area);

        let board_area = layouts[0];
        let side_area = layouts[1];

        // Draw 4x4 Grid Board
        let board_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Gray));
        
        let board_inner = board_block.inner(board_area);
        frame.render_widget(board_block, board_area);

        // Subdivide board into 4 rows
        let row_constraints = vec![Constraint::Ratio(1, 4); 4];
        let row_areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints(row_constraints)
            .split(board_inner);

        // Render each cell
        for r in 0..4 {
            let col_areas = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Ratio(1, 4); 4])
                .split(row_areas[r]);

            for c in 0..4 {
                let cell_val = self.grid[r][c];
                let (fg, bg) = Self::get_tile_colors(cell_val);
                
                let tile_block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(bg));

                frame.render_widget(tile_block.clone(), col_areas[c]);

                let tile_inner = tile_block.inner(col_areas[c]);
                
                // Draw tile content
                let content = if cell_val == 0 {
                    vec![Line::from("")]
                } else {
                    let padding = if tile_inner.height > 1 {
                        vec![Line::from(""); (tile_inner.height as usize - 1) / 2]
                    } else {
                        vec![]
                    };
                    
                    let mut lines = padding;
                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("{}", cell_val),
                            Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD)
                        )
                    ]));
                    lines
                };

                let cell_paragraph = Paragraph::new(content)
                    .alignment(Alignment::Center)
                    .style(Style::default().bg(bg));

                frame.render_widget(cell_paragraph, tile_inner);
            }
        }

        // Draw Sidebar Score & Controls
        let side_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6), // Score Panel
                Constraint::Min(6),    // Controls
            ])
            .split(side_area);

        let score_content = vec![
            Line::from(vec![
                Span::styled(" SCORE ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled(format!(" {:06}", self.score), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
        ];
        
        let score_paragraph = Paragraph::new(score_content)
            .block(Block::default().borders(Borders::ALL).title("STATS"))
            .alignment(Alignment::Center);
        frame.render_widget(score_paragraph, side_layout[0]);

        let instruct_content = vec![
            Line::from(Span::styled("  [↑↓←→] or [WASD]", Style::default().fg(Color::Cyan))),
            Line::from(Span::styled("  Slide Tiles", Style::default().fg(Color::White))),
            Line::from(""),
            Line::from(Span::styled("  [P]   Pause Game", Style::default().fg(Color::Gray))),
            Line::from(Span::styled("  [Esc] Quit Arcade", Style::default().fg(Color::Gray))),
        ];
        
        let instruct_paragraph = Paragraph::new(instruct_content)
            .block(Block::default().borders(Borders::ALL).title("KEYS"));
        frame.render_widget(instruct_paragraph, side_layout[1]);

        // Overlays
        if self.paused {
            let pause_area = Rect {
                x: board_inner.x + 3,
                y: board_inner.y + 3,
                width: board_inner.width.saturating_sub(6).max(16),
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
                x: board_inner.x + 2,
                y: board_inner.y + 2,
                width: board_inner.width.saturating_sub(4).max(18),
                height: 7,
            };
            frame.render_widget(Clear, go_area);
            let go_widget = Paragraph::new(vec![
                Line::from(""),
                Line::from(Span::styled(" GAME OVER! ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))),
                Line::from(format!("Final Score: {}", self.score)),
                Line::from(""),
                Line::from(Span::styled("Press [R] to retry", Style::default().fg(Color::Green))),
                Line::from(Span::styled("Press [Esc] to exit", Style::default().fg(Color::DarkGray))),
            ])
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Red)));
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
