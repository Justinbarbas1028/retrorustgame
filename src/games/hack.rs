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

const MAP_WIDTH: usize = 20;
const MAP_HEIGHT: usize = 12;

pub struct HackGame {
    title: &'static str,
    player_pos: (i32, i32),
    player_hp: i32,
    player_max_hp: i32,
    player_atk: i32,
    player_def: i32,
    score: u32,
    level: u32,
    game_over: bool,
    paused: bool,
    enemies: Vec<Enemy>,
    items: Vec<Item>,
    map: [[char; MAP_WIDTH]; MAP_HEIGHT],
    log: Vec<String>,
}

struct Enemy {
    pos: (i32, i32),
    hp: i32,
    symbol: char,
    name: &'static str,
    color: Color,
}

struct Item {
    pos: (i32, i32),
    symbol: char,
    name: &'static str,
    color: Color,
}

impl HackGame {
    pub fn new() -> Self {
        let mut game = Self {
            title: "Hack",
            player_pos: (2, 2),
            player_hp: 25,
            player_max_hp: 25,
            player_atk: 5,
            player_def: 1,
            score: 0,
            level: 1,
            game_over: false,
            paused: false,
            enemies: Vec::new(),
            items: Vec::new(),
            map: [['.'; MAP_WIDTH]; MAP_HEIGHT],
            log: vec!["Welcome to the depths!".to_string()],
        };
        game.generate_level();
        game
    }

    fn generate_level(&mut self) {
        self.enemies.clear();
        self.items.clear();
        
        // Build walls
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if x == 0 || x == MAP_WIDTH - 1 || y == 0 || y == MAP_HEIGHT - 1 {
                    self.map[y][x] = '#';
                } else {
                    self.map[y][x] = '.';
                }
            }
        }

        let mut rng = rand::thread_rng();

        // Add internal pillars
        for _ in 0..10 {
            let px = rng.gen_range(2..MAP_WIDTH - 2);
            let py = rng.gen_range(2..MAP_HEIGHT - 2);
            self.map[py][px] = '#';
        }

        // Place stairs Down
        self.map[MAP_HEIGHT - 2][MAP_WIDTH - 2] = '>';

        // Spawn Items
        let item_types = vec![
            ('$', "Gold", Color::Yellow),
            ('!', "Healing Potion", Color::Red),
            ('?', "Scroll of Power", Color::Cyan),
            ('/', "Sword", Color::LightBlue),
        ];

        for _ in 0..3 {
            let ix = rng.gen_range(2..MAP_WIDTH - 2) as i32;
            let iy = rng.gen_range(2..MAP_HEIGHT - 2) as i32;
            if self.map[iy as usize][ix as usize] == '.' {
                let choice = &item_types[rng.gen_range(0..item_types.len())];
                self.items.push(Item {
                    pos: (ix, iy),
                    symbol: choice.0,
                    name: choice.1,
                    color: choice.2,
                });
            }
        }

        // Spawn Enemies
        let enemy_types = vec![
            ('g', "Goblin", 10, Color::Green),
            ('o', "Orc", 15, Color::Red),
            ('s', "Snake", 8, Color::Yellow),
            ('r', "Rat", 5, Color::DarkGray),
        ];

        for _ in 0..4 {
            let ex = rng.gen_range(3..MAP_WIDTH - 2) as i32;
            let ey = rng.gen_range(3..MAP_HEIGHT - 2) as i32;
            if self.map[ey as usize][ex as usize] == '.' {
                let choice = &enemy_types[rng.gen_range(0..enemy_types.len())];
                self.enemies.push(Enemy {
                    pos: (ex, ey),
                    hp: choice.2,
                    symbol: choice.0,
                    name: choice.1,
                    color: choice.3,
                });
            }
        }
    }

    fn move_player(&mut self, dx: i32, dy: i32) {
        let nx = self.player_pos.0 + dx;
        let ny = self.player_pos.1 + dy;

        if nx < 0 || nx >= MAP_WIDTH as i32 || ny < 0 || ny >= MAP_HEIGHT as i32 {
            return;
        }

        // Wall collision
        if self.map[ny as usize][nx as usize] == '#' {
            return;
        }

        // Enemy check
        let mut hit_enemy_idx = None;
        for (i, e) in self.enemies.iter().enumerate() {
            if e.pos == (nx, ny) {
                hit_enemy_idx = Some(i);
                break;
            }
        }

        if let Some(idx) = hit_enemy_idx {
            let e_name = self.enemies[idx].name;
            let mut rng = rand::thread_rng();
            let dmg = (self.player_atk + rng.gen_range(-2..=2)).max(1);
            self.enemies[idx].hp -= dmg;
            self.log.push(format!("You hit {} for {} dmg!", e_name, dmg));
            
            if self.enemies[idx].hp <= 0 {
                self.log.push(format!("{} dies!", e_name));
                self.enemies.remove(idx);
                self.score += 150;
            } else {
                // Enemy retaliates
                let e_dmg = (rng.gen_range(1..=4) - self.player_def).max(1);
                self.player_hp -= e_dmg;
                self.log.push(format!("{} hits you back for {}!", e_name, e_dmg));
                if self.player_hp <= 0 {
                    self.game_over = true;
                    self.log.push("You have died!".to_string());
                }
            }
            return;
        }

        // Move to floor
        self.player_pos = (nx, ny);

        // Item pick-up check
        let mut got_item_idx = None;
        for (i, item) in self.items.iter().enumerate() {
            if item.pos == self.player_pos {
                got_item_idx = Some(i);
                break;
            }
        }

        if let Some(idx) = got_item_idx {
            let name = self.items[idx].name;
            self.log.push(format!("Picked up: {}!", name));
            match self.items[idx].symbol {
                '$' => {
                    self.score += 100;
                }
                '!' => {
                    self.player_hp = (self.player_hp + 15).min(self.player_max_hp);
                    self.score += 20;
                }
                '?' => {
                    self.player_atk += 2;
                    self.score += 50;
                }
                '/' => {
                    self.player_atk += 4;
                    self.score += 100;
                }
                _ => {}
            }
            self.items.remove(idx);
        }

        // Stairs Check
        if self.map[self.player_pos.1 as usize][self.player_pos.0 as usize] == '>' {
            self.level += 1;
            self.score += 500;
            self.log.push(format!("Advanced to level {}!", self.level));
            self.generate_level();
            self.player_pos = (2, 2);
        }

        // Enemy AI movement
        let mut rng = rand::thread_rng();
        for i in 0..self.enemies.len() {
            if rng.gen_bool(0.4) {
                let mut edx = (self.player_pos.0 - self.enemies[i].pos.0).signum();
                let mut edy = (self.player_pos.1 - self.enemies[i].pos.1).signum();
                if rng.gen_bool(0.5) {
                    edx = 0;
                } else {
                    edy = 0;
                }
                let nex = self.enemies[i].pos.0 + edx;
                let ney = self.enemies[i].pos.1 + edy;
                if self.map[ney as usize][nex as usize] != '#' && (nex, ney) != self.player_pos {
                    self.enemies[i].pos = (nex, ney);
                }
            }
        }
    }
}

impl Default for HackGame {
    fn default() -> Self {
        Self::new()
    }
}

impl Game for HackGame {
    fn update(&mut self, _delta: Duration) {
        // Turn-based, inputs drive updates
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
            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => self.move_player(0, -1),
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => self.move_player(0, 1),
            KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => self.move_player(-1, 0),
            KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => self.move_player(1, 0),
            KeyCode::Char('r') | KeyCode::Char('R') => {
                *self = Self::new();
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
            .border_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD));

        frame.render_widget(outer_block, area);

        let inner_area = area.inner(&ratatui::layout::Margin { horizontal: 2, vertical: 1 });
        
        let layouts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length((MAP_WIDTH * 2 + 2) as u16), // Board Area
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

        // Build visual buffer
        let mut visual_grid = [[(' ', Style::default()); MAP_WIDTH]; MAP_HEIGHT];

        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let sym = self.map[y][x];
                let color = if sym == '#' { Color::Rgb(80, 80, 80) } else { Color::Rgb(40, 40, 40) };
                visual_grid[y][x] = (sym, Style::default().fg(color));
            }
        }

        // Render Items
        for item in &self.items {
            visual_grid[item.pos.1 as usize][item.pos.0 as usize] = (item.symbol, Style::default().fg(item.color).add_modifier(Modifier::BOLD));
        }

        // Render Enemies
        for enemy in &self.enemies {
            visual_grid[enemy.pos.1 as usize][enemy.pos.0 as usize] = (enemy.symbol, Style::default().fg(enemy.color).add_modifier(Modifier::BOLD));
        }

        // Render Player
        visual_grid[self.player_pos.1 as usize][self.player_pos.0 as usize] = ('@', Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

        let mut rows = Vec::new();
        for y in 0..MAP_HEIGHT {
            let mut line_spans = Vec::new();
            for x in 0..MAP_WIDTH {
                let (sym, style) = visual_grid[y][x];
                if sym == '#' {
                    line_spans.push(Span::styled("██", style));
                } else if sym == '.' {
                    line_spans.push(Span::styled(". ", style));
                } else {
                    line_spans.push(Span::styled(format!("{} ", sym), style));
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
                Constraint::Length(9), // Stats Panel
                Constraint::Min(6),    // Action log panel
            ])
            .split(side_area);

        let stats_content = vec![
            Line::from(vec![
                Span::styled(" HP:     ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::styled(format!("{}/{}", self.player_hp, self.player_max_hp), Style::default().fg(Color::White)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" ATK/DEF:", Style::default().fg(Color::Cyan)),
                Span::styled(format!("{}/{}", self.player_atk, self.player_def), Style::default().fg(Color::White)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" DEPTH:  ", Style::default().fg(Color::Magenta)),
                Span::styled(format!("Level {}", self.level), Style::default().fg(Color::White)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" SCORE:  ", Style::default().fg(Color::Yellow)),
                Span::styled(format!("{}", self.score), Style::default().fg(Color::White)),
            ]),
        ];

        let stats_paragraph = Paragraph::new(stats_content)
            .block(Block::default().borders(Borders::ALL).title("HERO STATUS"));
        frame.render_widget(stats_paragraph, side_layout[0]);

        // Draw action logs
        let mut log_lines = Vec::new();
        let start_idx = self.log.len().saturating_sub(4);
        for entry in &self.log[start_idx..] {
            log_lines.push(Line::from(Span::styled(entry, Style::default().fg(Color::Gray))));
        }

        let log_paragraph = Paragraph::new(log_lines)
            .block(Block::default().borders(Borders::ALL).title("LOG"));
        frame.render_widget(log_paragraph, side_layout[1]);

        if self.paused {
            let pause_area = Rect {
                x: board_inner.x + (MAP_WIDTH * 2 - 18) as u16 / 2,
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
                x: board_inner.x + (MAP_WIDTH * 2 - 20) as u16 / 2,
                y: board_inner.y + 3,
                width: 20,
                height: 7,
            };
            frame.render_widget(Clear, go_area);
            
            let message = vec![
                Line::from(""),
                Line::from(Span::styled(" HERO FAILED! ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))),
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
