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

pub struct BattlestarGame {
    title: &'static str,
    current_room: usize,
    inventory: Vec<String>,
    score: u32,
    game_over: bool,
    paused: bool,
    moves: u32,
    log: Vec<String>,
    choices: Vec<String>,
}

struct Room {
    name: &'static str,
    desc: &'static str,
    choices: Vec<&'static str>,
}

impl BattlestarGame {
    pub fn new() -> Self {
        let mut game = Self {
            title: "Battlestar",
            current_room: 0,
            inventory: Vec::new(),
            score: 0,
            game_over: false,
            paused: false,
            moves: 0,
            log: vec!["Welcome to the game!".to_string()],
            choices: Vec::new(),
        };
        game.update_choices();
        game
    }

    fn get_rooms(&self) -> Vec<Room> {
        vec![
            Room { name: "Starship Bridge", desc: "Flashing red lights indicate system failures. Controls are East, Engine Room is South.", choices: vec!["Go East", "Go South", "Scan Systems"] },
            Room { name: "Officer Quarters", desc: "A cozy bunk bed. Under the pillow is an access keycard. Bridge is West.", choices: vec!["Go West", "Take Keycard", "Search Quarters"] },
            Room { name: "Engine Core", desc: "The warp drive is offline. An empty slot awaits a fuel cell.", choices: vec!["Go North", "Insert Fuel Cell", "Check Core Status"] }
        ]
    }

    fn update_choices(&mut self) {
        let rooms = self.get_rooms();
        if self.current_room < rooms.len() {
            self.choices = rooms[self.current_room].choices.iter().map(|s| s.to_string()).collect();
        }
    }

    fn handle_choice(&mut self, choice_idx: usize) {
        if choice_idx >= self.choices.len() {
            return;
        }
        let choice = self.choices[choice_idx].clone();
        self.moves += 1;
        self.log.push(format!("> Selected: {}", choice));
        
        match self.current_room {
            0 => match choice_idx {
                0 => { self.current_room = 1; self.log.push("Moved East.".to_string()); }
                1 => { self.current_room = 2; self.log.push("Moved South.".to_string()); }
                _ => { self.log.push("All thrusters are locked.".to_string()); }
            },
            1 => match choice_idx {
                0 => { self.current_room = 0; self.log.push("Returned West.".to_string()); }
                1 => {
                    if !self.inventory.contains(&"Keycard".to_string()) {
                        self.inventory.push("Keycard".to_string());
                        self.log.push("Obtained Keycard!".to_string());
                        self.score += 100;
                    } else {
                        self.log.push("Already taken.".to_string());
                    }
                }
                _ => { self.log.push("Personal logs show everything is lost.".to_string()); }
            },
            2 => match choice_idx {
                0 => { self.current_room = 0; self.log.push("Returned North.".to_string()); }
                1 => {
                    if self.inventory.contains(&"Keycard".to_string()) {
                        self.score += 600;
                        self.log.push("Core online! We are ready to warp!".to_string());
                        self.game_over = true;
                    } else {
                        self.log.push("You need the Keycard to unlock core interface.".to_string());
                    }
                }
                _ => { self.log.push("Subsystems are running on backup batteries.".to_string()); }
            },
            _ => {}
        }
        
        self.update_choices();
    }
}

impl Default for BattlestarGame {
    fn default() -> Self {
        Self::new()
    }
}

impl Game for BattlestarGame {
    fn update(&mut self, _delta: Duration) {
        // Turn based, updates on input
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
            KeyCode::Char('1') => self.handle_choice(0),
            KeyCode::Char('2') => self.handle_choice(1),
            KeyCode::Char('3') => self.handle_choice(2),
            KeyCode::Char('4') => self.handle_choice(3),
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

    fn draw(&self, frame: &mut Frame, area: Rect, _palette: &crate::settings::ThemePalette) {
        let outer_block = Block::default()
            .title(format!("  {} CABINET  ", self.title.to_uppercase()))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD));

        frame.render_widget(outer_block, area);

        let inner_area = area.inner(&ratatui::layout::Margin { horizontal: 2, vertical: 1 });
        let layouts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(20),
                Constraint::Length(25),
            ])
            .split(inner_area);

        let main_area = layouts[0];
        let side_area = layouts[1];

        // Main Panel Split: Room Description (50%), Options/Logs (50%)
        let main_split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(main_area);

        let rooms = self.get_rooms();
        let current_room_name = if self.current_room < rooms.len() { rooms[self.current_room].name } else { "Unknown" };
        let current_room_desc = if self.current_room < rooms.len() { rooms[self.current_room].desc } else { "Lost in time and space." };

        // Room Box
        let room_block = Block::default()
            .title(format!(" ROOM: {} ", current_room_name))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan));
        
        let room_inner = room_block.inner(main_split[0]);
        frame.render_widget(room_block, main_split[0]);

        let mut room_text = vec![
            Line::from(""),
            Line::from(Span::styled(current_room_desc, Style::default().fg(Color::White))),
            Line::from(""),
        ];

        let room_paragraph = Paragraph::new(room_text).wrap(ratatui::widgets::Wrap { trim: true });
        frame.render_widget(room_paragraph, room_inner);

        // Controls and choices Box
        let choices_block = Block::default()
            .title(" ACTIONS / CHOICES ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Yellow));
        
        let choices_inner = choices_block.inner(main_split[1]);
        frame.render_widget(choices_block, main_split[1]);

        let mut choice_lines = Vec::new();
        for (i, c) in self.choices.iter().enumerate() {
            choice_lines.push(Line::from(vec![
                Span::styled(format!("  [{}] ", i + 1), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(c, Style::default().fg(Color::White)),
            ]));
            choice_lines.push(Line::from(""));
        }

        let choices_paragraph = Paragraph::new(choice_lines);
        frame.render_widget(choices_paragraph, choices_inner);

        // Sidebar stats
        let side_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(10), // Stats
                Constraint::Min(6),    // Inventory & Log
            ])
            .split(side_area);

        let stats_content = vec![
            Line::from(vec![
                Span::styled(" SCORE: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(format!("{}", self.score), Style::default().fg(Color::White)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" MOVES: ", Style::default().fg(Color::Cyan)),
                Span::styled(format!("{}", self.moves), Style::default().fg(Color::White)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" ROOMS: ", Style::default().fg(Color::LightGreen)),
                Span::styled(format!("{}/{}", self.current_room + 1, rooms.len()), Style::default().fg(Color::White)),
            ]),
        ];

        let stats_paragraph = Paragraph::new(stats_content)
            .block(Block::default().borders(Borders::ALL).title("STATS"));
        frame.render_widget(stats_paragraph, side_layout[0]);

        // Inventory & Logs list
        let mut inv_content = vec![
            Line::from(Span::styled(" INVENTORY:", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))),
        ];
        if self.inventory.is_empty() {
            inv_content.push(Line::from(Span::styled("  (empty)", Style::default().fg(Color::DarkGray))));
        } else {
            for item in &self.inventory {
                inv_content.push(Line::from(Span::styled(format!("  • {}", item), Style::default().fg(Color::Green))));
            }
        }
        inv_content.push(Line::from(""));
        inv_content.push(Line::from(Span::styled(" RECENT LOG:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))));
        
        let start_idx = self.log.len().saturating_sub(4);
        for entry in &self.log[start_idx..] {
            inv_content.push(Line::from(Span::styled(entry, Style::default().fg(Color::Gray))));
        }

        let inv_paragraph = Paragraph::new(inv_content)
            .block(Block::default().borders(Borders::ALL).title("ADVENTURE LOG"));
        frame.render_widget(inv_paragraph, side_layout[1]);

        if self.paused {
            let pause_area = Rect {
                x: main_area.x + (main_area.width.saturating_sub(22)) / 2,
                y: main_area.y + (main_area.height.saturating_sub(5)) / 2,
                width: 22,
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
                x: main_area.x + (main_area.width.saturating_sub(30)) / 2,
                y: main_area.y + (main_area.height.saturating_sub(8)) / 2,
                width: 30,
                height: 8,
            };
            frame.render_widget(Clear, go_area);
            
            let message = vec![
                Line::from(""),
                Line::from(Span::styled(" QUEST COMPLETED! ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))),
                Line::from(format!("Final Score: {}", self.score)),
                Line::from(format!("Moves: {}", self.moves)),
                Line::from(""),
                Line::from(Span::styled("Press [R] to replay", Style::default().fg(Color::Green))),
                Line::from(Span::styled("Press [Esc] to exit", Style::default().fg(Color::DarkGray))),
            ];

            let go_widget = Paragraph::new(message)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Green)));
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
