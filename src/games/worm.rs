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

const BOARD_WIDTH: usize = 30;
const BOARD_HEIGHT: usize = 10;

pub struct WormGame {
    title: &'static str,
    player_y: i32,
    player_jumping: bool,
    jump_time: i32,
    projectiles: Vec<(i32, i32)>,
    obstacles: Vec<(i32, i32)>,
    score: u32,
    game_over: bool,
    paused: bool,
    tick_accumulator: Duration,
}

impl WormGame {
    pub fn new() -> Self {
        Self {
            title: "Worm",
            player_y: BOARD_HEIGHT as i32 - 2,
            player_jumping: false,
            jump_time: 0,
            projectiles: Vec::new(),
            obstacles: Vec::new(),
            score: 0,
            game_over: false,
            paused: false,
            tick_accumulator: Duration::from_secs(0),
        }
    }

    fn get_tick_rate(&self) -> Duration {
        Duration::from_millis(100)
    }
}

impl Default for WormGame {
    fn default() -> Self {
        Self::new()
    }
}

impl Game for WormGame {
    fn update(&mut self, delta: Duration) {
        if self.game_over || self.paused {
            return;
        }

        self.tick_accumulator += delta;
        let tick_rate = self.get_tick_rate();

        if self.tick_accumulator >= tick_rate {
            self.tick_accumulator = Duration::from_secs(0);

            // Handle jumping mechanics
            if self.player_jumping {
                self.jump_time += 1;
                if self.jump_time == 1 {
                    self.player_y = BOARD_HEIGHT as i32 - 4;
                } else if self.jump_time == 2 {
                    self.player_y = BOARD_HEIGHT as i32 - 4;
                } else {
                    self.player_y = BOARD_HEIGHT as i32 - 2;
                    self.player_jumping = false;
                    self.jump_time = 0;
                }
            }

            // Move projectiles
            for p in &mut self.projectiles {
                p.0 += 1;
            }
            self.projectiles.retain(|&p| p.0 >= 0 && p.0 < BOARD_WIDTH as i32);

            // Spawn obstacles
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.05) {
                self.obstacles.push((BOARD_WIDTH as i32 - 1, BOARD_HEIGHT as i32 - 2));
            }

            // Move obstacles
            for obs in &mut self.obstacles {
                obs.0 -= 1;
            }

            // Check collision with projectile
            let mut hit_idx = None;
            for (p_idx, p) in self.projectiles.iter().enumerate() {
                for (o_idx, obs) in self.obstacles.iter().enumerate() {
                    if (p.0 - obs.0).abs() <= 1 && p.1 == obs.1 {
                        hit_idx = Some((p_idx, o_idx));
                        break;
                    }
                }
            }
            if let Some((p_idx, o_idx)) = hit_idx {
                self.projectiles.remove(p_idx);
                self.obstacles.remove(o_idx);
                self.score += 200;
            }

            // Clean off-screen obstacles and award points
            let old_len = self.obstacles.len();
            self.obstacles.retain(|&obs| obs.0 > 0);
            let cleared = old_len - self.obstacles.len();
            self.score += cleared as u32 * 100;

            // Player Collision check
            for obs in &self.obstacles {
                if obs.0 == 2 && obs.1 == self.player_y {
                    self.game_over = true;
                }
            }
        }
    }

    fn handle_input(&mut self, key: KeyCode) -> GameCommand {
        if self.game_over {
            match key {
                KeyCode::Char('r') | KeyCode::Char('R') => {
                    *self = Self::new();
                    return GameCommand::Restart;
                }
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
            KeyCode::Up | KeyCode::Char(' ') | KeyCode::Char('w') | KeyCode::Char('W') => {
                if !self.player_jumping {
                    self.player_jumping = true;
                    self.jump_time = 0;
                }
            }
            KeyCode::Char('f') | KeyCode::Char('F') | KeyCode::Enter => {
                // Fire projectile
                self.projectiles.push((3, self.player_y));
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
            .title(format!("  {} CABINET  ", self.title.to_uppercase()))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

        frame.render_widget(outer_block, area);

        let inner_area = area.inner(&ratatui::layout::Margin { horizontal: 2, vertical: 1 });
        
        let layouts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length((BOARD_WIDTH * 2 + 2) as u16), // Board Area
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

        // Build visual grid
        let mut visual_grid = [[("  ", Style::default()); BOARD_WIDTH]; BOARD_HEIGHT];

        // Ground
        for x in 0..BOARD_WIDTH {
            visual_grid[BOARD_HEIGHT - 1][x] = ("▔▔", Style::default().fg(Color::Rgb(100, 100, 100)));
        }

        // Render obstacles
        for obs in &self.obstacles {
            if obs.0 >= 0 && obs.0 < BOARD_WIDTH as i32 && obs.1 >= 0 && obs.1 < BOARD_HEIGHT as i32 {
                visual_grid[obs.1 as usize][obs.0 as usize] = ("▲▲", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
            }
        }

        // Render projectiles
        for p in &self.projectiles {
            if p.0 >= 0 && p.0 < BOARD_WIDTH as i32 && p.1 >= 0 && p.1 < BOARD_HEIGHT as i32 {
                visual_grid[p.1 as usize][p.0 as usize] = ("* ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
            }
        }

        // Render Player
        if self.player_y >= 0 && self.player_y < BOARD_HEIGHT as i32 {
            visual_grid[self.player_y as usize][2] = ("█▄", Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD));
        }

        let mut rows = Vec::new();
        for y in 0..BOARD_HEIGHT {
            let mut line_spans = Vec::new();
            for x in 0..BOARD_WIDTH {
                let (sym, style) = visual_grid[y][x];
                line_spans.push(Span::styled(sym, style));
            }
            rows.push(Line::from(line_spans));
        }

        let board_paragraph = Paragraph::new(rows);
        frame.render_widget(board_paragraph, board_inner);

        // Sidebar stats
        let side_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8),
                Constraint::Min(6),
            ])
            .split(side_area);

        let stats_content = vec![
            Line::from(vec![
                Span::styled(" SCORE: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(format!("{}", self.score), Style::default().fg(Color::White)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" ACTION: ", Style::default().fg(Color::Cyan)),
                Span::styled(if self.player_jumping { "JUMPING" } else { "RUNNING" }, Style::default().fg(Color::White)),
            ]),
        ];

        let stats_paragraph = Paragraph::new(stats_content)
            .block(Block::default().borders(Borders::ALL).title("STATS"));
        frame.render_widget(stats_paragraph, side_layout[0]);

        let instruct_content = vec![
            Line::from(Span::styled("  [Space] Jump", Style::default().fg(Color::Gray))),
            Line::from(Span::styled("  [F/Ent] Fire", Style::default().fg(Color::Gray))),
            Line::from(""),
            Line::from(Span::styled("  [P]     Pause", Style::default().fg(Color::Gray))),
            Line::from(Span::styled("  [Esc]   Quit", Style::default().fg(Color::Gray))),
        ];

        let instruct_paragraph = Paragraph::new(instruct_content)
            .block(Block::default().borders(Borders::ALL).title("CONTROLS"));
        frame.render_widget(instruct_paragraph, side_layout[1]);

        if self.paused {
            let pause_area = Rect {
                x: board_inner.x + (BOARD_WIDTH * 2 - 18) as u16 / 2,
                y: board_inner.y + 2,
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
                x: board_inner.x + (BOARD_WIDTH * 2 - 20) as u16 / 2,
                y: board_inner.y + 2,
                width: 20,
                height: 7,
            };
            frame.render_widget(Clear, go_area);
            
            let message = vec![
                Line::from(""),
                Line::from(Span::styled(" IMPACT CRASH! ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))),
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
