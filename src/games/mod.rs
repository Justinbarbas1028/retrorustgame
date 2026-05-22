pub mod adom;
pub mod adventure;
pub mod alienrl;
pub mod allure_stars;
pub mod angband;
pub mod arithmetic;
pub mod ascii_patrol;
pub mod ascii_sector;
pub mod atc;
pub mod backgammon;
pub mod bastet;
pub mod battleship;
pub mod battlestar;
pub mod bcd;
pub mod blackjack;
pub mod boggle;
pub mod brogue;
pub mod caesar;
pub mod canfield;
pub mod cataclysm;
pub mod caves_of_qud;
pub mod cfscores;
pub mod checkers;
pub mod chess;
pub mod cmatrix;
pub mod countmail;
pub mod cribbage;
pub mod ctris;
pub mod dab;
pub mod dcss;
pub mod doomrl;
pub mod dwarf_fortress;
pub mod end_of_eden;
pub mod firewall;
pub mod freecell;
pub mod game_2048;
pub mod go_fish;
pub mod go_game;
pub mod gomoku;
pub mod gorched;
pub mod greed;
pub mod hack;
pub mod hangman;
pub mod harmonist;
pub mod hunt;
pub mod larn;
pub mod mastermind;
pub mod mazeventure;
pub mod mille;
pub mod minesweeper;
pub mod momodora;
pub mod monopoly;
pub mod moon_buggy;
pub mod moria;
pub mod morse;
pub mod nethack;
pub mod ninja;
pub mod number;
pub mod omega;
pub mod othello;
pub mod pacman;
pub mod phantasia;
pub mod pig;
pub mod pipes;
pub mod pokete;
pub mod pong;
pub mod primes;
pub mod progress95;
pub mod quiz;
pub mod rain;
pub mod robots;
pub mod roguelike;
pub mod rot13;
pub mod rps;
pub mod sail;
pub mod shoot_em;
pub mod sil;
pub mod snake;
pub mod snscore;
pub mod sokoban;
pub mod solitaire;
pub mod space_invaders;
pub mod sudoku;
pub mod teachgammon;
pub mod tetris;
pub mod tetro;
pub mod tggw;
pub mod tint;
pub mod tome;
pub mod trek;
pub mod vitetris;
pub mod wordle;
pub mod worm;
pub mod wump;
pub mod wumpus;
pub mod yahtzee;
pub mod zangband;
pub mod zork;

use std::time::Duration;
use ratatui::Frame;
use ratatui::layout::Rect;
use crossterm::event::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameType {
    Adom,
    Adventure,
    Alienrl,
    AllureStars,
    Angband,
    Arithmetic,
    AsciiPatrol,
    AsciiSector,
    Atc,
    Backgammon,
    Bastet,
    Battleship,
    Battlestar,
    Bcd,
    Blackjack,
    Boggle,
    Brogue,
    Caesar,
    Canfield,
    Cataclysm,
    CavesOfQud,
    Cfscores,
    Checkers,
    Chess,
    Cmatrix,
    Countmail,
    Cribbage,
    Ctris,
    Dab,
    Dcss,
    Doomrl,
    DwarfFortress,
    EndOfEden,
    Firewall,
    Freecell,
    Game2048,
    GoFish,
    GoGame,
    Gomoku,
    Gorched,
    Greed,
    Hack,
    Hangman,
    Harmonist,
    Hunt,
    Larn,
    Mastermind,
    Mazeventure,
    Mille,
    Minesweeper,
    Momodora,
    Monopoly,
    MoonBuggy,
    Moria,
    Morse,
    Nethack,
    Ninja,
    Number,
    Omega,
    Othello,
    Pacman,
    Phantasia,
    Pig,
    Pipes,
    Pokete,
    Pong,
    Primes,
    ProgressCLI95,
    Quiz,
    Rain,
    Robots,
    Roguelike,
    Rot13,
    Rps,
    Sail,
    ShootEm,
    Sil,
    Snake,
    Snscore,
    Sokoban,
    Solitaire,
    SpaceInvaders,
    Sudoku,
    Teachgammon,
    Tetris,
    Tetro,
    Tggw,
    Tint,
    Tome,
    Trek,
    Vitetris,
    Wordle,
    Worm,
    Wump,
    Wumpus,
    Yahtzee,
    Zangband,
    Zork,
}

impl GameType {
    pub fn name(&self) -> &'static str {
        match self {
            GameType::Adom => "Adom",
            GameType::Adventure => "Adventure",
            GameType::Alienrl => "Alienrl",
            GameType::AllureStars => "Allure Stars",
            GameType::Angband => "Angband",
            GameType::Arithmetic => "Arithmetic",
            GameType::AsciiPatrol => "Ascii Patrol",
            GameType::AsciiSector => "Ascii Sector",
            GameType::Atc => "Air Traffic Controller",
            GameType::Backgammon => "Backgammon",
            GameType::Bastet => "Bastet",
            GameType::Battleship => "Battleship",
            GameType::Battlestar => "Battlestar",
            GameType::Bcd => "BCD Binary",
            GameType::Blackjack => "Blackjack",
            GameType::Boggle => "Boggle",
            GameType::Brogue => "Brogue",
            GameType::Caesar => "Caesar",
            GameType::Canfield => "Canfield",
            GameType::Cataclysm => "Cataclysm",
            GameType::CavesOfQud => "Caves Of Qud",
            GameType::Cfscores => "Cfscores",
            GameType::Checkers => "Checkers",
            GameType::Chess => "Chess",
            GameType::Cmatrix => "CMatrix Hacker",
            GameType::Countmail => "Countmail",
            GameType::Cribbage => "Cribbage",
            GameType::Ctris => "CTris Color Match",
            GameType::Dab => "Dab",
            GameType::Dcss => "DCSS Dungeon",
            GameType::Doomrl => "Doomrl",
            GameType::DwarfFortress => "Dwarf Fortress",
            GameType::EndOfEden => "End Of Eden",
            GameType::Firewall => "Firewall",
            GameType::Freecell => "Freecell",
            GameType::Game2048 => "2048",
            GameType::GoFish => "Go Fish",
            GameType::GoGame => "Go Game",
            GameType::Gomoku => "Gomoku",
            GameType::Gorched => "Gorched",
            GameType::Greed => "Greed",
            GameType::Hack => "Hack",
            GameType::Hangman => "Hangman",
            GameType::Harmonist => "Harmonist",
            GameType::Hunt => "Hunt",
            GameType::Larn => "Larn",
            GameType::Mastermind => "Mastermind",
            GameType::Mazeventure => "Mazeventure",
            GameType::Mille => "Mille",
            GameType::Minesweeper => "Minesweeper",
            GameType::Momodora => "Momodora",
            GameType::Monopoly => "Monopoly",
            GameType::MoonBuggy => "Moon Buggy",
            GameType::Moria => "Moria",
            GameType::Morse => "Morse",
            GameType::Nethack => "Nethack",
            GameType::Ninja => "Ninja",
            GameType::Number => "Number",
            GameType::Omega => "Omega",
            GameType::Othello => "Othello",
            GameType::Pacman => "Pacman",
            GameType::Phantasia => "Phantasia",
            GameType::Pig => "Pig",
            GameType::Pipes => "Pipes",
            GameType::Pokete => "Pokete",
            GameType::Pong => "Pong",
            GameType::Primes => "Primes",
            GameType::ProgressCLI95 => "ProgressCLI95",
            GameType::Quiz => "Quiz",
            GameType::Rain => "Rain",
            GameType::Robots => "Robots",
            GameType::Roguelike => "Roguelike",
            GameType::Rot13 => "ROT13 decipher",
            GameType::Rps => "RPS Tournament",
            GameType::Sail => "Sail",
            GameType::ShootEm => "Shoot 'Em Batch",
            GameType::Sil => "Sil",
            GameType::Snake => "Snake",
            GameType::Snscore => "Snscore",
            GameType::Sokoban => "Sokoban",
            GameType::Solitaire => "Solitaire",
            GameType::SpaceInvaders => "Space Invaders",
            GameType::Sudoku => "Sudoku",
            GameType::Teachgammon => "Teachgammon",
            GameType::Tetris => "Tetris",
            GameType::Tetro => "Tetro",
            GameType::Tggw => "The Ground Gives Way",
            GameType::Tint => "Tint",
            GameType::Tome => "Tales of Maj'Eyal",
            GameType::Trek => "Trek",
            GameType::Vitetris => "Vitetris",
            GameType::Wordle => "Wordle",
            GameType::Worm => "Worm",
            GameType::Wump => "Wump",
            GameType::Wumpus => "Wumpus",
            GameType::Yahtzee => "Yahtzee",
            GameType::Zangband => "Zangband",
            GameType::Zork => "Zork",
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
