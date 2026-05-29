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

const MAP_WIDTH: usize = 24;
const MAP_HEIGHT: usize = 14;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TileType {
    Wall,
    Floor,
    Stairs,
}

#[derive(Clone, Debug, PartialEq)]
enum EnemyType {
    Goblin,
    Orc,
    Troll,
}

impl EnemyType {
    pub fn name(&self) -> &'static str {
        match self {
            EnemyType::Goblin => "Goblin",
            EnemyType::Orc => "Orc",
            EnemyType::Troll => "Troll",
        }
    }

    pub fn char_symbol(&self) -> &'static str {
        match self {
            EnemyType::Goblin => "g",
            EnemyType::Orc => "o",
            EnemyType::Troll => "T",
        }
    }


    pub fn stats(&self, depth: u32) -> (i32, i32, u32) {
        // Returns (HP, Attack, XP Reward) scaled with dungeon depth
        let scale = depth as i32;
        match self {
            EnemyType::Goblin => (15 + scale * 3, 4 + scale, 15 * depth),
            EnemyType::Orc => (30 + scale * 5, 8 + scale, 30 * depth),
            EnemyType::Troll => (60 + scale * 10, 14 + scale * 2, 60 * depth),
        }
    }
}

struct Enemy {
    x: usize,
    y: usize,
    enemy_type: EnemyType,
    hp: i32,
    attack: i32,
    xp_reward: u32,
}

#[derive(Clone, Debug, PartialEq)]
enum ItemType {
    Potion, // Heals HP
    Sword,  // Adds Attack
    Shield, // Adds Max HP + HP
}

struct Item {
    x: usize,
    y: usize,
    item_type: ItemType,
}

struct Room {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

pub struct RoguelikeGame {
    map: [[TileType; MAP_WIDTH]; MAP_HEIGHT],
    player_x: usize,
    player_y: usize,
    
    // Player Stats
    hp: i32,
    max_hp: i32,
    attack: i32,
    level: u32,
    xp: u32,
    depth: u32,
    score: u32,
    
    enemies: Vec<Enemy>,
    items: Vec<Item>,
    logs: Vec<String>,
    
    game_over: bool,
    paused: bool,
}

impl Default for RoguelikeGame {
    fn default() -> Self {
        Self::new()
    }
}

impl RoguelikeGame {
    pub fn new() -> Self {
        let mut game = Self {
            map: [[TileType::Wall; MAP_WIDTH]; MAP_HEIGHT],
            player_x: 0,
            player_y: 0,
            hp: 80,
            max_hp: 80,
            attack: 8,
            level: 1,
            xp: 0,
            depth: 1,
            score: 0,
            enemies: Vec::new(),
            items: Vec::new(),
            logs: Vec::new(),
            game_over: false,
            paused: false,
        };
        game.log("Welcome to the Roguelike Dungeon!".to_string());
        game.generate_level();
        game
    }

    fn log(&mut self, msg: String) {
        self.logs.push(msg);
        if self.logs.len() > 10 {
            self.logs.remove(0);
        }
    }

    fn generate_level(&mut self) {
        self.map = [[TileType::Wall; MAP_WIDTH]; MAP_HEIGHT];
        self.enemies.clear();
        self.items.clear();

        let mut rng = rand::thread_rng();
        let mut rooms: Vec<Room> = Vec::new();

        // Try to generate 3 non-overlapping rooms
        for _ in 0..6 {
            if rooms.len() >= 3 {
                break;
            }
            let w = rng.gen_range(4..8);
            let h = rng.gen_range(3..6);
            let x = rng.gen_range(1..MAP_WIDTH - w - 1);
            let y = rng.gen_range(1..MAP_HEIGHT - h - 1);

            // Simple overlap check
            let mut overlap = false;
            for r in &rooms {
                let overlap_x = x < r.x + r.w && x + w > r.x;
                let overlap_y = y < r.y + r.h && y + h > r.y;
                if overlap_x && overlap_y {
                    overlap = true;
                    break;
                }
            }

            if !overlap {
                rooms.push(Room { x, y, w, h });
            }
        }

        // Dig out floors
        for r in &rooms {
            for y_coord in r.y..r.y + r.h {
                for x_coord in r.x..r.x + r.w {
                    self.map[y_coord][x_coord] = TileType::Floor;
                }
            }
        }

        // Draw corridors connecting room centers
        for i in 0..rooms.len() - 1 {
            let cx1 = rooms[i].x + rooms[i].w / 2;
            let cy1 = rooms[i].y + rooms[i].h / 2;
            let cx2 = rooms[i + 1].x + rooms[i + 1].w / 2;
            let cy2 = rooms[i + 1].y + rooms[i + 1].h / 2;

            // Connect horizontally then vertically
            let x_start = cx1.min(cx2);
            let x_end = cx1.max(cx2);
            for x in x_start..=x_end {
                self.map[cy1][x] = TileType::Floor;
            }

            let y_start = cy1.min(cy2);
            let y_end = cy1.max(cy2);
            for y in y_start..=y_end {
                self.map[y][cx2] = TileType::Floor;
            }
        }

        // Spawn Player in Room 0 center
        self.player_x = rooms[0].x + rooms[0].w / 2;
        self.player_y = rooms[0].y + rooms[0].h / 2;

        // Spawn Stairs in Room 2 (or last room) center
        let last_idx = rooms.len() - 1;
        let sx = rooms[last_idx].x + rooms[last_idx].w / 2;
        let sy = rooms[last_idx].y + rooms[last_idx].h / 2;
        self.map[sy][sx] = TileType::Stairs;

        // Populate other rooms with items and enemies
        for (idx, r) in rooms.iter().enumerate() {
            if idx == 0 {
                continue; // Safe start room
            }

            // Spawn 1-2 enemies
            let enemy_count = rng.gen_range(1..=2);
            for _ in 0..enemy_count {
                let ex = rng.gen_range(r.x..r.x + r.w);
                let ey = rng.gen_range(r.y..r.y + r.h);
                if (ex != sx || ey != sy) && (ex != self.player_x || ey != self.player_y) {
                    let enemy_roll = rng.gen_range(0..100);
                    let enemy_type = if enemy_roll < 55 {
                        EnemyType::Goblin
                    } else if enemy_roll < 85 {
                        EnemyType::Orc
                    } else {
                        EnemyType::Troll
                    };

                    let (ehp, eatt, exp) = enemy_type.stats(self.depth);
                    self.enemies.push(Enemy {
                        x: ex,
                        y: ey,
                        enemy_type,
                        hp: ehp,
                        attack: eatt,
                        xp_reward: exp,
                    });
                }
            }

            // 60% chance of spawning 1 item
            if rng.gen_bool(0.6) {
                let ix = rng.gen_range(r.x..r.x + r.w);
                let iy = rng.gen_range(r.y..r.y + r.h);
                if (ix != sx || iy != sy) && (ix != self.player_x || iy != self.player_y) {
                    let item_roll = rng.gen_range(0..3);
                    let item_type = match item_roll {
                        0 => ItemType::Potion,
                        1 => ItemType::Sword,
                        _ => ItemType::Shield,
                    };
                    self.items.push(Item { x: ix, y: iy, item_type });
                }
            }
        }
        
        self.log(format!("Entered Dungeon Depth {}", self.depth));
    }

    fn try_move(&mut self, dx: i32, dy: i32) {
        let new_x = (self.player_x as i32 + dx) as usize;
        let new_y = (self.player_y as i32 + dy) as usize;

        if new_x >= MAP_WIDTH || new_y >= MAP_HEIGHT {
            return;
        }

        // A. Attack Enemy if present
        let mut enemy_idx = None;
        for (idx, enemy) in self.enemies.iter().enumerate() {
            if enemy.x == new_x && enemy.y == new_y {
                enemy_idx = Some(idx);
                break;
            }
        }

        if let Some(idx) = enemy_idx {
            // Hit calculations
            let mut rng = rand::thread_rng();
            let variance = rng.gen_range(-2..=2);
            let dmg = (self.attack + variance).max(1);
            
            self.enemies[idx].hp -= dmg;
            let name = self.enemies[idx].enemy_type.name();
            self.log(format!("You hit {} for {} dmg!", name, dmg));

            if self.enemies[idx].hp <= 0 {
                let reward = self.enemies[idx].xp_reward;
                self.log(format!("Defeated {}! (+{} XP)", name, reward));
                self.xp += reward;
                self.score += reward / 2;
                self.enemies.remove(idx);
                self.check_level_up();
            }

            // Action triggers enemy turn
            self.enemy_turn();
            return;
        }

        // B. Check Walls
        if self.map[new_y][new_x] == TileType::Wall {
            return; // Can't walk through walls
        }

        // C. Make the move
        self.player_x = new_x;
        self.player_y = new_y;

        // D. Grab Items if on them
        let mut grabbed_idx = None;
        for (idx, item) in self.items.iter().enumerate() {
            if item.x == self.player_x && item.y == self.player_y {
                grabbed_idx = Some(idx);
                break;
            }
        }

        if let Some(idx) = grabbed_idx {
            let item = &self.items[idx];
            match item.item_type {
                ItemType::Potion => {
                    let heal = 25 + (self.level as i32 * 5);
                    self.hp = (self.hp + heal).min(self.max_hp);
                    self.log(format!("Drank potion: healed +{} HP!", heal));
                }
                ItemType::Sword => {
                    self.attack += 2;
                    self.log("Found Steel Sword: attack +2!".to_string());
                }
                ItemType::Shield => {
                    self.max_hp += 10;
                    self.hp += 10;
                    self.log("Found Iron Shield: max HP +10!".to_string());
                }
            }
            self.score += 50;
            self.items.remove(idx);
        }

        // E. Check stairs
        if self.map[self.player_y][self.player_x] == TileType::Stairs {
            self.depth += 1;
            self.score += 150;
            self.generate_level();
            return;
        }

        // Moving triggers enemy turn
        self.enemy_turn();
    }

    fn check_level_up(&mut self) {
        let req_xp = self.level * 100;
        if self.xp >= req_xp {
            self.xp -= req_xp;
            self.level += 1;
            self.max_hp += 15;
            self.hp = self.max_hp; // Heals completely on level up
            self.attack += 2;
            self.log(format!("LEVEL UP! Reached Lvl {}. Max HP & Atk increased!", self.level));
        }
    }

    fn enemy_turn(&mut self) {
        let mut rng = rand::thread_rng();
        
        for idx in 0..self.enemies.len() {
            let (ex, ey, eattack, ename) = {
                let enemy = &self.enemies[idx];
                (enemy.x, enemy.y, enemy.attack, enemy.enemy_type.name().to_string())
            };
            
            // Calculate Manhattan distance to player
            let dist = (ex as i32 - self.player_x as i32).abs() + (ey as i32 - self.player_y as i32).abs();
            
            if dist <= 1 {
                // Adjacent! Attack Player
                let variance = rng.gen_range(-1..=1);
                let dmg = (eattack + variance).max(1);
                self.hp -= dmg;
                self.log(format!("{} bites you for {} dmg!", ename, dmg));

                if self.hp <= 0 {
                    self.hp = 0;
                    self.game_over = true;
                    self.log("You died... Game Over!".to_string());
                }
            } else if dist <= 5 {
                // Inside aggro radius: Step closer to player
                let mut step_x = 0;
                let mut step_y = 0;

                if ex < self.player_x {
                    step_x = 1;
                } else if ex > self.player_x {
                    step_x = -1;
                }

                if ey < self.player_y {
                    step_y = 1;
                } else if ey > self.player_y {
                    step_y = -1;
                }

                // Prefer horizontal step or vertical step
                let next_x = (ex as i32 + step_x) as usize;
                let next_y = (ey as i32 + step_y) as usize;

                let mut moved_x = false;
                let mut moved_y = false;

                // Move if it doesn't collide with walls or other enemies
                if self.map[ey][next_x] == TileType::Floor && !self.is_occupied_by_enemy(next_x, ey) {
                    moved_x = true;
                } else if self.map[next_y][ex] == TileType::Floor && !self.is_occupied_by_enemy(ex, next_y) {
                    moved_y = true;
                }

                let enemy = &mut self.enemies[idx];
                if moved_x {
                    enemy.x = next_x;
                } else if moved_y {
                    enemy.y = next_y;
                }
            }
        }
    }

    fn is_occupied_by_enemy(&self, x: usize, y: usize) -> bool {
        self.enemies.iter().any(|e| e.x == x && e.y == y)
    }
}

impl Game for RoguelikeGame {
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
            KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => self.try_move(-1, 0),
            KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => self.try_move(1, 0),
            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => self.try_move(0, -1),
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => self.try_move(0, 1),
            // Wait-turn key
            KeyCode::Char(' ') | KeyCode::Char('.') => self.enemy_turn(),
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => return GameCommand::Exit,
            _ => {}
        }

        GameCommand::None
    }

    fn draw(&self, frame: &mut Frame, area: Rect, palette: &ThemePalette) {
        let outer_block = Block::default()
            .title(" ROGUELIKE CRAWLER ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(palette.accent_alt).add_modifier(Modifier::BOLD));

        frame.render_widget(outer_block, area);

        let inner_area = area.inner(&ratatui::layout::Margin { horizontal: 2, vertical: 1 });
        
        let layouts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length((MAP_WIDTH * 2 + 2) as u16), // Map board Area
                Constraint::Min(12),                          // Sidebar info & logs
            ])
            .split(inner_area);

        let board_area = layouts[0];
        let side_area = layouts[1];

        // Draw Map Border
        let board_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(palette.muted));
        
        let board_inner = board_block.inner(board_area);
        frame.render_widget(board_block, board_area);

        // Build dual-character grid buffer
        let mut visual_grid = [[None; MAP_WIDTH]; MAP_HEIGHT];

        // 1. Dig Map Tiles
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let style = match self.map[y][x] {
                    TileType::Wall => Style::default().fg(palette.muted),
                    TileType::Floor => Style::default().fg(palette.muted),
                    TileType::Stairs => Style::default().fg(palette.accent).add_modifier(Modifier::BOLD),
                };
                visual_grid[y][x] = Some(style);
            }
        }

        // 2. Draw Items
        for item in &self.items {
            let color = match item.item_type {
                ItemType::Potion => palette.accent,
                ItemType::Sword => palette.accent,
                ItemType::Shield => palette.accent_alt,
            };
            visual_grid[item.y][item.x] = Some(Style::default().fg(color).add_modifier(Modifier::BOLD));
        }

        // 3. Draw Enemies
        for enemy in &self.enemies {
            let enemy_color = match enemy.enemy_type {
                EnemyType::Goblin => palette.danger,
                EnemyType::Orc => palette.danger,
                EnemyType::Troll => palette.muted,
            };
            visual_grid[enemy.y][enemy.x] = Some(Style::default().fg(enemy_color).add_modifier(Modifier::BOLD));
        }

        // 4. Draw Player
        if !self.game_over {
            visual_grid[self.player_y][self.player_x] = Some(Style::default().fg(palette.accent_alt).add_modifier(Modifier::BOLD));
        }

        // Render buffer to paragraph
        let mut rows = Vec::new();
        for y in 0..MAP_HEIGHT {
            let mut line_spans = Vec::new();
            for x in 0..MAP_WIDTH {
                if let Some(style) = visual_grid[y][x] {
                    if !self.game_over && y == self.player_y && x == self.player_x {
                        line_spans.push(Span::styled("@ ", style));
                    } else if let Some(enemy) = self.enemies.iter().find(|e| e.x == x && e.y == y) {
                        line_spans.push(Span::styled(enemy.enemy_type.char_symbol(), style));
                    } else if let Some(item) = self.items.iter().find(|i| i.x == x && i.y == y) {
                        let symbol = match item.item_type {
                            ItemType::Potion => "p",
                            ItemType::Sword => "s",
                            ItemType::Shield => "d",
                        };
                        line_spans.push(Span::styled(symbol, style));
                    } else {
                        let cell_char = match self.map[y][x] {
                            TileType::Wall => "##",
                            TileType::Floor => ". ",
                            TileType::Stairs => "> ",
                        };
                        line_spans.push(Span::styled(cell_char, style));
                    }
                } else {
                    line_spans.push(Span::raw("  "));
                }
            }
            rows.push(Line::from(line_spans));
        }

        let board_paragraph = Paragraph::new(rows);
        frame.render_widget(board_paragraph, board_inner);

        // Sidebar stats & action logs layout
        let side_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8), // Stats Panel
                Constraint::Min(6),    // Action log scrolling
            ])
            .split(side_area);

        // HP progress bar calculations
        let hp_ratio = (self.hp as f32 / self.max_hp as f32).max(0.0);
        let hp_bar_len = 10;
        let filled_hp_len = (hp_ratio * hp_bar_len as f32) as usize;
        let mut hp_bar = "[".to_string();
        for i in 0..hp_bar_len {
            if i < filled_hp_len {
                hp_bar.push('=');
            } else {
                hp_bar.push(' ');
            }
        }
        hp_bar.push(']');

        let stats_content = vec![
            Line::from(vec![
                Span::styled(" STATUS ", Style::default().fg(palette.accent_alt).add_modifier(Modifier::BOLD)),
                Span::styled(format!(" Lvl {} Depth {}", self.level, self.depth), Style::default().fg(palette.body)),
            ]),
            Line::from(vec![
                Span::styled(" HP     ", Style::default().fg(palette.danger)),
                Span::styled(format!(" {}/{}", self.hp, self.max_hp), Style::default().fg(palette.body)),
            ]),
            Line::from(vec![
                Span::styled(hp_bar, Style::default().fg(if hp_ratio > 0.4 { palette.accent } else { palette.danger })),
            ]),
            Line::from(vec![
                Span::styled(" ATK    ", Style::default().fg(palette.accent)),
                Span::styled(format!(" {}", self.attack), Style::default().fg(palette.body)),
            ]),
            Line::from(vec![
                Span::styled(" XP     ", Style::default().fg(palette.accent_alt)),
                Span::styled(format!(" {}/{}", self.xp, self.level * 100), Style::default().fg(palette.body)),
            ]),
            Line::from(vec![
                Span::styled(" SCORE  ", Style::default().fg(palette.accent_alt)),
                Span::styled(format!(" {:05}", self.score), Style::default().fg(palette.body)),
            ]),
        ];

        let stats_paragraph = Paragraph::new(stats_content)
            .block(Block::default().borders(Borders::ALL).title("STATS"));
        frame.render_widget(stats_paragraph, side_layout[0]);

        // Action logs Paragraph
        let mut log_spans = Vec::new();
        // Display last 5 logs
        for log in self.logs.iter().rev().take(5).rev() {
            let color = if log.contains("hit") {
                palette.danger
            } else if log.contains("Level") || log.contains("Welcome") {
                palette.accent
            } else if log.contains("Defeated") || log.contains("healed") || log.contains("Steel") || log.contains("Iron") {
                palette.accent
            } else {
                palette.muted
            };
            log_spans.push(Line::from(Span::styled(format!("> {}", log), Style::default().fg(color))));
        }

        let logs_paragraph = Paragraph::new(log_spans)
            .block(Block::default().borders(Borders::ALL).title("DUNGEON LOG"));
        frame.render_widget(logs_paragraph, side_layout[1]);

        // Overlays
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
                Line::from(Span::styled(" PAUSED ", Style::default().fg(palette.accent_alt).add_modifier(Modifier::BOLD))),
                Line::from(Span::styled("Press [Tab] to resume", Style::default().fg(palette.muted))),
            ])
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(palette.accent_alt)));
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
                Line::from(Span::styled(" HEAVILY WOUNDED ", Style::default().fg(palette.danger).add_modifier(Modifier::BOLD))),
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
