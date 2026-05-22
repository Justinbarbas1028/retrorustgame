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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TetrominoType {
    I, O, T, S, Z, J, L
}

impl TetrominoType {
    pub fn color(&self) -> Color {
        match self {
            TetrominoType::I => Color::Cyan,
            TetrominoType::O => Color::Yellow,
            TetrominoType::T => Color::Magenta,
            TetrominoType::S => Color::Green,
            TetrominoType::Z => Color::Red,
            TetrominoType::J => Color::Blue,
            TetrominoType::L => Color::Rgb(255, 127, 0), // Orange
        }
    }

    pub fn blocks(&self) -> Vec<(i32, i32)> {
        match self {
            // Relative coordinates to shape center
            TetrominoType::I => vec![(-1, 0), (0, 0), (1, 0), (2, 0)],
            TetrominoType::O => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
            TetrominoType::T => vec![(-1, 0), (0, 0), (1, 0), (0, 1)],
            TetrominoType::S => vec![(0, 0), (1, 0), (-1, 1), (0, 1)],
            TetrominoType::Z => vec![(-1, 0), (0, 0), (0, 1), (1, 1)],
            TetrominoType::J => vec![(-1, 0), (0, 0), (1, 0), (-1, 1)],
            TetrominoType::L => vec![(-1, 0), (0, 0), (1, 0), (1, 1)],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tetromino {
    pub shape_type: TetrominoType,
    pub blocks: Vec<(i32, i32)>,
}

impl Tetromino {
    pub fn new(shape_type: TetrominoType) -> Self {
        Self {
            shape_type,
            blocks: shape_type.blocks(),
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let types = [
            TetrominoType::I,
            TetrominoType::O,
            TetrominoType::T,
            TetrominoType::S,
            TetrominoType::Z,
            TetrominoType::J,
            TetrominoType::L,
        ];
        let shape = types[rng.gen_range(0..types.len())];
        Self::new(shape)
    }

    pub fn rotate_clockwise(&self) -> Vec<(i32, i32)> {
        if self.shape_type == TetrominoType::O {
            return self.blocks.clone();
        }
        self.blocks
            .iter()
            .map(|&(x, y)| (-y, x))
            .collect()
    }
}

pub struct TetrisGame {
    board: [[Option<Color>; 10]; 20],
    current_piece: Tetromino,
    next_piece: Tetromino,
    held_piece: Option<Tetromino>,
    can_hold: bool,
    current_x: i32,
    current_y: i32,
    score: u32,
    lines_cleared: u32,
    level: u32,
    game_over: bool,
    paused: bool,
    tick_timer: Duration,
    tick_rate: Duration,
    flash_lines: Vec<usize>,
    flash_timer: Duration,
}

impl Default for TetrisGame {
    fn default() -> Self {
        Self::new()
    }
}

impl TetrisGame {
    pub fn new() -> Self {
        let mut game = Self {
            board: [[None; 10]; 20],
            current_piece: Tetromino::random(),
            next_piece: Tetromino::random(),
            held_piece: None,
            can_hold: true,
            current_x: 4,
            current_y: 0,
            score: 0,
            lines_cleared: 0,
            level: 1,
            game_over: false,
            paused: false,
            tick_timer: Duration::from_secs(0),
            tick_rate: Duration::from_millis(800),
            flash_lines: Vec::new(),
            flash_timer: Duration::from_secs(0),
        };
        
        // Spawn active piece
        game.reset_piece_position();
        game
    }

    fn reset_piece_position(&mut self) {
        self.current_x = 4;
        self.current_y = 0;
        
        // Adjust for wider pieces to avoid spawn collision immediately
        if self.current_piece.shape_type == TetrominoType::I {
            self.current_y = 0;
        }

        // If spawned block immediately collides, it's game over
        if self.check_collision(self.current_x, self.current_y, &self.current_piece.blocks) {
            self.game_over = true;
        }
    }

    fn check_collision(&self, x: i32, y: i32, blocks: &[(i32, i32)]) -> bool {
        for &(bx, by) in blocks {
            let target_x = x + bx;
            let target_y = y + by;

            if target_x < 0 || target_x >= 10 || target_y < 0 || target_y >= 20 {
                return true;
            }

            if self.board[target_y as usize][target_x as usize].is_some() {
                return true;
            }
        }
        false
    }

    fn lock_piece(&mut self) {
        for &(bx, by) in &self.current_piece.blocks {
            let target_x = self.current_x + bx;
            let target_y = self.current_y + by;
            if target_y >= 0 && target_y < 20 && target_x >= 0 && target_x < 10 {
                self.board[target_y as usize][target_x as usize] = Some(self.current_piece.shape_type.color());
            }
        }

        self.check_line_clears();
        
        // Cycle pieces
        self.current_piece = self.next_piece.clone();
        self.next_piece = Tetromino::random();
        self.can_hold = true;
        self.reset_piece_position();
    }

    fn check_line_clears(&mut self) {
        let mut completed = Vec::new();
        for y in 0..20 {
            let mut full = true;
            for x in 0..10 {
                if self.board[y][x].is_none() {
                    full = false;
                    break;
                }
            }
            if full {
                completed.push(y);
            }
        }

        if !completed.is_empty() {
            // Visual line clear flash triggers
            self.flash_lines = completed.clone();
            self.flash_timer = Duration::from_millis(150);
            
            // Score tracking
            let base_scores = [0, 100, 300, 500, 800];
            let raw_points = base_scores[completed.len().min(4)];
            self.score += raw_points * self.level;
            
            self.lines_cleared += completed.len() as u32;
            let new_level = (self.lines_cleared / 10) + 1;
            if new_level != self.level {
                self.level = new_level;
                // Scale difficulty speed
                let speed_ms = (800 - (self.level.min(9) - 1) * 80).max(100);
                self.tick_rate = Duration::from_millis(speed_ms as u64);
            }
        }
    }

    fn apply_line_clears(&mut self) {
        let mut y = 19;
        while y > 0 {
            let mut full = true;
            for x in 0..10 {
                if self.board[y][x].is_none() {
                    full = false;
                    break;
                }
            }

            if full {
                // Shift down all rows above
                for sy in (1..=y).rev() {
                    self.board[sy] = self.board[sy - 1];
                }
                self.board[0] = [None; 10];
                // Keep y at the same index to check the newly shifted-down row
            } else {
                y -= 1;
            }
        }
        self.flash_lines.clear();
    }

    fn get_ghost_y(&self) -> i32 {
        let mut ghost_y = self.current_y;
        while !self.check_collision(self.current_x, ghost_y + 1, &self.current_piece.blocks) {
            ghost_y += 1;
        }
        ghost_y
    }

    fn try_move(&mut self, dx: i32, dy: i32) -> bool {
        if !self.check_collision(self.current_x + dx, self.current_y + dy, &self.current_piece.blocks) {
            self.current_x += dx;
            self.current_y += dy;
            true
        } else {
            false
        }
    }

    fn try_rotate(&mut self) {
        let new_blocks = self.current_piece.rotate_clockwise();
        
        // Wall kick offsets to try in order: None, Shift Left, Shift Right, Shift Down/Up
        let kicks = vec![(0, 0), (-1, 0), (1, 0), (0, -1), (-2, 0), (2, 0)];
        
        for (kx, ky) in kicks {
            if !self.check_collision(self.current_x + kx, self.current_y + ky, &new_blocks) {
                self.current_piece.blocks = new_blocks;
                self.current_x += kx;
                self.current_y += ky;
                break;
            }
        }
    }

    fn hold_piece(&mut self) {
        if !self.can_hold {
            return;
        }

        let current_type = self.current_piece.shape_type;
        if let Some(held) = &self.held_piece {
            self.current_piece = Tetromino::new(held.shape_type);
            self.held_piece = Some(Tetromino::new(current_type));
        } else {
            self.held_piece = Some(Tetromino::new(current_type));
            self.current_piece = self.next_piece.clone();
            self.next_piece = Tetromino::random();
        }
        self.can_hold = false;
        self.reset_piece_position();
    }

    fn hard_drop(&mut self) {
        let ghost_y = self.get_ghost_y();
        self.current_y = ghost_y;
        self.lock_piece();
    }
}

impl Game for TetrisGame {
    fn update(&mut self, delta: Duration) {
        if self.game_over || self.paused {
            return;
        }

        // Handle lines flash timer
        if !self.flash_lines.is_empty() {
            if self.flash_timer <= delta {
                self.apply_line_clears();
                self.flash_timer = Duration::from_secs(0);
            } else {
                self.flash_timer -= delta;
                return; // Delay standard game loops during clear flashes
            }
        }

        self.tick_timer += delta;
        if self.tick_timer >= self.tick_rate {
            self.tick_timer = Duration::from_secs(0);
            if !self.try_move(0, 1) {
                self.lock_piece();
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
            KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
                self.try_move(-1, 0);
            }
            KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
                self.try_move(1, 0);
            }
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                // Soft drop gives 1 point
                if self.try_move(0, 1) {
                    self.score += 1;
                }
            }
            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                self.try_rotate();
            }
            KeyCode::Char(' ') => {
                // Hard drop gives 2 points per cell
                let start_y = self.current_y;
                self.hard_drop();
                let dropped = self.current_y - start_y;
                if dropped > 0 {
                    self.score += (dropped as u32) * 2;
                }
            }
            KeyCode::Char('c') | KeyCode::Char('C') | KeyCode::Char('h') | KeyCode::Char('H') => {
                self.hold_piece();
            }
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                return GameCommand::Exit;
            }
            _ => {}
        }
        GameCommand::None
    }

    fn draw(&self, frame: &mut Frame, area: Rect) {
        // Double size block layout: block width = 2 columns, height = 1 row
        // Tetris board takes 10cols * 2 wide = 20 width. Plus border = 22. Height = 20. Plus border = 22.
        let outer_block = Block::default()
            .title(" TETRIS CABIN ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

        frame.render_widget(outer_block, area);

        // Inner layout inside the arcade cabinet border
        let inner_area = area.inner(&ratatui::layout::Margin { horizontal: 2, vertical: 1 });
        
        let sub_layouts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(8),  // Hold Area
                Constraint::Length(22), // Main Board (20 grid width + 2 borders)
                Constraint::Min(12),   // Score & Next details
            ])
            .split(inner_area);

        let hold_area = sub_layouts[0];
        let board_area = sub_layouts[1];
        let sidebar_area = sub_layouts[2];

        // 1. Render Hold Area
        let hold_box = Block::default()
            .title("HOLD")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(if self.can_hold { Color::White } else { Color::DarkGray }));
        
        frame.render_widget(hold_box, hold_area);

        if let Some(held) = &self.held_piece {
            let mut lines = vec![Line::from(""); 4];
            let offset_x = 2i32;
            let offset_y = 1i32;
            
            // Build visual representation
            for y_cell in 0..3 {
                let mut line_spans = Vec::new();
                for x_cell in 0..3 {
                    // Check if block exists relative to center
                    let rx = x_cell as i32 - offset_x;
                    let ry = y_cell as i32 - offset_y;
                    
                    if held.blocks.contains(&(rx, ry)) {
                        line_spans.push(Span::styled("██", Style::default().fg(held.shape_type.color())));
                    } else {
                        line_spans.push(Span::raw("  "));
                    }
                }
                lines[y_cell] = Line::from(line_spans);
            }
            
            let hold_content = Paragraph::new(lines).alignment(Alignment::Center);
            let hold_inner = hold_area.inner(&ratatui::layout::Margin { horizontal: 1, vertical: 1 });
            frame.render_widget(hold_content, hold_inner);
        }

        // 2. Render Main Tetris Grid Board
        let board_box = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Gray));
        
        let board_inner = board_box.inner(board_area);
        frame.render_widget(board_box, board_area);

        // Build grid buffer
        let mut visual_grid = [[None; 10]; 20];
        
        // Locked blocks
        for y in 0..20 {
            for x in 0..10 {
                if let Some(color) = self.board[y][x] {
                    visual_grid[y][x] = Some(Style::default().fg(color));
                }
            }
        }

        // Ghost piece (faint shadow)
        if !self.game_over && !self.paused {
            let ghost_y = self.get_ghost_y();
            for &(bx, by) in &self.current_piece.blocks {
                let gx = self.current_x + bx;
                let gy = ghost_y + by;
                if gy >= 0 && gy < 20 && gx >= 0 && gx < 10 {
                    // Make it look dotted/dimmed using DarkGray/Gray outline or text representation
                    visual_grid[gy as usize][gx as usize] = Some(Style::default().fg(Color::DarkGray));
                }
            }
        }

        // Active falling piece
        if !self.game_over && !self.paused {
            for &(bx, by) in &self.current_piece.blocks {
                let px = self.current_x + bx;
                let py = self.current_y + by;
                if py >= 0 && py < 20 && px >= 0 && px < 10 {
                    visual_grid[py as usize][px as usize] = Some(Style::default().fg(self.current_piece.shape_type.color()));
                }
            }
        }

        // Apply flashes for cleared lines
        for &line_idx in &self.flash_lines {
            for x in 0..10 {
                visual_grid[line_idx][x] = Some(Style::default().fg(Color::White));
            }
        }

        // Build paragraph lines
        let mut board_rows = Vec::new();
        for y in 0..20 {
            let mut line_spans = Vec::new();
            for x in 0..10 {
                if let Some(style) = visual_grid[y][x] {
                    // Flash line is solid white
                    if !self.flash_lines.is_empty() && self.flash_lines.contains(&y) {
                        line_spans.push(Span::styled("██", style));
                    } else if style.fg == Some(Color::DarkGray) {
                        line_spans.push(Span::styled("░░", style)); // Ghost piece texturizing
                    } else {
                        line_spans.push(Span::styled("██", style));
                    }
                } else {
                    // Grid dots for polished layout
                    line_spans.push(Span::styled(". ", Style::default().fg(Color::Rgb(50, 50, 50))));
                }
            }
            board_rows.push(Line::from(line_spans));
        }

        let board_paragraph = Paragraph::new(board_rows);
        frame.render_widget(board_paragraph, board_inner);

        // 3. Render Right Sidebar (Score, Level, Next Piece)
        let side_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6), // Next piece panel
                Constraint::Length(8), // Stats panel
                Constraint::Min(4),    // Instructions
            ])
            .split(sidebar_area);

        // 3a. Next Piece Box
        let next_box = Block::default()
            .title("NEXT")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White));
        
        frame.render_widget(next_box, side_layout[0]);
        let next_inner = side_layout[0].inner(&ratatui::layout::Margin { horizontal: 1, vertical: 1 });

        let mut next_lines = vec![Line::from(""); 3];
        let next_offset_x = 2i32;
        let next_offset_y = 1i32;
        
        for y_cell in 0..3 {
            let mut spans = Vec::new();
            for x_cell in 0..3 {
                let rx = x_cell as i32 - next_offset_x;
                let ry = y_cell as i32 - next_offset_y;
                if self.next_piece.blocks.contains(&(rx, ry)) {
                    spans.push(Span::styled("██", Style::default().fg(self.next_piece.shape_type.color())));
                } else {
                    spans.push(Span::raw("  "));
                }
            }
            next_lines[y_cell] = Line::from(spans);
        }
        let next_paragraph = Paragraph::new(next_lines).alignment(Alignment::Center);
        frame.render_widget(next_paragraph, next_inner);

        // 3b. Score and Stats Panel
        let stats_content = vec![
            Line::from(vec![
                Span::styled(" SCORE  ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled(format!(" {:06}", self.score), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" LEVEL  ", Style::default().fg(Color::Yellow)),
                Span::styled(format!(" {}", self.level), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled(" LINES  ", Style::default().fg(Color::Yellow)),
                Span::styled(format!(" {}", self.lines_cleared), Style::default().fg(Color::Cyan)),
            ]),
        ];
        
        let stats_paragraph = Paragraph::new(stats_content)
            .block(Block::default().borders(Borders::ALL).title("STATS"))
            .alignment(Alignment::Left);
        frame.render_widget(stats_paragraph, side_layout[1]);

        // 3c. How to Play Instructions
        let instruct_content = vec![
            Line::from(Span::styled("  [←→] Slide", Style::default().fg(Color::Gray))),
            Line::from(Span::styled("  [↑]  Rotate", Style::default().fg(Color::Gray))),
            Line::from(Span::styled("  [↓]  Soft Drop", Style::default().fg(Color::Gray))),
            Line::from(Span::styled("  [Spc]Hard Drop", Style::default().fg(Color::Gray))),
            Line::from(Span::styled("  [C]  Hold Item", Style::default().fg(Color::Gray))),
            Line::from(Span::styled("  [P]  Pause Game", Style::default().fg(Color::Gray))),
            Line::from(Span::styled("  [Esc]Quit", Style::default().fg(Color::Gray))),
        ];
        let instruct_paragraph = Paragraph::new(instruct_content)
            .block(Block::default().borders(Borders::ALL).title("KEYS"));
        frame.render_widget(instruct_paragraph, side_layout[2]);

        // 4. Overlays (Pause / Game Over)
        if self.paused {
            let pause_area = Rect {
                x: board_inner.x + 2,
                y: board_inner.y + 7,
                width: 16,
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
                x: board_inner.x + 1,
                y: board_inner.y + 6,
                width: 18,
                height: 7,
            };
            frame.render_widget(Clear, go_area);
            let go_widget = Paragraph::new(vec![
                Line::from(""),
                Line::from(Span::styled(" GAME OVER! ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))),
                Line::from(format!("Score: {}", self.score)),
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
