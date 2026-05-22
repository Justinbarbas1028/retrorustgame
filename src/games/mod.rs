pub mod tetris;
pub mod game_2048;
pub mod minesweeper;
pub mod space_invaders;
pub mod roguelike;
pub mod snake;
pub mod wordle;
pub mod battleship;
pub mod hangman;
pub mod pong;

use std::time::Duration;
use ratatui::Frame;
use ratatui::layout::Rect;
use crossterm::event::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameType {
    Tetris,
    Game2048,
    Minesweeper,
    SpaceInvaders,
    Roguelike,
    Snake,
    Wordle,
    Battleship,
    Hangman,
    Pong,
}

impl GameType {
    pub fn name(&self) -> &'static str {
        match self {
            GameType::Tetris => "Tetris",
            GameType::Game2048 => "2048",
            GameType::Minesweeper => "Minesweeper",
            GameType::SpaceInvaders => "Space Invaders",
            GameType::Roguelike => "Roguelike Crawler",
            GameType::Snake => "Snake",
            GameType::Wordle => "Wordle",
            GameType::Battleship => "Battleship",
            GameType::Hangman => "Hangman",
            GameType::Pong => "Pong",
        }
    }
}

pub enum GameCommand {
    None,
    Exit,
    Restart,
}

pub trait Game {
    fn update(&mut self, delta: Duration);
    fn handle_input(&mut self, key: KeyCode) -> GameCommand;
    fn draw(&self, frame: &mut Frame, area: Rect);
    fn get_score(&self) -> u32;
    fn is_game_over(&self) -> bool;
}
