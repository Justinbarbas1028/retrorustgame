use std::time::Duration;
use ratatui::{
    Frame,
    layout::{Rect, Layout, Constraint, Direction, Alignment},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Paragraph, Clear},
};
use crossterm::event::KeyCode;
use super::{Game, GameCommand};

const GRID_WIDTH: i32 = 30;
const GRID_HEIGHT: i32 = 15;

pub struct PongGame {
    player_y: f32,
    ai_y: f32,
    ball_x: f32,
    ball_y: f32,
    vel_x: f32,
    vel_y: f32,
    player_score: u32,
    ai_score: u32,
    game_over: bool,
    paused: bool,
    winner: Option<&'static str>,
}

impl Default for PongGame {
    fn default() -> Self {
        Self::new()
    }
}

impl PongGame {
    pub fn new() -> Self {
        let mut game = Self {
            player_y: (GRID_HEIGHT / 2) as f32,
            ai_y: (GRID_HEIGHT / 2) as f32,
            ball_x: (GRID_WIDTH / 2) as f32,
            ball_y: (GRID_HEIGHT / 2) as f32,
            vel_x: 0.0,
            vel_y: 0.0,
            player_score: 0,
            ai_score: 0,
            game_over: false,
            paused: false,
            winner: None,
        };
        game.reset_ball(true); // Serve to player first
        game
    }

    fn reset_ball(&mut self, serve_to_player: bool) {
        self.ball_x = (GRID_WIDTH / 2) as f32;
        self.ball_y = (GRID_HEIGHT / 2) as f32;
        self.vel_x = if serve_to_player { -14.0 } else { 14.0 };
        // Randomize initial vertical velocity direction
        let rng_y = if rand::random::<bool>() { 6.0 } else { -6.0 };
        self.vel_y = rng_y;
    }
}

impl Game for PongGame {
    fn update(&mut self, delta: Duration) {
        if self.game_over || self.paused {
            return;
        }

        let dt = delta.as_secs_f32();

        // 1. Move Ball
        self.ball_x += self.vel_x * dt;
        self.ball_y += self.vel_y * dt;

        // 2. Ball Collisions with Top/Bottom Walls
        if self.ball_y <= 1.0 {
            self.ball_y = 1.0;
            self.vel_y = -self.vel_y;
        } else if self.ball_y >= (GRID_HEIGHT - 2) as f32 {
            self.ball_y = (GRID_HEIGHT - 2) as f32;
            self.vel_y = -self.vel_y;
        }

        // 3. AI Paddle Logic (Moves towards ball Y, with speed limit to remain beatable)
        let ai_speed = 8.5; // units per second
        let target_y = self.ball_y;
        let diff_y = target_y - self.ai_y;
        if diff_y.abs() > 0.5 {
            let movement = ai_speed * dt * diff_y.signum();
            if movement.abs() < diff_y.abs() {
                self.ai_y += movement;
            } else {
                self.ai_y = target_y;
            }
        }
        // Constrain AI paddle inside screen boundary
        self.ai_y = self.ai_y.clamp(2.0, (GRID_HEIGHT - 3) as f32);

        // 4. Ball Collisions with Player Paddle (Left paddle at x = 2)
        if self.vel_x < 0.0 && self.ball_x <= 2.2 && self.ball_x >= 1.0 {
            let p_y = self.player_y;
            if self.ball_y >= p_y - 1.6 && self.ball_y <= p_y + 1.6 {
                // Bounce
                self.ball_x = 2.2;
                self.vel_x = -self.vel_x * 1.1; // Speed up
                
                // Add spin based on where the ball hits the paddle
                let hit_offset = self.ball_y - p_y;
                self.vel_y = hit_offset * 9.0;
                
                // Clamp speed to avoid absolute madness
                self.vel_x = self.vel_x.clamp(14.0, 35.0);
            }
        }

        // 5. Ball Collisions with AI Paddle (Right paddle at x = GRID_WIDTH - 3)
        if self.vel_x > 0.0 && self.ball_x >= (GRID_WIDTH as f32 - 3.2) && self.ball_x <= (GRID_WIDTH as f32 - 2.0) {
            let a_y = self.ai_y;
            if self.ball_y >= a_y - 1.6 && self.ball_y <= a_y + 1.6 {
                // Bounce
                self.ball_x = GRID_WIDTH as f32 - 3.2;
                self.vel_x = -self.vel_x * 1.1; // Speed up
                
                // Add spin
                let hit_offset = self.ball_y - a_y;
                self.vel_y = hit_offset * 9.0;
                
                self.vel_x = self.vel_x.clamp(-35.0, -14.0);
            }
        }

        // 6. Point Scoring / Out of Bounds
        if self.ball_x < 0.5 {
            // AI scores
            self.ai_score += 1;
            if self.ai_score >= 5 {
                self.game_over = true;
                self.winner = Some("COMPUTER");
            } else {
                self.reset_ball(false); // Serve to AI
            }
        } else if self.ball_x > (GRID_WIDTH - 1) as f32 - 0.5 {
            // Player scores
            self.player_score += 1;
            if self.player_score >= 5 {
                self.game_over = true;
                self.winner = Some("YOU");
            } else {
                self.reset_ball(true); // Serve to player
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

        // Real-time paddle controls (Up/Down W/S)
        match key {
            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                self.player_y = (self.player_y - 1.0).max(2.0);
            }
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                self.player_y = (self.player_y + 1.0).min((GRID_HEIGHT - 3) as f32);
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
            .title(" PONG CABINET ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

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

        // Draw Board Border
        let board_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray));
        
        let board_inner = board_block.inner(board_area);
        frame.render_widget(board_block, board_area);

        // Build display grid buffer
        let mut visual_grid = [[None; GRID_WIDTH as usize]; GRID_HEIGHT as usize];

        // Draw dotted center line
        for y in 1..GRID_HEIGHT as usize - 1 {
            if y % 2 == 1 {
                visual_grid[y][GRID_WIDTH as usize / 2] = Some(Style::default().fg(Color::Rgb(60, 60, 60)));
            }
        }

        // Draw Player Paddle (left: x = 1)
        let py_center = self.player_y.round() as i32;
        for dy in -1..=1 {
            let y = py_center + dy;
            if y >= 1 && y < GRID_HEIGHT - 1 {
                visual_grid[y as usize][1] = Some(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD));
            }
        }

        // Draw AI Paddle (right: x = GRID_WIDTH - 2)
        let ay_center = self.ai_y.round() as i32;
        for dy in -1..=1 {
            let y = ay_center + dy;
            if y >= 1 && y < GRID_HEIGHT - 1 {
                visual_grid[y as usize][GRID_WIDTH as usize - 2] = Some(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
            }
        }

        // Draw Ball
        let bx = self.ball_x.round() as i32;
        let by = self.ball_y.round() as i32;
        if bx >= 0 && bx < GRID_WIDTH && by >= 0 && by < GRID_HEIGHT {
            visual_grid[by as usize][bx as usize] = Some(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
        }

        // Render buffer to vector of lines
        let mut rows = Vec::new();
        for y in 0..GRID_HEIGHT as usize {
            let mut line_spans = Vec::new();
            for x in 0..GRID_WIDTH as usize {
                // Board boundaries top/bottom
                if y == 0 || y == GRID_HEIGHT as usize - 1 {
                    line_spans.push(Span::styled("══", Style::default().fg(Color::DarkGray)));
                } else if let Some(style) = visual_grid[y][x] {
                    if x == 1 {
                        line_spans.push(Span::styled("█▌", style));
                    } else if x == GRID_WIDTH as usize - 2 {
                        line_spans.push(Span::styled("▐█", style));
                    } else if x == bx as usize && y == by as usize {
                        line_spans.push(Span::styled("⬤ ", style));
                    } else {
                        // Dotted net center line
                        line_spans.push(Span::styled("│ ", style));
                    }
                } else {
                    line_spans.push(Span::raw("  "));
                }
            }
            rows.push(Line::from(line_spans));
        }

        let board_paragraph = Paragraph::new(rows);
        frame.render_widget(board_paragraph, board_inner);

        // Sidebar Panel
        let side_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8), // Scoreboard
                Constraint::Min(6),    // Keys Panel
            ])
            .split(side_area);

        let scoreboard_content = vec![
            Line::from(vec![
                Span::styled(" YOU     ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::styled(format!(" {}", self.player_score), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(" ═══════════"),
            Line::from(vec![
                Span::styled(" AI      ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::styled(format!(" {}", self.ai_score), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" BALL SPEED", Style::default().fg(Color::Yellow)),
                Span::styled(format!("  {}% ", (self.vel_x.abs() / 14.0 * 100.0) as u32), Style::default().fg(Color::White)),
            ]),
        ];

        let scoreboard_paragraph = Paragraph::new(scoreboard_content)
            .block(Block::default().borders(Borders::ALL).title("SCOREBOARD"));
        frame.render_widget(scoreboard_paragraph, side_layout[0]);

        let instruct_content = vec![
            Line::from(Span::styled("  [Up/Down]  Paddle Up/Dn", Style::default().fg(Color::Gray))),
            Line::from(Span::styled("  [W / S]    Paddle Up/Dn", Style::default().fg(Color::Gray))),
            Line::from(""),
            Line::from(Span::styled("  [P]        Pause Game", Style::default().fg(Color::Gray))),
            Line::from(Span::styled("  [Esc]      Exit Game", Style::default().fg(Color::Gray))),
        ];

        let instruct_paragraph = Paragraph::new(instruct_content)
            .block(Block::default().borders(Borders::ALL).title("KEYS"));
        frame.render_widget(instruct_paragraph, side_layout[1]);

        // Draw Overlays
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
                x: board_inner.x + (GRID_WIDTH * 2 - 22) as u16 / 2,
                y: board_inner.y + 3,
                width: 22,
                height: 7,
            };
            frame.render_widget(Clear, go_area);
            
            let win_style = if self.winner == Some("YOU") {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            };

            let message = vec![
                Line::from(""),
                Line::from(Span::styled(format!(" {} WINS! ", self.winner.unwrap_or("")), win_style)),
                Line::from(""),
                Line::from(Span::styled("Press [R] to replay", Style::default().fg(Color::Green))),
                Line::from(Span::styled("Press [Esc] to exit", Style::default().fg(Color::DarkGray))),
            ];

            let go_widget = Paragraph::new(message)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_style(win_style));
            frame.render_widget(go_widget, go_area);
        }
    }

    fn get_score(&self) -> u32 {
        // High score calculation: 1000 per points scored, with bonus if AI got few/no points
        if self.player_score == 5 {
            5000 + (5 - self.ai_score) * 1000
        } else {
            self.player_score * 1000
        }
    }

    fn is_game_over(&self) -> bool {
        self.game_over
    }
}
