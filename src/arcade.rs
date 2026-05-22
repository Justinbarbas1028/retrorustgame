use std::time::Duration;
use ratatui::{
    Frame,
    layout::{Rect, Layout, Constraint, Direction, Alignment},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Paragraph, Wrap},
};
use crossterm::event::KeyCode;
use crate::high_scores::HighScores;
use crate::games::{
    Game, GameType, GameCommand,
    tetris::TetrisGame,
    game_2048::Game2048,
    minesweeper::MinesweeperGame,
    space_invaders::SpaceInvadersGame,
    roguelike::RoguelikeGame,
    snake::SnakeGame,
    wordle::WordleGame,
    battleship::BattleshipGame,
    hangman::HangmanGame,
    pong::PongGame,
};

pub struct ArcadeConsole {
    games: Vec<GameType>,
    selected_index: usize,
    active_game: Option<Box<dyn Game>>,
    active_game_type: Option<GameType>,
    high_scores: HighScores,
}

impl Default for ArcadeConsole {
    fn default() -> Self {
        Self::new()
    }
}

impl ArcadeConsole {
    pub fn new() -> Self {
        Self {
            games: vec![
                GameType::Tetris,
                GameType::Game2048,
                GameType::Minesweeper,
                GameType::SpaceInvaders,
                GameType::Roguelike,
                GameType::Snake,
                GameType::Wordle,
                GameType::Battleship,
                GameType::Hangman,
                GameType::Pong,
            ],
            selected_index: 0,
            active_game: None,
            active_game_type: None,
            high_scores: HighScores::load(),
        }
    }

    fn create_game(game_type: GameType) -> Box<dyn Game> {
        match game_type {
            GameType::Tetris => Box::new(TetrisGame::new()),
            GameType::Game2048 => Box::new(Game2048::new()),
            GameType::Minesweeper => Box::new(MinesweeperGame::new()),
            GameType::SpaceInvaders => Box::new(SpaceInvadersGame::new()),
            GameType::Roguelike => Box::new(RoguelikeGame::new()),
            GameType::Snake => Box::new(SnakeGame::new()),
            GameType::Wordle => Box::new(WordleGame::new()),
            GameType::Battleship => Box::new(BattleshipGame::new()),
            GameType::Hangman => Box::new(HangmanGame::new()),
            GameType::Pong => Box::new(PongGame::new()),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        if let Some(game) = &mut self.active_game {
            game.update(delta);
            
            // Check if game ended to update and save high score in real-time
            if game.is_game_over() {
                if let Some(game_type) = self.active_game_type {
                    let score = game.get_score();
                    self.high_scores.update_score(game_type.name(), score);
                }
            }
        }
    }

    pub fn handle_input(&mut self, key: KeyCode) -> bool {
        // Returns true if we should terminate the whole application
        if let Some(game) = &mut self.active_game {
            match game.handle_input(key) {
                GameCommand::Exit => {
                    // Update high score on exit
                    if let Some(game_type) = self.active_game_type {
                        let final_score = game.get_score();
                        self.high_scores.update_score(game_type.name(), final_score);
                    }
                    self.active_game = None;
                    self.active_game_type = None;
                }
                GameCommand::Restart => {
                    if let Some(game_type) = self.active_game_type {
                        self.active_game = Some(Self::create_game(game_type));
                    }
                }
                GameCommand::None => {}
            }
            false
        } else {
            // Main menu navigation
            match key {
                KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                    if self.selected_index > 0 {
                        self.selected_index -= 1;
                    } else {
                        self.selected_index = self.games.len() - 1; // Wrap around
                    }
                }
                KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                    if self.selected_index < self.games.len() - 1 {
                        self.selected_index += 1;
                    } else {
                        self.selected_index = 0; // Wrap around
                    }
                }
                KeyCode::Enter | KeyCode::Char(' ') => {
                    let selected = self.games[self.selected_index];
                    self.active_game = Some(Self::create_game(selected));
                    self.active_game_type = Some(selected);
                }
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                    return true; // Quit the entire arcade console
                }
                _ => {}
            }
            false
        }
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        if let Some(game) = &self.active_game {
            // Render active game full-frame
            game.draw(frame, area);
        } else {
            // Render Main Arcade Hub UI
            let outer_block = Block::default()
                .title(" RUST RETRO ARCADE CABINET ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
            
            frame.render_widget(outer_block, area);

            let inner_area = area.inner(&ratatui::layout::Margin { horizontal: 2, vertical: 1 });

            // Layout split: Header, Center, Footer
            let main_layouts = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(5), // Glowing neon retro header
                    Constraint::Min(12),   // Game list & Preview panel
                    Constraint::Length(3), // Footer buttons
                ])
                .split(inner_area);

            let header_area = main_layouts[0];
            let center_area = main_layouts[1];
            let footer_area = main_layouts[2];

            // 1. Draw glowing header
            let header_content = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled(" 👾 🕹️  R U S T   R E T R O   A R C A D E   C A B I N E T  🕹️ 👾 ", 
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                    ),
                ]),
                Line::from(vec![
                    Span::styled("════════════════════════════════════════════════════════════════════════", 
                        Style::default().fg(Color::Rgb(100, 100, 100))
                    ),
                ]),
            ];
            let header_paragraph = Paragraph::new(header_content).alignment(Alignment::Center);
            frame.render_widget(header_paragraph, header_area);

            // 2. Draw Center Split: Left = Menu list, Right = Game detail card
            let center_split = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(40), // Left List
                    Constraint::Percentage(60), // Right Card
                ])
                .split(center_area);

            let list_area = center_split[0];
            let card_area = center_split[1];

            // 2a. Draw Game List Panel
            let list_block = Block::default()
                .title(" SELECT GAME ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::White));
            
            let list_inner = list_block.inner(list_area);
            frame.render_widget(list_block, list_area);

            let mut game_rows = Vec::new();
            game_rows.push(Line::from(""));

            for (idx, game) in self.games.iter().enumerate() {
                let is_selected = idx == self.selected_index;
                let mut spans = Vec::new();

                if is_selected {
                    spans.push(Span::styled(" ▶ 🕹️  ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
                    spans.push(Span::styled(
                        format!("{:<20}", game.name()),
                        Style::default().fg(Color::Yellow).bg(Color::Rgb(50, 50, 0)).add_modifier(Modifier::BOLD)
                    ));
                } else {
                    spans.push(Span::styled("      ", Style::default().fg(Color::DarkGray)));
                    spans.push(Span::styled(
                        format!("{:<20}", game.name()),
                        Style::default().fg(Color::Gray)
                    ));
                }

                game_rows.push(Line::from(spans));
                game_rows.push(Line::from("")); // Double spacing
            }

            let list_paragraph = Paragraph::new(game_rows);
            frame.render_widget(list_paragraph, list_inner);

            // 2b. Draw Preview Card details panel
            let selected_game = self.games[self.selected_index];
            let high_score = self.high_scores.get_score(selected_game.name());

            let card_block = Block::default()
                .title(" GAME PREVIEW ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan));
            
            let card_inner = card_block.inner(card_area);
            frame.render_widget(card_block, card_area);

            // Detailed descriptions and stylized logo art
            let (logo_art, desc, controls) = match selected_game {
                GameType::Tetris => (
                    vec![
                        "  ██████╗████████╗██████╗ ██╗███████╗",
                        "  ╚══██╔══╝╚══██╔══╝██╔══██╗██║██╔════╝",
                        "     ██║      ██║   ██████╔╝██║███████╗",
                        "     ██║      ██║   ██╔══██╗██║╚════██║",
                        "     ██║      ██║   ██║  ██║██║███████║",
                    ],
                    "A classic real-time tile match game! Fit falling tetrominoes together to clear complete horizontal lines. Features increasing levels, speeds, and block previews.",
                    "Move: Left/Right Arrow  •  Rotate: Up Arrow  •  Soft Drop: Down Arrow  •  Hard Drop: Space  •  Hold: C / Shift  •  Pause: P",
                ),
                GameType::Game2048 => (
                    vec![
                        "  ██████╗  ██████╗  ██╗  ██╗  ██████╗ ",
                        "  ╚════██╗██╔═══██╗██║  ██║██╔════██╗",
                        "   █████╔╝██║   ██║███████║╚██████╔╝",
                        "  ██╔═══╝ ██║   ██║╚════██║██╔═══██╗",
                        "  ███████╗╚██████╔╝     ██║╚██████╔╝",
                    ],
                    "Slide numeric tiles on a 4x4 grid. When two matching values touch, they combine into a double-value tile! Work your way up to make the prestigious 2048 block.",
                    "Slide: Arrow Keys / WASD  •  Pause: P  •  Quit Arcade: Escape",
                ),
                GameType::Minesweeper => (
                    vec![
                        "  ███╗   ███╗██╗███╗   ██╗███████╗",
                        "  ████╗ ████║██║████╗  ██║██╔════╝",
                        "  ██╔████╔██║██║██╔██╗ ██║█████╗  ",
                        "  ██║╚██╔╝██║██║██║╚██╗██║██╔══╝  ",
                        "  ██║ ╚═╝ ██║██║██║ ╚████║███████╗",
                    ],
                    "A pure logic deduction puzzle. Clean the safe grid cells without setting off any hidden mines. Use flags to pinpoint mines and clear pockets with cascading opens.",
                    "Move Cursor: Arrow Keys / WASD  •  Dig Cell: Space / Enter  •  Toggle Flag: F  •  Pause: P",
                ),
                GameType::SpaceInvaders => (
                    vec![
                        "  ███████╗██████╗  █████╗  ██████╗███████╗",
                        "  ██╔════╝██╔══██╗██╔══██╗██╔════╝██╔════╝",
                        "  ███████╗██████╔╝███████║██║     █████╗  ",
                        "  ╚════██║██╔═══╝ ██╔══██║██║     ██╔══╝  ",
                        "  ███████║██║     ██║  ██║╚██████╗███████╗",
                    ],
                    "Retro real-time action shooter! Defend your base ship from swarms of descending invaders. Hide behind bunkers, fire lasers, and survive waves of incoming threats.",
                    "Move Ship: Left/Right Arrow or A/D  •  Shoot Laser: Space / Up Arrow / W  •  Pause: P",
                ),
                GameType::Roguelike => (
                    vec![
                        "  ██████╗  ██████╗  ██████╗ ██╗   ██╗███████╗",
                        "  ██╔══██╗██╔═══██╗██╔════╝ ██║   ██║██╔════╝",
                        "  ██████╔╝██║   ██║██║  ███╗██║   ██║█████╗  ",
                        "  ██╔══██╗██║   ██║██║   ██║██║   ██║██╔══╝  ",
                        "  ██║  ██║╚██████╔╝╚██████╔╝╚██████╔╝███████╗",
                    ],
                    "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find swords/shields, fight goblins/orcs/trolls, and go deep down the stairs.",
                    "Move/Attack: Arrow Keys / WASD  •  Skip Turn: Space / Period  •  Pause: P",
                ),
                GameType::Snake => (
                    vec![
                        "  ██████╗███╗   ██╗ █████╗ ██╗  ██╗███████╗",
                        "  ██╔════╝████╗  ██║██╔══██╗██║  ██║██╔════╝",
                        "  ███████╗██╔██╗ ██║███████║███████║█████╗  ",
                        "  ╚════██║██║╚██╗██║██╔══██║██╔══██║██╔══╝  ",
                        "  ███████║██║ ╚████║██║  ██║██║  ██║███████╗",
                    ],
                    "A classic real-time retro arcade game. Control the snake to eat food and grow longer, but avoid crashing into walls or yourself! Includes dynamic speed scaling as your score climbs.",
                    "Move: Arrow Keys / WASD  •  Pause: P  •  Quit Game: Escape",
                ),
                GameType::Wordle => (
                    vec![
                        "  ██╗    ██╗ ██████╗ ██████╗ ██████╗ ██╗     ███████╗",
                        "  ██║    ██║██╔═══██╗██╔══██╗██╔══██╗██║     ██╔════╝",
                        "  ██║ █╗ ██║██║   ██║██████╔╝██║  ██║██║     █████╗  ",
                        "  ██║███╗██║██║   ██║██╔══██╗██║  ██║██║     ██╔══╝  ",
                        "  ╚███╔███╔╝╚██████╔╝██║  ██║██████╔╝███████╗███████╗",
                    ],
                    "Challenge your vocabulary with this 5-letter word game! You have 6 attempts to guess the secret word. Features colorful elimination keyboard grids and letter status indicators.",
                    "Type Letters: A-Z  •  Delete: Backspace  •  Submit Word: Enter  •  Pause: P  •  Quit Game: Escape",
                ),
                GameType::Battleship => (
                    vec![
                        "  ██████╗  █████╗ ████████╗████████╗██╗     ███████╗",
                        "  ██╔══██╗██╔══██╗╚══██╔══╝╚══██╔══╝██║     ██╔════╝",
                        "  ██████╔╝███████║   ██║      ██║   ██║     █████╗  ",
                        "  ██╔══██╗██╔══██║   ██║      ██║   ██║     ██╔══╝  ",
                        "  ██████╔╝██║  ██║   ██║      ██║   ███████╗███████╗",
                    ],
                    "A classic grid-based tactical warfare game. Position your battleships on the 10x10 ocean grid and take turns with the AI targeting and firing salvos to sink the enemy fleet.",
                    "Move Cursor: Arrows / WASD  •  Rotate Ship: R  •  Confirm/Fire: Enter/Space  •  Pause: P  •  Quit: Escape",
                ),
                GameType::Hangman => (
                    vec![
                        "  ██╗  ██╗ █████╗ ███╗   ██╗ ██████╗███╗   ███╗ █████╗ ███╗   ██╗",
                        "  ██║  ██║██╔══██╗████╗  ██║██╔════╝████╗ ████║██╔══██╗████╗  ██║",
                        "  ██║ █╗ ██║██║   ██║██████╔╝██║  ██║██║     █████╗  ",
                        "  ██║███╗██║██║   ██║██╔══██╗██║  ██║██║     ██╔══╝  ",
                        "  ╚███╔███╔╝╚██████╔╝██║  ██║██████╔╝███████╗███████╗",
                    ],
                    "The timeless word-guessing challenge! Guess letters one by one to reveal the secret word. Each incorrect guess adds a piece to the classic ASCII gallows art.",
                    "Guess Letter: A-Z  •  Pause: P  •  Quit Game: Escape",
                ),
                GameType::Pong => (
                    vec![
                        "  ██████╗  ██████╗ ███╗   ██╗ ██████╗ ",
                        "  ██╔══██╗██╔═══██╗████╗  ██║██╔════╝ ",
                        "  ██████╔╝██║   ██║██╔██╗ ██║██║  ███╗",
                        "  ██╔═══╝ ██║   ██║██║╚██╗██║██║   ██║",
                        "  ██║     ╚██████╔╝██║ ╚████║╚██████╔╝",
                    ],
                    "The grandfather of video games, rebuilt in full glory! Control your left paddle in real-time to deflect the ball past the computer opponent. Features smooth ball spin physics.",
                    "Move Paddle: Up/Down Arrows / W/S  •  Pause: P  •  Quit Game: Escape",
                ),
            };

            let mut card_rows = Vec::new();
            card_rows.push(Line::from(""));
            
            // Draw logo art
            for row in logo_art {
                card_rows.push(Line::from(vec![
                    Span::styled(row, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                ]));
            }
            card_rows.push(Line::from(""));

            // Description
            card_rows.push(Line::from(vec![
                Span::styled("   ABOUT: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]));
            card_rows.push(Line::from(format!("   {}", desc)));
            card_rows.push(Line::from(""));

            // High Score
            card_rows.push(Line::from(vec![
                Span::styled("   PERSONAL HIGH SCORE: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(format!(" {:06}", high_score), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            ]));
            card_rows.push(Line::from(""));

            // Controls
            card_rows.push(Line::from(vec![
                Span::styled("   CONTROLS: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]));
            card_rows.push(Line::from(format!("   {}", controls)));

            let card_paragraph = Paragraph::new(card_rows)
                .wrap(Wrap { trim: false });
            frame.render_widget(card_paragraph, card_inner);

            // 3. Draw Footer Keymap Panel
            let footer_content = vec![
                Line::from(vec![
                    Span::styled(" [↑↓/WASD] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Span::styled("Select Game   ", Style::default().fg(Color::White)),
                    Span::styled(" [Enter/Space] ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                    Span::styled("Start Playing   ", Style::default().fg(Color::White)),
                    Span::styled(" [Esc/Q] ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                    Span::styled("Quit Arcade", Style::default().fg(Color::White)),
                ]),
            ];
            let footer_paragraph = Paragraph::new(footer_content)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(footer_paragraph, footer_area);
        }
    }
}
