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

pub struct ArithmeticGame {
    title: &'static str,
    target: u32,
    player_input: String,
    score: u32,
    high_score: u32,
    game_over: bool,
    paused: bool,
    time_limit: f32,
    status_msg: String,
    prompt: String,
}

impl ArithmeticGame {
    pub fn new() -> Self {
        let mut game = Self {
            title: "Arithmetic speed",
            target: 0,
            player_input: String::new(),
            score: 0,
            high_score: 0,
            game_over: false,
            paused: false,
            time_limit: 15.0,
            status_msg: "Type the correct answer fast!".to_string(),
            prompt: String::new(),
        };
        game.next_question();
        game
    }

    fn next_question(&mut self) {
        let mut rng = rand::thread_rng();
        let op = rng.gen_range(0..3);
        let n1 = rng.gen_range(1..20);
        let n2 = rng.gen_range(1..20);
        match op {
            0 => { self.target = n1 + n2; self.prompt = format!("{} + {} = ?", n1, n2); },
            1 => { let n1 = n1.max(n2); let n2 = n1.min(n2); self.target = n1 - n2; self.prompt = format!("{} - {} = ?", n1, n2); },
            _ => { let n1 = rng.gen_range(1..10); let n2 = rng.gen_range(1..10); self.target = n1 * n2; self.prompt = format!("{} * {} = ?", n1, n2); },
        }
        self.player_input.clear();
    }

    fn submit(&mut self) {
        let parsed = self.player_input.trim().parse::<u32>().unwrap_or(0);
        if parsed == self.target {
            self.score += 100;
            self.status_msg = "CORRECT!".to_string();
            self.time_limit = (self.time_limit + 3.0).min(15.0);
            self.next_question();
        } else {
            self.score = self.score.saturating_sub(20);
            self.status_msg = "WRONG! Try again.".to_string();
            self.player_input.clear();
        }
    }
}

impl Default for ArithmeticGame {
    fn default() -> Self {
        Self::new()
    }
}

impl Game for ArithmeticGame {
    fn update(&mut self, delta: Duration) {
        if self.game_over || self.paused {
            return;
        }
        self.time_limit -= delta.as_secs_f32();
        if self.time_limit <= 0.0 {
            self.game_over = true;
            self.status_msg = "Time's up!".to_string();
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
            KeyCode::Char(c) => {
                if c.is_digit(10) {
                    self.player_input.push(c);
                }
            }
            KeyCode::Backspace => {
                self.player_input.pop();
            }
            KeyCode::Enter => {
                self.submit();
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
            .title(format!("  {} SPEED CABINET  ", self.title.to_uppercase()))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

        frame.render_widget(outer_block, area);

        let inner_area = area.inner(&ratatui::layout::Margin { horizontal: 2, vertical: 1 });
        let layouts = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),  // Header stats
                Constraint::Min(6),     // Question
                Constraint::Length(3),  // Input panel
            ])
            .split(inner_area);

        // Header Panel
        let header_content = vec![
            Line::from(vec![
                Span::styled(" SCORE: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(format!("{:05}", self.score), Style::default().fg(Color::White)),
                Span::styled("     TIME LEFT: ", Style::default().fg(Color::Red)),
                Span::styled(format!("{:.1}s", self.time_limit), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
        ];
        frame.render_widget(Paragraph::new(header_content).alignment(Alignment::Center).block(Block::default().borders(Borders::BOTTOM)), layouts[0]);

        // Main Question Box
        let q_block = Block::default()
            .title(" SPEED CHALLENGE ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Yellow));
        
        let q_inner = q_block.inner(layouts[1]);
        frame.render_widget(q_block, layouts[1]);

        let q_content = vec![
            Line::from(""),
            Line::from(Span::styled(&self.prompt, Style::default().fg(Color::White).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(Span::styled(&self.status_msg, Style::default().fg(Color::Gray))),
        ];
        frame.render_widget(Paragraph::new(q_content).alignment(Alignment::Center), q_inner);

        // Input Panel
        let input_block = Block::default()
            .title(" YOUR ANSWER ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green));
            
        let input_inner = input_block.inner(layouts[2]);
        frame.render_widget(input_block, layouts[2]);

        let input_para = Paragraph::new(Line::from(vec![
            Span::styled(" > ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(&self.player_input, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]));
        frame.render_widget(input_para, input_inner);

        if self.paused {
            let pause_area = Rect {
                x: inner_area.x + (inner_area.width.saturating_sub(18)) / 2,
                y: inner_area.y + (inner_area.height.saturating_sub(5)) / 2,
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
                x: inner_area.x + (inner_area.width.saturating_sub(25)) / 2,
                y: inner_area.y + (inner_area.height.saturating_sub(7)) / 2,
                width: 25,
                height: 7,
            };
            frame.render_widget(Clear, go_area);
            let go_widget = Paragraph::new(vec![
                Line::from(""),
                Line::from(Span::styled(" TIME EXPIRED ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))),
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
