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

pub struct BackgammonGame {
    title: &'static str,
    player_hand: Vec<String>,
    dealer_hand: Vec<String>,
    score: u32,
    player_score: u32,
    dealer_score: u32,
    status_msg: String,
    game_over: bool,
    paused: bool,
    bet: u32,
}

impl BackgammonGame {
    pub fn new() -> Self {
        let mut game = Self {
            title: "Backgammon",
            player_hand: Vec::new(),
            dealer_hand: Vec::new(),
            score: 1000,
            player_score: 0,
            dealer_score: 0,
            status_msg: "Place your bet and Deal!".to_string(),
            game_over: false,
            paused: false,
            bet: 100,
        };
        game.deal();
        game
    }

    fn calculate_score(hand: &[String]) -> u32 {
        let mut val = 0;
        let mut aces = 0;
        for card in hand {
            let card_val = &card[0..card.len()-1];
            if card_val == "A" {
                aces += 1;
                val += 11;
            } else if card_val == "K" || card_val == "Q" || card_val == "J" || card_val == "10" {
                val += 10;
            } else {
                val += card_val.parse::<u32>().unwrap_or(0);
            }
        }
        while val > 21 && aces > 0 {
            val -= 10;
            aces -= 1;
        }
        val
    }

    fn draw_card(&self) -> String {
        let suits = vec!["♠", "♥", "♦", "♣"];
        let values = vec!["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];
        let mut rng = rand::thread_rng();
        format!("{}{}", values[rng.gen_range(0..values.len())], suits[rng.gen_range(0..suits.len())])
    }

    fn deal(&mut self) {
        self.player_hand.clear();
        self.dealer_hand.clear();
        self.player_hand.push(self.draw_card());
        self.player_hand.push(self.draw_card());
        self.dealer_hand.push(self.draw_card());
        self.dealer_hand.push(self.draw_card());
        
        self.player_score = Self::calculate_score(&self.player_hand);
        self.dealer_score = Self::calculate_score(&self.dealer_hand);
        
        self.status_msg = "Hit [H] or Stand [S]?".to_string();
        if self.player_score == 21 {
            self.stand();
        }
    }

    fn hit(&mut self) {
        if self.game_over || self.paused {
            return;
        }
        self.player_hand.push(self.draw_card());
        self.player_score = Self::calculate_score(&self.player_hand);
        if self.player_score > 21 {
            self.score = self.score.saturating_sub(self.bet);
            self.status_msg = "Bust! You Lose!".to_string();
            self.game_over = true;
        }
    }

    fn stand(&mut self) {
        if self.game_over || self.paused {
            return;
        }
        while self.dealer_score < 17 {
            self.dealer_hand.push(self.draw_card());
            self.dealer_score = Self::calculate_score(&self.dealer_hand);
        }
        
        if self.dealer_score > 21 {
            self.score += self.bet;
            self.status_msg = "Dealer Busts! You Win!".to_string();
        } else if self.player_score > self.dealer_score {
            self.score += self.bet;
            self.status_msg = "You Win!".to_string();
        } else if self.player_score < self.dealer_score {
            self.score = self.score.saturating_sub(self.bet);
            self.status_msg = "Dealer Wins!".to_string();
        } else {
            self.status_msg = "Push! Draw!".to_string();
        }
        self.game_over = true;
    }
}

impl Default for BackgammonGame {
    fn default() -> Self {
        Self::new()
    }
}

impl Game for BackgammonGame {
    fn update(&mut self, _delta: Duration) {
        // Turn-based card game
    }

    fn handle_input(&mut self, key: KeyCode) -> GameCommand {
        if self.game_over {
            match key {
                KeyCode::Char('r') | KeyCode::Char('R') | KeyCode::Enter | KeyCode::Char(' ') => {
                    self.deal();
                    self.game_over = false;
                    return GameCommand::Restart;
                }
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
            KeyCode::Char('h') | KeyCode::Char('H') => self.hit(),
            KeyCode::Char('s') | KeyCode::Char('S') => self.stand(),
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                return GameCommand::Exit;
            }
            _ => {}
        }

        GameCommand::None
    }

    fn draw(&self, frame: &mut Frame, area: Rect, _palette: &crate::settings::ThemePalette) {
        let outer_block = Block::default()
            .title(format!("  {} CABINET  ", self.title.to_uppercase()))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

        frame.render_widget(outer_block, area);

        let inner_area = area.inner(&ratatui::layout::Margin { horizontal: 2, vertical: 1 });
        let layouts = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(45), // Dealer Hand
                Constraint::Percentage(45), // Player Hand
                Constraint::Length(3),      // Status Log
            ])
            .split(inner_area);

        let dealer_block = Block::default()
            .title(" DEALER HAND ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Red));
        
        let d_inner = dealer_block.inner(layouts[0]);
        frame.render_widget(dealer_block, layouts[0]);

        let player_block = Block::default()
            .title(" PLAYER HAND ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Green));
            
        let p_inner = player_block.inner(layouts[1]);
        frame.render_widget(player_block, layouts[1]);

        // Draw Hands
        let mut d_lines = vec![Line::from("")];
        let mut d_spans = Vec::new();
        for (i, card) in self.dealer_hand.iter().enumerate() {
            if i == 1 && !self.game_over {
                d_spans.push(Span::styled(" [🂠 HIDDEN] ", Style::default().fg(Color::DarkGray)));
            } else {
                d_spans.push(Span::styled(format!(" [{}] ", card), Style::default().fg(Color::White).bg(Color::Rgb(20, 20, 20))));
            }
        }
        d_lines.push(Line::from(d_spans));
        if self.game_over {
            d_lines.push(Line::from(""));
            d_lines.push(Line::from(Span::styled(format!("  Score: {}", self.dealer_score), Style::default().fg(Color::Gray))));
        }
        frame.render_widget(Paragraph::new(d_lines), d_inner);

        let mut p_lines = vec![Line::from("")];
        let mut p_spans = Vec::new();
        for card in &self.player_hand {
            p_spans.push(Span::styled(format!(" [{}] ", card), Style::default().fg(Color::Green).bg(Color::Rgb(20, 20, 20))));
        }
        p_lines.push(Line::from(p_spans));
        p_lines.push(Line::from(""));
        p_lines.push(Line::from(Span::styled(format!("  Score: {}", self.player_score), Style::default().fg(Color::Green))));
        frame.render_widget(Paragraph::new(p_lines), p_inner);

        // Status Msg
        let status_para = Paragraph::new(Line::from(Span::styled(&self.status_msg, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))))
            .alignment(Alignment::Center);
        frame.render_widget(status_para, layouts[2]);

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
                Line::from(Span::styled("Press [Tab] to resume", Style::default().fg(Color::DarkGray))),
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
                Line::from(Span::styled(" GAME OVER ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))),
                Line::from(format!("Chips Remaining: {}", self.score)),
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
