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

const GRID_SIZE: usize = 10;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum CellState {
    Water,
    ShipSegment,
    Miss,
    Hit,
    Sunk,
}

#[derive(Clone, Debug)]
struct Ship {
    name: &'static str,
    length: usize,
    coordinates: Vec<(usize, usize)>,
    hits: usize,
}

impl Ship {
    fn new(name: &'static str, length: usize) -> Self {
        Self {
            name,
            length,
            coordinates: Vec::new(),
            hits: 0,
        }
    }
}

pub struct BattleshipGame {
    player_grid: [[CellState; GRID_SIZE]; GRID_SIZE],
    ai_grid: [[CellState; GRID_SIZE]; GRID_SIZE],
    player_ships: Vec<Ship>,
    ai_ships: Vec<Ship>,
    placement_idx: usize,
    placement_vertical: bool,
    cursor_x: usize,
    cursor_y: usize,
    is_placement_phase: bool,
    game_over: bool,
    won: bool,
    paused: bool,
    score: u32,
    log_messages: Vec<String>,
}

impl Default for BattleshipGame {
    fn default() -> Self {
        Self::new()
    }
}

impl BattleshipGame {
    pub fn new() -> Self {
        let mut game = Self {
            player_grid: [[CellState::Water; GRID_SIZE]; GRID_SIZE],
            ai_grid: [[CellState::Water; GRID_SIZE]; GRID_SIZE],
            player_ships: vec![
                Ship::new("Carrier", 5),
                Ship::new("Battleship", 4),
                Ship::new("Destroyer", 3),
                Ship::new("Submarine", 3),
                Ship::new("Patrol Boat", 2),
            ],
            ai_ships: vec![
                Ship::new("Carrier", 5),
                Ship::new("Battleship", 4),
                Ship::new("Destroyer", 3),
                Ship::new("Submarine", 3),
                Ship::new("Patrol Boat", 2),
            ],
            placement_idx: 0,
            placement_vertical: false,
            cursor_x: 0,
            cursor_y: 0,
            is_placement_phase: true,
            game_over: false,
            won: false,
            paused: false,
            score: 0,
            log_messages: vec!["Place your Carrier (length: 5)".to_string()],
        };

        game.place_ai_ships();
        game
    }

    fn add_log(&mut self, msg: String) {
        self.log_messages.push(msg);
        if self.log_messages.len() > 6 {
            self.log_messages.remove(0);
        }
    }

    fn place_ai_ships(&mut self) {
        let mut rng = rand::thread_rng();

        for s_idx in 0..self.ai_ships.len() {
            let ship_len = self.ai_ships[s_idx].length;
            loop {
                let vertical = rng.gen_bool(0.5);
                let rx = rng.gen_range(0..GRID_SIZE);
                let ry = rng.gen_range(0..GRID_SIZE);

                // Boundary check
                if (vertical && ry + ship_len > GRID_SIZE) || (!vertical && rx + ship_len > GRID_SIZE) {
                    continue;
                }

                // Overlap check
                let mut overlap = false;
                let mut coords = Vec::new();
                for i in 0..ship_len {
                    let cx = if vertical { rx } else { rx + i };
                    let cy = if vertical { ry + i } else { ry };
                    if self.ai_grid[cy][cx] != CellState::Water {
                        overlap = true;
                        break;
                    }
                    coords.push((cx, cy));
                }

                if !overlap {
                    for &(cx, cy) in &coords {
                        self.ai_grid[cy][cx] = CellState::ShipSegment;
                    }
                    self.ai_ships[s_idx].coordinates = coords;
                    break;
                }
            }
        }
    }

    fn try_place_player_ship(&mut self) {
        let ship_len = self.player_ships[self.placement_idx].length;
        let vertical = self.placement_vertical;
        let rx = self.cursor_x;
        let ry = self.cursor_y;

        // Boundary check
        if (vertical && ry + ship_len > GRID_SIZE) || (!vertical && rx + ship_len > GRID_SIZE) {
            self.add_log("Out of grid bounds!".to_string());
            return;
        }

        // Overlap check
        let mut overlap = false;
        let mut coords = Vec::new();
        for i in 0..ship_len {
            let cx = if vertical { rx } else { rx + i };
            let cy = if vertical { ry + i } else { ry };
            if self.player_grid[cy][cx] == CellState::ShipSegment {
                overlap = true;
                break;
            }
            coords.push((cx, cy));
        }

        if overlap {
            self.add_log("Ships cannot overlap!".to_string());
            return;
        }

        // Place ship segments
        for &(cx, cy) in &coords {
            self.player_grid[cy][cx] = CellState::ShipSegment;
        }
        self.player_ships[self.placement_idx].coordinates = coords;

        self.placement_idx += 1;
        self.cursor_x = 0;
        self.cursor_y = 0;

        if self.placement_idx >= self.player_ships.len() {
            self.is_placement_phase = false;
            self.add_log("Battle phase started! Select targets.".to_string());
        } else {
            let next_ship = self.player_ships[self.placement_idx].name;
            let next_len = self.player_ships[self.placement_idx].length;
            self.add_log(format!("Place your {} (length: {})", next_ship, next_len));
        }
    }

    fn player_fire(&mut self) {
        let tx = self.cursor_x;
        let ty = self.cursor_y;

        let target_cell = self.ai_grid[ty][tx];
        if target_cell == CellState::Hit || target_cell == CellState::Miss || target_cell == CellState::Sunk {
            self.add_log("Already targeted this square!".to_string());
            return;
        }

        if target_cell == CellState::ShipSegment {
            self.ai_grid[ty][tx] = CellState::Hit;
            self.add_log(format!("HIT at coordinate ({},{})!", tx + 1, ty + 1));
            self.score += 100;

            // Increment ship hit counter
            let mut ship_idx_sunk = None;
            for s_idx in 0..self.ai_ships.len() {
                if self.ai_ships[s_idx].coordinates.contains(&(tx, ty)) {
                    self.ai_ships[s_idx].hits += 1;
                    if self.ai_ships[s_idx].hits == self.ai_ships[s_idx].length {
                        ship_idx_sunk = Some(s_idx);
                    }
                    break;
                }
            }

            if let Some(sunk_idx) = ship_idx_sunk {
                let name = self.ai_ships[sunk_idx].name;
                self.add_log(format!("You SUNK the AI's {}!", name));
                self.score += 500;
                for &(cx, cy) in &self.ai_ships[sunk_idx].coordinates {
                    self.ai_grid[cy][cx] = CellState::Sunk;
                }
            }

            self.check_game_over();
            if self.game_over {
                return;
            }
        } else {
            self.ai_grid[ty][tx] = CellState::Miss;
            self.add_log(format!("Miss at coordinate ({},{})!", tx + 1, ty + 1));
        }

        // Trigger AI Turn
        self.ai_fire();
    }

    fn ai_fire(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let tx = rng.gen_range(0..GRID_SIZE);
            let ty = rng.gen_range(0..GRID_SIZE);

            let target_cell = self.player_grid[ty][tx];
            if target_cell == CellState::Hit || target_cell == CellState::Miss || target_cell == CellState::Sunk {
                continue; // retry
            }

            if target_cell == CellState::ShipSegment {
                self.player_grid[ty][tx] = CellState::Hit;
                self.add_log(format!("AI HITS your ship at ({},{})!", tx + 1, ty + 1));

                let mut ship_idx_sunk = None;
                for s_idx in 0..self.player_ships.len() {
                    if self.player_ships[s_idx].coordinates.contains(&(tx, ty)) {
                        self.player_ships[s_idx].hits += 1;
                        if self.player_ships[s_idx].hits == self.player_ships[s_idx].length {
                            ship_idx_sunk = Some(s_idx);
                        }
                        break;
                    }
                }

                if let Some(sunk_idx) = ship_idx_sunk {
                    let name = self.player_ships[sunk_idx].name;
                    self.add_log(format!("AI SUNK your {}!", name));
                    for &(cx, cy) in &self.player_ships[sunk_idx].coordinates {
                        self.player_grid[cy][cx] = CellState::Sunk;
                    }
                }

                self.check_game_over();
            } else {
                self.player_grid[ty][tx] = CellState::Miss;
                self.add_log(format!("AI fires and misses at ({},{})!", tx + 1, ty + 1));
            }
            break;
        }
    }

    fn check_game_over(&mut self) {
        let ai_sunk = self.ai_ships.iter().all(|s| s.hits == s.length);
        let player_sunk = self.player_ships.iter().all(|s| s.hits == s.length);

        if ai_sunk {
            self.game_over = true;
            self.won = true;
            self.score += 2000;
        } else if player_sunk {
            self.game_over = true;
            self.won = false;
        }
    }
}

impl Game for BattleshipGame {
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
            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                if self.cursor_y < GRID_SIZE - 1 {
                    self.cursor_y += 1;
                }
            }
            KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
                if self.cursor_x < GRID_SIZE - 1 {
                    self.cursor_x += 1;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                if self.is_placement_phase {
                    self.placement_vertical = !self.placement_vertical;
                }
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                if self.is_placement_phase {
                    self.try_place_player_ship();
                } else {
                    self.player_fire();
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
            .title(" BATTLESHIP CABINET ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

        frame.render_widget(outer_block, area);

        let inner_area = area.inner(&ratatui::layout::Margin { horizontal: 2, vertical: 1 });

        let layouts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40), // Player Grid
                Constraint::Percentage(40), // AI Grid
                Constraint::Percentage(20), // Stats & Logs
            ])
            .split(inner_area);

        let player_area = layouts[0];
        let ai_area = layouts[1];
        let side_area = layouts[2];

        // Draw Player Grid Block
        let player_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::LightBlue))
            .title(" PLAYER FLEET ");
        let player_inner = player_block.inner(player_area);
        frame.render_widget(player_block, player_area);

        // Draw AI Grid Block
        let ai_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red))
            .title(" AI TARGET GRID ");
        let ai_inner = ai_block.inner(ai_area);
        frame.render_widget(ai_block, ai_area);

        // Build Player sea rows
        let mut player_rows = Vec::new();
        for y in 0..GRID_SIZE {
            let mut spans = Vec::new();
            spans.push(Span::raw(format!("{:2} ", y + 1))); // numbering
            for x in 0..GRID_SIZE {
                let cell = self.player_grid[y][x];
                let is_hovered = self.is_placement_phase 
                    && ((self.placement_vertical && x == self.cursor_x && y >= self.cursor_y && y < self.cursor_y + self.player_ships[self.placement_idx].length)
                    || (!self.placement_vertical && y == self.cursor_y && x >= self.cursor_x && x < self.cursor_x + self.player_ships[self.placement_idx].length));

                let (symbol, color) = match cell {
                    CellState::Water => if is_hovered { ("▓▓", Color::Yellow) } else { ("~~", Color::Blue) },
                    CellState::ShipSegment => if is_hovered { ("▓▓", Color::Yellow) } else { ("██", Color::Gray) },
                    CellState::Miss => ("o ", Color::DarkGray),
                    CellState::Hit => ("x ", Color::Red),
                    CellState::Sunk => ("XX", Color::Rgb(128, 0, 0)),
                };
                spans.push(Span::styled(symbol, Style::default().fg(color)));
            }
            player_rows.push(Line::from(spans));
        }
        let player_paragraph = Paragraph::new(player_rows);
        frame.render_widget(player_paragraph, player_inner);

        // Build AI target grid rows
        let mut ai_rows = Vec::new();
        for y in 0..GRID_SIZE {
            let mut spans = Vec::new();
            spans.push(Span::raw(format!("{:2} ", y + 1)));
            for x in 0..GRID_SIZE {
                let cell = self.ai_grid[y][x];
                let is_cursor = !self.is_placement_phase && x == self.cursor_x && y == self.cursor_y;

                let (symbol, color) = match cell {
                    CellState::ShipSegment | CellState::Water => {
                        if is_cursor { ("▓▓", Color::Yellow) } else { ("~~", Color::Blue) }
                    }
                    CellState::Miss => if is_cursor { ("o ", Color::Yellow) } else { ("o ", Color::DarkGray) },
                    CellState::Hit => if is_cursor { ("x ", Color::Yellow) } else { ("x ", Color::Red) },
                    CellState::Sunk => if is_cursor { ("XX", Color::Yellow) } else { ("XX", Color::Rgb(128, 0, 0)) },
                };
                spans.push(Span::styled(symbol, Style::default().fg(color)));
            }
            ai_rows.push(Line::from(spans));
        }
        let ai_paragraph = Paragraph::new(ai_rows);
        frame.render_widget(ai_paragraph, ai_inner);

        // Sidebar Panel
        let side_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(7), // Stats
                Constraint::Min(6),    // Action log
            ])
            .split(side_area);

        let stats_content = vec![
            Line::from(vec![
                Span::styled(" SCORE ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(format!("  {:05}", self.score), Style::default().fg(Color::White)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" PHASE ", Style::default().fg(Color::Magenta)),
                Span::styled(
                    if self.is_placement_phase { " PLACE" } else { " BATTLE" },
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
                ),
            ]),
        ];
        let stats_paragraph = Paragraph::new(stats_content)
            .block(Block::default().borders(Borders::ALL).title("STATS"));
        frame.render_widget(stats_paragraph, side_layout[0]);

        // Scrolling logs
        let mut log_spans = Vec::new();
        for log in self.log_messages.iter().rev() {
            let color = if log.contains("HIT") || log.contains("SUNK") {
                Color::Green
            } else if log.contains("AI HITS") {
                Color::Red
            } else if log.contains("bounds") || log.contains("overlap") {
                Color::Yellow
            } else {
                Color::Gray
            };
            log_spans.push(Line::from(Span::styled(format!("> {}", log), Style::default().fg(color))));
        }
        let logs_paragraph = Paragraph::new(log_spans)
            .block(Block::default().borders(Borders::ALL).title("TACTICAL LOGS"));
        frame.render_widget(logs_paragraph, side_layout[1]);

        // Overlays
        if self.paused {
            let pause_area = Rect {
                x: ai_inner.x,
                y: ai_inner.y + 3,
                width: 20,
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
                x: ai_inner.x,
                y: ai_inner.y + 2,
                width: 22,
                height: 7,
            };
            frame.render_widget(Clear, go_area);
            
            let message = vec![
                Line::from(""),
                Line::from(Span::styled(
                    if self.won { " VICTORY! FLEET SUNK " } else { " FLEET COMPLETELY SUNK " },
                    Style::default().fg(if self.won { Color::Green } else { Color::Red }).add_modifier(Modifier::BOLD)
                )),
                Line::from(format!("Final Score: {}", self.score)),
                Line::from(""),
                Line::from(Span::styled("Press [R] to retry", Style::default().fg(Color::Green))),
                Line::from(Span::styled("Press [Esc] to exit", Style::default().fg(Color::DarkGray))),
            ];

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
