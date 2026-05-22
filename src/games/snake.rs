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

const GRID_WIDTH: i32 = 24;
const GRID_HEIGHT: i32 = 14;

pub struct SnakeGame {
    snake: Vec<(i32, i32)>,
    direction: (i32, i32),
    food: (i32, i32),
    score: u32,
    game_over: bool,
    paused: bool,
    tick_accumulator: Duration,
}

impl Default for SnakeGame {
    fn default() -> Self {
        Self::new()
    }
}

impl SnakeGame {
    pub fn new() -> Self {
        let mut game = Self {
            snake: vec![(10, 7), (9, 7), (8, 7)],
            direction: (1, 0),
            food: (0, 0),
            score: 0,
            game_over: false,
            paused: false,
            tick_accumulator: Duration::from_secs(0),
        };
        game.spawn_food();
        game
    }

    fn spawn_food(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let fx = rng.gen_range(1..GRID_WIDTH - 1);
            let fy = rng.gen_range(1..GRID_HEIGHT - 1);
            // Don't spawn food on the snake
            if !self.snake.iter().any(|&pos| pos == (fx, fy)) {
                self.food = (fx, fy);
                break;
            }
        }
    }

    fn get_tick_rate(&self) -> Duration {
        // Starts at 150ms, speeds up by 5ms for every 200 points scored
        let speed_reduction = (self.score / 200) as u64 * 5;
        let speed_ms = 150u64.saturating_sub(speed_reduction).max(60);
        Duration::from_millis(speed_ms)
    }
}

impl Game for SnakeGame {
    fn update(&mut self, delta: Duration) {
        if self.game_over || self.paused {
            return;
        }

        self.tick_accumulator += delta;
        let tick_rate = self.get_tick_rate();

        if self.tick_accumulator >= tick_rate {
            self.tick_accumulator = Duration::from_secs(0);

            // Calculate next head position
            let head = self.snake[0];
            let next_head = (head.0 + self.direction.0, head.1 + self.direction.1);

            // Collision with boundaries
            if next_head.0 <= 0 || next_head.0 >= GRID_WIDTH - 1 || next_head.1 <= 0 || next_head.1 >= GRID_HEIGHT - 1 {
                self.game_over = true;
                return;
            }

            // Collision with tail
            if self.snake.iter().any(|&pos| pos == next_head) {
                self.game_over = true;
                return;
            }

            // Insert new head
            self.snake.insert(0, next_head);

            // Eat food check
            if next_head == self.food {
                self.score += 100;
                self.spawn_food();
            } else {
                // Remove tail if didn't eat
                self.snake.pop();
            }
        }
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

        match key {
            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                if self.direction != (0, 1) {
                    self.direction = (0, -1);
                }
            }
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                if self.direction != (0, -1) {
                    self.direction = (0, 1);
                }
            }
            KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
                if self.direction != (1, 0) {
                    self.direction = (-1, 0);
                }
            }
            KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
                if self.direction != (-1, 0) {
                    self.direction = (1, 0);
                }
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
            .title(" SNAKE CABINET ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD));

        frame.render_widget(outer_block, area);

        let inner_area = area.inner(&ratatui::layout::Margin { horizontal: 2, vertical: 1 });
        
        let layouts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length((GRID_WIDTH * 2 + 2) as u16), // Board Area
                Constraint::Min(12),                            // Stats panel
            ])
            .split(inner_area);

        let board_area = layouts[0];
        let side_area = layouts[1];

        // Draw Map Border
        let board_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Gray));
        
        let board_inner = board_block.inner(board_area);
        frame.render_widget(board_block, board_area);

        // Build character map buffer
        let mut visual_grid = [[None; GRID_WIDTH as usize]; GRID_HEIGHT as usize];

        // Draw boundaries visually
        for x in 0..GRID_WIDTH as usize {
            visual_grid[0][x] = Some(Style::default().fg(Color::Rgb(80, 80, 80)));
            visual_grid[GRID_HEIGHT as usize - 1][x] = Some(Style::default().fg(Color::Rgb(80, 80, 80)));
        }
        for y in 0..GRID_HEIGHT as usize {
            visual_grid[y][0] = Some(Style::default().fg(Color::Rgb(80, 80, 80)));
            visual_grid[y][GRID_WIDTH as usize - 1] = Some(Style::default().fg(Color::Rgb(80, 80, 80)));
        }

        // Draw Food
        visual_grid[self.food.1 as usize][self.food.0 as usize] = Some(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));

        // Draw Snake body (head distinct color)
        for (i, &pos) in self.snake.iter().enumerate() {
            if pos.0 >= 0 && pos.0 < GRID_WIDTH && pos.1 >= 0 && pos.1 < GRID_HEIGHT {
                let color = if i == 0 { Color::LightGreen } else { Color::Green };
                visual_grid[pos.1 as usize][pos.0 as usize] = Some(Style::default().fg(color).add_modifier(Modifier::BOLD));
            }
        }

        // Render buffer
        let mut rows = Vec::new();
        for y in 0..GRID_HEIGHT as usize {
            let mut line_spans = Vec::new();
            for x in 0..GRID_WIDTH as usize {
                if let Some(style) = visual_grid[y][x] {
                    if y == self.food.1 as usize && x == self.food.0 as usize {
                        line_spans.push(Span::styled("♥ ", style));
                    } else if self.snake.first() == Some(&(x as i32, y as i32)) {
                        line_spans.push(Span::styled("● ", style));
                    } else if self.snake.contains(&(x as i32, y as i32)) {
                        line_spans.push(Span::styled("o ", style));
                    } else {
                        // Border block
                        line_spans.push(Span::styled("██", style));
                    }
                } else {
                    line_spans.push(Span::raw("  "));
                }
            }
            rows.push(Line::from(line_spans));
        }

        let board_paragraph = Paragraph::new(rows);
        frame.render_widget(board_paragraph, board_inner);

        // Sidebar stats
        let side_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(7), // Stats Panel
                Constraint::Min(6),    // Keys Panel
            ])
            .split(side_area);

        let speed_pct = 150 - self.get_tick_rate().as_millis() as u32;
        let stats_content = vec![
            Line::from(vec![
                Span::styled(" SCORE ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(format!("  {:05}", self.score), Style::default().fg(Color::White)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" SPEED ", Style::default().fg(Color::Cyan)),
                Span::styled(format!("  {} mph", speed_pct + 10), Style::default().fg(Color::White)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" LENGTH", Style::default().fg(Color::LightGreen)),
                Span::styled(format!("  {:03}", self.snake.len()), Style::default().fg(Color::White)),
            ]),
        ];

        let stats_paragraph = Paragraph::new(stats_content)
            .block(Block::default().borders(Borders::ALL).title("STATS"));
        frame.render_widget(stats_paragraph, side_layout[0]);

        let instruct_content = vec![
            Line::from(Span::styled("  [Arrows/WASD] Move", Style::default().fg(Color::Gray))),
            Line::from(""),
            Line::from(Span::styled("  [P]           Pause", Style::default().fg(Color::Gray))),
            Line::from(Span::styled("  [Esc]         Quit", Style::default().fg(Color::Gray))),
        ];

        let instruct_paragraph = Paragraph::new(instruct_content)
            .block(Block::default().borders(Borders::ALL).title("KEYS"));
        frame.render_widget(instruct_paragraph, side_layout[1]);

        // Overlays
        if self.paused {
            let pause_area = Rect {
                x: board_inner.x + (GRID_WIDTH * 2 - 18) as u16 / 2,
                y: board_inner.y + 4,
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
                x: board_inner.x + (GRID_WIDTH * 2 - 20) as u16 / 2,
                y: board_inner.y + 3,
                width: 20,
                height: 7,
            };
            frame.render_widget(Clear, go_area);
            
            let message = vec![
                Line::from(""),
                Line::from(Span::styled(" CRASH DEATH! ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))),
                Line::from(format!("Final Score: {}", self.score)),
                Line::from(""),
                Line::from(Span::styled("Press [R] to retry", Style::default().fg(Color::Green))),
                Line::from(Span::styled("Press [Esc] to exit", Style::default().fg(Color::DarkGray))),
            ];

            let go_widget = Paragraph::new(message)
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
