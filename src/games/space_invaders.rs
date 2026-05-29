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

const WIDTH: i32 = 30;
const HEIGHT: i32 = 16;

struct Alien {
    x: i32,
    y: i32,
    points: u32,
}

struct Bunker {
    x: i32,
    y: i32,
    health: u8,
}

pub struct SpaceInvadersGame {
    player_x: i32,
    aliens: Vec<Alien>,
    alien_direction: i32, // 1 for right, -1 for left
    player_lasers: Vec<(i32, i32)>,
    alien_lasers: Vec<(i32, i32)>,
    bunkers: Vec<Bunker>,
    lives: u32,
    score: u32,
    wave: u32,
    game_over: bool,
    paused: bool,
    
    // Timers
    laser_timer: Duration,
    alien_move_timer: Duration,
    alien_shoot_timer: Duration,
    laser_move_timer: Duration,
}

impl Default for SpaceInvadersGame {
    fn default() -> Self {
        Self::new()
    }
}

impl SpaceInvadersGame {
    pub fn new() -> Self {
        let mut game = Self {
            player_x: WIDTH / 2,
            aliens: Vec::new(),
            alien_direction: 1,
            player_lasers: Vec::new(),
            alien_lasers: Vec::new(),
            bunkers: Vec::new(),
            lives: 3,
            score: 0,
            wave: 1,
            game_over: false,
            paused: false,
            
            laser_timer: Duration::from_secs(0),
            alien_move_timer: Duration::from_secs(0),
            alien_shoot_timer: Duration::from_secs(0),
            laser_move_timer: Duration::from_secs(0),
        };
        game.init_wave();
        game.init_bunkers();
        game
    }

    fn init_wave(&mut self) {
        self.aliens.clear();
        self.player_lasers.clear();
        self.alien_lasers.clear();
        self.alien_direction = 1;

        // 3 rows of aliens, 6 columns
        for row in 0..3 {
            for col in 0..6 {
                let x = 4 + col * 3;
                let y = 1 + row * 2;
                let points = match row {
                    0 => 30, // Top
                    1 => 20, // Middle
                    _ => 10, // Bottom
                };
                self.aliens.push(Alien { x, y, points });
            }
        }
    }

    fn init_bunkers(&mut self) {
        self.bunkers.clear();
        // 3 bunkers spaced across the screen
        let bunker_x_offsets = [5, 13, 21];
        for &bx in &bunker_x_offsets {
            for offset in 0..3 {
                self.bunkers.push(Bunker {
                    x: bx + offset,
                    y: HEIGHT - 3,
                    health: 3,
                });
            }
        }
    }

    fn fire_player_laser(&mut self) {
        if self.laser_timer == Duration::from_secs(0) {
            self.player_lasers.push((self.player_x, HEIGHT - 2));
            self.laser_timer = Duration::from_millis(300); // 300ms cooldown
        }
    }

    fn get_alien_move_rate(&self) -> Duration {
        let count = self.aliens.len() as u64;
        let base_speed: u64 = match count {
            0..=2 => 100,
            3..=5 => 200,
            6..=10 => 350,
            11..=15 => 500,
            _ => 650,
        };
        // Speed up in later waves
        let wave_bonus = (self.wave - 1) as u64 * 80;
        let speed = base_speed.saturating_sub(wave_bonus).max(60);
        Duration::from_millis(speed)
    }
}

impl Game for SpaceInvadersGame {
    fn update(&mut self, delta: Duration) {
        if self.game_over || self.paused {
            return;
        }

        // 1. Cooldown timers
        self.laser_timer = self.laser_timer.saturating_sub(delta);
        self.laser_move_timer += delta;
        self.alien_move_timer += delta;
        self.alien_shoot_timer += delta;

        // 2. Projectile movement (80ms speed tick)
        if self.laser_move_timer >= Duration::from_millis(80) {
            self.laser_move_timer = Duration::from_secs(0);

            // Move player lasers up
            for laser in &mut self.player_lasers {
                laser.1 -= 1;
            }
            self.player_lasers.retain(|&(_, y)| y >= 0);

            // Move alien lasers down
            for laser in &mut self.alien_lasers {
                laser.1 += 1;
            }
            self.alien_lasers.retain(|&(_, y)| y < HEIGHT);

            // Laser collision checking
            // A. Player Lasers vs. Aliens
            let mut hit_lasers = Vec::new();
            let mut dead_aliens = Vec::new();

            for (l_idx, &(lx, ly)) in self.player_lasers.iter().enumerate() {
                for (a_idx, alien) in self.aliens.iter().enumerate() {
                    if lx == alien.x && ly == alien.y {
                        hit_lasers.push(l_idx);
                        dead_aliens.push(a_idx);
                        self.score += alien.points;
                        break;
                    }
                }
            }

            hit_lasers.sort();
            hit_lasers.dedup();
            for &idx in hit_lasers.iter().rev() {
                if idx < self.player_lasers.len() {
                    self.player_lasers.remove(idx);
                }
            }

            dead_aliens.sort();
            dead_aliens.dedup();
            for &idx in dead_aliens.iter().rev() {
                if idx < self.aliens.len() {
                    self.aliens.remove(idx);
                }
            }

            // Next wave if clear
            if self.aliens.is_empty() {
                self.wave += 1;
                self.score += 500;
                self.init_wave();
                self.init_bunkers();
                return;
            }

            // B. Player Lasers vs. Bunkers
            let mut hit_lasers = Vec::new();
            for (l_idx, &(lx, ly)) in self.player_lasers.iter().enumerate() {
                for bunker in &mut self.bunkers {
                    if bunker.health > 0 && lx == bunker.x && ly == bunker.y {
                        bunker.health -= 1;
                        hit_lasers.push(l_idx);
                        break;
                    }
                }
            }
            hit_lasers.sort();
            hit_lasers.dedup();
            for &idx in hit_lasers.iter().rev() {
                if idx < self.player_lasers.len() {
                    self.player_lasers.remove(idx);
                }
            }

            // C. Alien Lasers vs. Bunkers
            let mut hit_lasers = Vec::new();
            for (l_idx, &(lx, ly)) in self.alien_lasers.iter().enumerate() {
                for bunker in &mut self.bunkers {
                    if bunker.health > 0 && lx == bunker.x && ly == bunker.y {
                        bunker.health -= 1;
                        hit_lasers.push(l_idx);
                        break;
                    }
                }
            }
            hit_lasers.sort();
            hit_lasers.dedup();
            for &idx in hit_lasers.iter().rev() {
                if idx < self.alien_lasers.len() {
                    self.alien_lasers.remove(idx);
                }
            }

            // D. Alien Lasers vs. Player
            let mut hit_lasers = Vec::new();
            for (l_idx, &(lx, ly)) in self.alien_lasers.iter().enumerate() {
                if lx == self.player_x && ly == HEIGHT - 2 {
                    hit_lasers.push(l_idx);
                    if self.lives > 1 {
                        self.lives -= 1;
                    } else {
                        self.lives = 0;
                        self.game_over = true;
                    }
                }
            }
            for &idx in hit_lasers.iter().rev() {
                if idx < self.alien_lasers.len() {
                    self.alien_lasers.remove(idx);
                }
            }
        }

        // 3. Move Alien Swarm
        let alien_move_rate = self.get_alien_move_rate();
        if self.alien_move_timer >= alien_move_rate {
            self.alien_move_timer = Duration::from_secs(0);

            let mut hit_border = false;
            for alien in &self.aliens {
                let next_x = alien.x + self.alien_direction;
                if next_x < 0 || next_x >= WIDTH {
                    hit_border = true;
                    break;
                }
            }

            if hit_border {
                // Drop down by 1 row and reverse horizontal direction
                self.alien_direction = -self.alien_direction;
                for alien in &mut self.aliens {
                    alien.y += 1;
                    if alien.y >= HEIGHT - 2 {
                        self.game_over = true;
                    }
                }
            } else {
                // Move horizontal
                for alien in &mut self.aliens {
                    alien.x += self.alien_direction;
                }
            }
        }

        // 4. Random Alien Shoot Ticks (1.2 seconds rate)
        if self.alien_shoot_timer >= Duration::from_millis(1200) {
            self.alien_shoot_timer = Duration::from_secs(0);
            if !self.aliens.is_empty() {
                let mut rng = rand::thread_rng();
                let idx = rng.gen_range(0..self.aliens.len());
                let shooter = &self.aliens[idx];
                self.alien_lasers.push((shooter.x, shooter.y + 1));
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
            KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
                if self.player_x > 0 {
                    self.player_x -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
                if self.player_x < WIDTH - 1 {
                    self.player_x += 1;
                }
            }
            KeyCode::Char(' ') | KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                self.fire_player_laser();
            }
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                return GameCommand::Exit;
            }
            _ => {}
        }

        GameCommand::None
    }

    fn draw(&self, frame: &mut Frame, area: Rect, palette: &ThemePalette) {
        let outer_block = Block::default()
            .title(" SPACE INVADERS CABINET ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(palette.accent_alt).add_modifier(Modifier::BOLD));

        frame.render_widget(outer_block, area);

        let inner_area = area.inner(&ratatui::layout::Margin { horizontal: 2, vertical: 1 });
        
        let layouts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length((WIDTH * 2 + 2) as u16), // 30 cols * 2 + 2 borders = 62 width
                Constraint::Min(12),                       // Stats panel
            ])
            .split(inner_area);

        let board_area = layouts[0];
        let side_area = layouts[1];

        let board_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(palette.muted));
        
        let board_inner = board_block.inner(board_area);
        frame.render_widget(board_block, board_area);

        // Grid double-buffer grid
        let mut visual_grid = [[None; WIDTH as usize]; HEIGHT as usize];

        // 1. Draw Bunkers
        for bunker in &self.bunkers {
            if bunker.health > 0 {
                let color = match bunker.health {
                    3 => palette.accent,
                    2 => palette.accent,
                    _ => palette.muted,
                };
                visual_grid[bunker.y as usize][bunker.x as usize] = Some(Style::default().fg(color));
            }
        }

        // 2. Draw Player lasers
        for &(lx, ly) in &self.player_lasers {
            if ly >= 0 && ly < HEIGHT {
                visual_grid[ly as usize][lx as usize] = Some(Style::default().fg(palette.accent_alt).add_modifier(Modifier::BOLD));
            }
        }

        // 3. Draw Alien lasers
        for &(lx, ly) in &self.alien_lasers {
            if ly >= 0 && ly < HEIGHT {
                visual_grid[ly as usize][lx as usize] = Some(Style::default().fg(palette.danger));
            }
        }

        // 4. Draw Aliens
        for alien in &self.aliens {
            let color = match alien.points {
                30 => palette.accent_alt,
                20 => palette.accent,
                _ => palette.accent,
            };
            visual_grid[alien.y as usize][alien.x as usize] = Some(Style::default().fg(color));
        }

        // 5. Draw Player
        if !self.game_over {
            visual_grid[HEIGHT as usize - 2][self.player_x as usize] = Some(Style::default().fg(palette.accent).add_modifier(Modifier::BOLD));
        }

        // Build character output lines
        let mut rows = Vec::new();
        for y in 0..HEIGHT as usize {
            let mut line_spans = Vec::new();
            for x in 0..WIDTH as usize {
                if let Some(style) = visual_grid[y][x] {
                    if y == HEIGHT as usize - 2 && x == self.player_x as usize {
                        line_spans.push(Span::styled("▲▲", style));
                    } else if self.player_lasers.contains(&(x as i32, y as i32)) {
                        line_spans.push(Span::styled("║ ", style));
                    } else if self.alien_lasers.contains(&(x as i32, y as i32)) {
                        line_spans.push(Span::styled("│ ", style));
                    } else if self.bunkers.iter().any(|b| b.health > 0 && b.x == x as i32 && b.y == y as i32) {
                        let bunker_ref = self.bunkers.iter().find(|b| b.x == x as i32 && b.y == y as i32).unwrap();
                        let char_symbol = match bunker_ref.health {
                            3 => "██",
                            2 => "▒▒",
                            _ => "░░",
                        };
                        line_spans.push(Span::styled(char_symbol, style));
                    } else {
                        let alien_ref = self.aliens.iter().find(|a| a.x == x as i32 && a.y == y as i32);
                        if let Some(alien) = alien_ref {
                            let alien_char = match alien.points {
                                30 => "👾",
                                20 => "🛸",
                                _ => "👽",
                            };
                            line_spans.push(Span::styled(alien_char, style));
                        } else {
                            line_spans.push(Span::raw("  "));
                        }
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
                Constraint::Length(8), // Stats
                Constraint::Min(6),    // Keys Help
            ])
            .split(side_area);

        let mut lives_span = Vec::new();
        for _ in 0..self.lives {
            lives_span.push(Span::styled("▲ ", Style::default().fg(palette.accent)));
        }
        while lives_span.len() < 3 {
            lives_span.push(Span::styled(". ", Style::default().fg(palette.muted)));
        }

        let stats_content = vec![
            Line::from(vec![
                Span::styled(" SCORE ", Style::default().fg(palette.accent_alt).add_modifier(Modifier::BOLD)),
                Span::styled(format!("  {:05}", self.score), Style::default().fg(palette.body)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" WAVE  ", Style::default().fg(palette.accent_alt)),
                Span::styled(format!("  {:02}", self.wave), Style::default().fg(palette.body).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" SHIPS ", Style::default().fg(palette.accent)),
            ]),
            Line::from(lives_span),
        ];

        let stats_paragraph = Paragraph::new(stats_content)
            .block(Block::default().borders(Borders::ALL).title("STATS"));
        frame.render_widget(stats_paragraph, side_layout[0]);

        let instruct_content = vec![
            Line::from(Span::styled("  [←→]/[A/D]  Move", Style::default().fg(palette.muted))),
            Line::from(Span::styled("  [Spc]/[W]   Shoot", Style::default().fg(palette.body))),
            Line::from(""),
            Line::from(Span::styled("  [Tab]       Pause", Style::default().fg(palette.muted))),
            Line::from(Span::styled("  [Esc]       Quit", Style::default().fg(palette.muted))),
        ];

        let instruct_paragraph = Paragraph::new(instruct_content)
            .block(Block::default().borders(Borders::ALL).title("KEYS"));
        frame.render_widget(instruct_paragraph, side_layout[1]);

        // Overlays
        if self.paused {
            let pause_area = Rect {
                x: board_inner.x + (WIDTH * 2 - 18) as u16 / 2,
                y: board_inner.y + 5,
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
                x: board_inner.x + (WIDTH * 2 - 20) as u16 / 2,
                y: board_inner.y + 4,
                width: 20,
                height: 7,
            };
            frame.render_widget(Clear, go_area);
            
            let message = vec![
                Line::from(""),
                Line::from(Span::styled(" INVASION COMPLETE ", Style::default().fg(palette.danger).add_modifier(Modifier::BOLD))),
                Line::from(format!("Final Score: {}", self.score)),
                Line::from(""),
                Line::from(Span::styled("Press [R] to retry", Style::default().fg(palette.accent))),
                Line::from(Span::styled("Press [Esc] to exit", Style::default().fg(palette.muted))),
            ];

            let go_widget = Paragraph::new(message)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(palette.danger)));
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
