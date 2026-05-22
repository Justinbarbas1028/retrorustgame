use std::time::Duration;

use crossterm::event::KeyCode;
use ratatui::{
    layout::{Alignment, Constraint, Direction as LayoutDirection, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use super::{Game, GameCommand};

const BOARD_WIDTH: i32 = 34;
const BOARD_HEIGHT: i32 = 16;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct TronGame {
    player: (i32, i32),
    player_dir: Direction,
    ai: (i32, i32),
    ai_dir: Direction,
    player_trail: Vec<(i32, i32)>,
    ai_trail: Vec<(i32, i32)>,
    score: u32,
    game_over: bool,
    player_won: bool,
    paused: bool,
    tick_accumulator: Duration,
}

impl TronGame {
    pub fn new() -> Self {
        let player = (6, BOARD_HEIGHT / 2);
        let ai = (BOARD_WIDTH - 7, BOARD_HEIGHT / 2);
        Self {
            player,
            player_dir: Direction::Right,
            ai,
            ai_dir: Direction::Left,
            player_trail: vec![player],
            ai_trail: vec![ai],
            score: 0,
            game_over: false,
            player_won: false,
            paused: false,
            tick_accumulator: Duration::from_secs(0),
        }
    }

    fn tick_rate(&self) -> Duration {
        let speedup = (self.score / 250).min(60) as u64;
        Duration::from_millis(125_u64.saturating_sub(speedup).max(55))
    }

    fn turn(&mut self, direction: Direction) {
        if !Self::opposites(self.player_dir, direction) {
            self.player_dir = direction;
        }
    }

    fn opposites(a: Direction, b: Direction) -> bool {
        matches!(
            (a, b),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
        )
    }

    fn step(position: (i32, i32), direction: Direction) -> (i32, i32) {
        match direction {
            Direction::Up => (position.0, position.1 - 1),
            Direction::Down => (position.0, position.1 + 1),
            Direction::Left => (position.0 - 1, position.1),
            Direction::Right => (position.0 + 1, position.1),
        }
    }

    fn is_wall(position: (i32, i32)) -> bool {
        position.0 < 0
            || position.1 < 0
            || position.0 >= BOARD_WIDTH
            || position.1 >= BOARD_HEIGHT
    }

    fn is_occupied(&self, position: (i32, i32)) -> bool {
        self.player_trail.contains(&position) || self.ai_trail.contains(&position)
    }

    fn choose_ai_direction(&self) -> Direction {
        let candidates = match self.ai_dir {
            Direction::Up => [Direction::Up, Direction::Left, Direction::Right, Direction::Down],
            Direction::Down => [Direction::Down, Direction::Right, Direction::Left, Direction::Up],
            Direction::Left => [Direction::Left, Direction::Down, Direction::Up, Direction::Right],
            Direction::Right => [Direction::Right, Direction::Up, Direction::Down, Direction::Left],
        };

        candidates
            .into_iter()
            .find(|direction| {
                let next = Self::step(self.ai, *direction);
                !Self::is_wall(next) && !self.is_occupied(next)
            })
            .unwrap_or(self.ai_dir)
    }

    fn advance(&mut self) {
        self.ai_dir = self.choose_ai_direction();
        let next_player = Self::step(self.player, self.player_dir);
        let next_ai = Self::step(self.ai, self.ai_dir);
        let player_crashed =
            Self::is_wall(next_player) || self.is_occupied(next_player) || next_player == next_ai;
        let ai_crashed = Self::is_wall(next_ai) || self.is_occupied(next_ai);

        if player_crashed || ai_crashed {
            self.game_over = true;
            self.player_won = ai_crashed && !player_crashed;
            if self.player_won {
                self.score += 500;
            }
            return;
        }

        self.player = next_player;
        self.ai = next_ai;
        self.player_trail.push(next_player);
        self.ai_trail.push(next_ai);
        self.score += 10;
    }
}

impl Default for TronGame {
    fn default() -> Self {
        Self::new()
    }
}

impl Game for TronGame {
    fn update(&mut self, delta: Duration) {
        if self.game_over || self.paused {
            return;
        }

        self.tick_accumulator += delta;
        if self.tick_accumulator >= self.tick_rate() {
            self.tick_accumulator = Duration::from_secs(0);
            self.advance();
        }
    }

    fn handle_input(&mut self, key: KeyCode) -> GameCommand {
        if self.game_over {
            match key {
                KeyCode::Char('r') | KeyCode::Char('R') | KeyCode::Enter => {
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
            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => self.turn(Direction::Up),
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => self.turn(Direction::Down),
            KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => self.turn(Direction::Left),
            KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => self.turn(Direction::Right),
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => return GameCommand::Exit,
            _ => {}
        }

        GameCommand::None
    }

    fn draw(&self, frame: &mut Frame, area: Rect) {
        let outer_block = Block::default()
            .title("  TRON LIGHT CYCLES  ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
        frame.render_widget(outer_block, area);

        let inner_area = area.inner(&ratatui::layout::Margin { horizontal: 2, vertical: 1 });
        let layouts = Layout::default()
            .direction(LayoutDirection::Horizontal)
            .constraints([Constraint::Length((BOARD_WIDTH * 2 + 2) as u16), Constraint::Min(18)])
            .split(inner_area);

        let board_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Blue));
        let board_inner = board_block.inner(layouts[0]);
        frame.render_widget(board_block, layouts[0]);

        let mut rows = Vec::new();
        for y in 0..BOARD_HEIGHT {
            let mut spans = Vec::new();
            for x in 0..BOARD_WIDTH {
                let position = (x, y);
                let (symbol, style) = if position == self.player {
                    ("[]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                } else if position == self.ai {
                    ("<>", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                } else if self.player_trail.contains(&position) {
                    ("##", Style::default().fg(Color::LightBlue))
                } else if self.ai_trail.contains(&position) {
                    ("##", Style::default().fg(Color::LightRed))
                } else {
                    ("  ", Style::default())
                };
                spans.push(Span::styled(symbol, style));
            }
            rows.push(Line::from(spans));
        }
        frame.render_widget(Paragraph::new(rows), board_inner);

        let side_content = vec![
            Line::from(vec![
                Span::styled(" SCORE: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(format!("{}", self.score), Style::default().fg(Color::White)),
            ]),
            Line::from(""),
            Line::from(Span::styled(" Stay ahead of your trail.", Style::default().fg(Color::Gray))),
            Line::from(Span::styled(" Force the red cycle into", Style::default().fg(Color::Gray))),
            Line::from(Span::styled(" a wall or light path.", Style::default().fg(Color::Gray))),
            Line::from(""),
            Line::from(Span::styled(" [Arrows/WASD] Turn", Style::default().fg(Color::Gray))),
            Line::from(Span::styled(" [P] Pause", Style::default().fg(Color::Gray))),
            Line::from(Span::styled(" [Esc/Q] Quit", Style::default().fg(Color::Gray))),
        ];
        frame.render_widget(
            Paragraph::new(side_content).block(Block::default().borders(Borders::ALL).title("GRID STATUS")),
            layouts[1],
        );

        if self.paused {
            let pause_area = Rect {
                x: board_inner.x + (BOARD_WIDTH * 2 - 18) as u16 / 2,
                y: board_inner.y + 5,
                width: 18,
                height: 5,
            };
            frame.render_widget(Clear, pause_area);
            let pause_widget = Paragraph::new(vec![
                Line::from(""),
                Line::from(Span::styled(" PAUSED ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
                Line::from(Span::styled("Press P to resume", Style::default().fg(Color::DarkGray))),
            ])
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Yellow)));
            frame.render_widget(pause_widget, pause_area);
        } else if self.game_over {
            let go_area = Rect {
                x: board_inner.x + (BOARD_WIDTH * 2 - 28) as u16 / 2,
                y: board_inner.y + 5,
                width: 28,
                height: 7,
            };
            frame.render_widget(Clear, go_area);
            let title = if self.player_won { " GRID CAPTURED " } else { " DEREZZED " };
            let color = if self.player_won { Color::Green } else { Color::Red };
            let go_widget = Paragraph::new(vec![
                Line::from(""),
                Line::from(Span::styled(title, Style::default().fg(color).add_modifier(Modifier::BOLD))),
                Line::from(format!("Final Score: {}", self.score)),
                Line::from(""),
                Line::from(Span::styled("Press [R] to retry", Style::default().fg(Color::Green))),
                Line::from(Span::styled("Press [Esc] to exit", Style::default().fg(Color::DarkGray))),
            ])
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(color)));
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
