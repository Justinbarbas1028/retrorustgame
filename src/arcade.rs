use std::time::Duration;
use ratatui::{
    Frame,
    layout::{Rect, Layout, Constraint, Direction, Alignment},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Paragraph, Wrap, Clear},
};
use crossterm::event::KeyCode;
use crate::high_scores::HighScores;
use crate::games::{
    Game, GameType, GameCommand,
    adom::AdomGame,
    adventure::AdventureGame,
    alienrl::AlienrlGame,
    allure_stars::AllureStarsGame,
    angband::AngbandGame,
    arithmetic::ArithmeticGame,
    ascii_patrol::AsciiPatrolGame,
    ascii_sector::AsciiSectorGame,
    atc::AtcGame,
    backgammon::BackgammonGame,
    bastet::BastetGame,
    battleship::BattleshipGame,
    battlestar::BattlestarGame,
    bcd::BcdGame,
    blackjack::BlackjackGame,
    boggle::BoggleGame,
    brogue::BrogueGame,
    caesar::CaesarGame,
    canfield::CanfieldGame,
    cataclysm::CataclysmGame,
    caves_of_qud::CavesOfQudGame,
    cfscores::CfscoresGame,
    checkers::CheckersGame,
    chess::ChessGame,
    cmatrix::CmatrixGame,
    countmail::CountmailGame,
    cribbage::CribbageGame,
    ctris::CtrisGame,
    dab::DabGame,
    dcss::DcssGame,
    doomrl::DoomrlGame,
    dwarf_fortress::DwarfFortressGame,
    end_of_eden::EndOfEdenGame,
    firewall::FirewallGame,
    freecell::FreecellGame,
    game_2048::Game2048,
    go_fish::GoFishGame,
    go_game::GoGameGame,
    gomoku::GomokuGame,
    gorched::GorchedGame,
    greed::GreedGame,
    hack::HackGame,
    hangman::HangmanGame,
    harmonist::HarmonistGame,
    hunt::HuntGame,
    larn::LarnGame,
    mastermind::MastermindGame,
    mazeventure::MazeventureGame,
    mille::MilleGame,
    minesweeper::MinesweeperGame,
    momodora::MomodoraGame,
    monopoly::MonopolyGame,
    moon_buggy::MoonBuggyGame,
    moria::MoriaGame,
    morse::MorseGame,
    nethack::NetHackGame,
    ninja::NinjaGame,
    number::NumberGame,
    omega::OmegaGame,
    othello::OthelloGame,
    pacman::PacmanGame,
    phantasia::PhantasiaGame,
    pig::PigGame,
    pipes::PipesGame,
    pokete::PoketeGame,
    pong::PongGame,
    primes::PrimesGame,
    progress95::ProgressCLI95Game,
    quiz::QuizGame,
    rain::RainGame,
    robots::RobotsGame,
    roguelike::RoguelikeGame,
    rot13::Rot13Game,
    rps::RpsGame,
    sail::SailGame,
    shoot_em::ShootEmGame,
    sil::SilGame,
    snake::SnakeGame,
    snscore::SnscoreGame,
    sokoban::SokobanGame,
    solitaire::SolitaireGame,
    space_invaders::SpaceInvadersGame,
    sudoku::SudokuGame,
    teachgammon::TeachgammonGame,
    tetris::TetrisGame,
    tetro::TetroGame,
    tggw::TggwGame,
    tint::TintGame,
    tome::TomeGame,
    trek::TrekGame,
    vitetris::VitetrisGame,
    wordle::WordleGame,
    worm::WormGame,
    wump::WumpGame,
    wumpus::WumpusGame,
    yahtzee::YahtzeeGame,
    zangband::ZangbandGame,
    zork::ZorkGame,
};

pub struct ArcadeConsole {
    categories: Vec<&'static str>,
    active_category_idx: usize,
    active_games: Vec<GameType>,
    selected_index: usize,
    scroll_offset: usize,
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
        let categories = vec![
            "Action & Arcade",
            "Roguelikes & RPGs",
            "Board & Card Games",
            "Text Adventures",
            "Brain & Speed Utilities",
        ];
        let active_category_idx = 0;
        let active_games = Self::get_games_for_category(categories[active_category_idx]);
        Self {
            categories,
            active_category_idx,
            active_games,
            selected_index: 0,
            scroll_offset: 0,
            active_game: None,
            active_game_type: None,
            high_scores: HighScores::load(),
        }
    }

    fn get_games_for_category(category: &str) -> Vec<GameType> {
        match category {
        "Action & Arcade" => vec![GameType::AsciiPatrol, GameType::Bastet, GameType::Cmatrix, GameType::Ctris, GameType::Firewall, GameType::Gorched, GameType::Momodora, GameType::MoonBuggy, GameType::Ninja, GameType::Pacman, GameType::Pipes, GameType::Pong, GameType::ProgressCLI95, GameType::Rain, GameType::ShootEm, GameType::Snake, GameType::SpaceInvaders, GameType::Tetris, GameType::Tetro, GameType::Tint, GameType::Vitetris, GameType::Worm],
        "Roguelikes & RPGs" => vec![GameType::Adom, GameType::Alienrl, GameType::AllureStars, GameType::Angband, GameType::AsciiSector, GameType::Atc, GameType::Brogue, GameType::Cataclysm, GameType::CavesOfQud, GameType::Dcss, GameType::Doomrl, GameType::DwarfFortress, GameType::Greed, GameType::Hack, GameType::Harmonist, GameType::Hunt, GameType::Larn, GameType::Mazeventure, GameType::Moria, GameType::Nethack, GameType::Omega, GameType::Pokete, GameType::Robots, GameType::Roguelike, GameType::Sail, GameType::Sil, GameType::Sokoban, GameType::Sudoku, GameType::Tggw, GameType::Tome, GameType::Trek, GameType::Wump, GameType::Wumpus, GameType::Zangband],
        "Board & Card Games" => vec![GameType::Backgammon, GameType::Battleship, GameType::Blackjack, GameType::Boggle, GameType::Canfield, GameType::Checkers, GameType::Chess, GameType::Cribbage, GameType::Dab, GameType::EndOfEden, GameType::Freecell, GameType::GoFish, GameType::GoGame, GameType::Gomoku, GameType::Mastermind, GameType::Mille, GameType::Monopoly, GameType::Othello, GameType::Pig, GameType::Rps, GameType::Solitaire, GameType::Teachgammon, GameType::Yahtzee],
        "Text Adventures" => vec![GameType::Adventure, GameType::Battlestar, GameType::Phantasia, GameType::Zork],
        "Brain & Speed Utilities" => vec![GameType::Arithmetic, GameType::Bcd, GameType::Caesar, GameType::Cfscores, GameType::Countmail, GameType::Game2048, GameType::Hangman, GameType::Minesweeper, GameType::Morse, GameType::Number, GameType::Primes, GameType::Quiz, GameType::Rot13, GameType::Snscore, GameType::Wordle],
            _ => vec![],
        }
    }

    fn create_game(game_type: GameType) -> Box<dyn Game> {
        match game_type {
            GameType::Adom => Box::new(AdomGame::new()),
            GameType::Adventure => Box::new(AdventureGame::new()),
            GameType::Alienrl => Box::new(AlienrlGame::new()),
            GameType::AllureStars => Box::new(AllureStarsGame::new()),
            GameType::Angband => Box::new(AngbandGame::new()),
            GameType::Arithmetic => Box::new(ArithmeticGame::new()),
            GameType::AsciiPatrol => Box::new(AsciiPatrolGame::new()),
            GameType::AsciiSector => Box::new(AsciiSectorGame::new()),
            GameType::Atc => Box::new(AtcGame::new()),
            GameType::Backgammon => Box::new(BackgammonGame::new()),
            GameType::Bastet => Box::new(BastetGame::new()),
            GameType::Battleship => Box::new(BattleshipGame::new()),
            GameType::Battlestar => Box::new(BattlestarGame::new()),
            GameType::Bcd => Box::new(BcdGame::new()),
            GameType::Blackjack => Box::new(BlackjackGame::new()),
            GameType::Boggle => Box::new(BoggleGame::new()),
            GameType::Brogue => Box::new(BrogueGame::new()),
            GameType::Caesar => Box::new(CaesarGame::new()),
            GameType::Canfield => Box::new(CanfieldGame::new()),
            GameType::Cataclysm => Box::new(CataclysmGame::new()),
            GameType::CavesOfQud => Box::new(CavesOfQudGame::new()),
            GameType::Cfscores => Box::new(CfscoresGame::new()),
            GameType::Checkers => Box::new(CheckersGame::new()),
            GameType::Chess => Box::new(ChessGame::new()),
            GameType::Cmatrix => Box::new(CmatrixGame::new()),
            GameType::Countmail => Box::new(CountmailGame::new()),
            GameType::Cribbage => Box::new(CribbageGame::new()),
            GameType::Ctris => Box::new(CtrisGame::new()),
            GameType::Dab => Box::new(DabGame::new()),
            GameType::Dcss => Box::new(DcssGame::new()),
            GameType::Doomrl => Box::new(DoomrlGame::new()),
            GameType::DwarfFortress => Box::new(DwarfFortressGame::new()),
            GameType::EndOfEden => Box::new(EndOfEdenGame::new()),
            GameType::Firewall => Box::new(FirewallGame::new()),
            GameType::Freecell => Box::new(FreecellGame::new()),
            GameType::Game2048 => Box::new(Game2048::new()),
            GameType::GoFish => Box::new(GoFishGame::new()),
            GameType::GoGame => Box::new(GoGameGame::new()),
            GameType::Gomoku => Box::new(GomokuGame::new()),
            GameType::Gorched => Box::new(GorchedGame::new()),
            GameType::Greed => Box::new(GreedGame::new()),
            GameType::Hack => Box::new(HackGame::new()),
            GameType::Hangman => Box::new(HangmanGame::new()),
            GameType::Harmonist => Box::new(HarmonistGame::new()),
            GameType::Hunt => Box::new(HuntGame::new()),
            GameType::Larn => Box::new(LarnGame::new()),
            GameType::Mastermind => Box::new(MastermindGame::new()),
            GameType::Mazeventure => Box::new(MazeventureGame::new()),
            GameType::Mille => Box::new(MilleGame::new()),
            GameType::Minesweeper => Box::new(MinesweeperGame::new()),
            GameType::Momodora => Box::new(MomodoraGame::new()),
            GameType::Monopoly => Box::new(MonopolyGame::new()),
            GameType::MoonBuggy => Box::new(MoonBuggyGame::new()),
            GameType::Moria => Box::new(MoriaGame::new()),
            GameType::Morse => Box::new(MorseGame::new()),
            GameType::Nethack => Box::new(NetHackGame::new()),
            GameType::Ninja => Box::new(NinjaGame::new()),
            GameType::Number => Box::new(NumberGame::new()),
            GameType::Omega => Box::new(OmegaGame::new()),
            GameType::Othello => Box::new(OthelloGame::new()),
            GameType::Pacman => Box::new(PacmanGame::new()),
            GameType::Phantasia => Box::new(PhantasiaGame::new()),
            GameType::Pig => Box::new(PigGame::new()),
            GameType::Pipes => Box::new(PipesGame::new()),
            GameType::Pokete => Box::new(PoketeGame::new()),
            GameType::Pong => Box::new(PongGame::new()),
            GameType::Primes => Box::new(PrimesGame::new()),
            GameType::ProgressCLI95 => Box::new(ProgressCLI95Game::new()),
            GameType::Quiz => Box::new(QuizGame::new()),
            GameType::Rain => Box::new(RainGame::new()),
            GameType::Robots => Box::new(RobotsGame::new()),
            GameType::Roguelike => Box::new(RoguelikeGame::new()),
            GameType::Rot13 => Box::new(Rot13Game::new()),
            GameType::Rps => Box::new(RpsGame::new()),
            GameType::Sail => Box::new(SailGame::new()),
            GameType::ShootEm => Box::new(ShootEmGame::new()),
            GameType::Sil => Box::new(SilGame::new()),
            GameType::Snake => Box::new(SnakeGame::new()),
            GameType::Snscore => Box::new(SnscoreGame::new()),
            GameType::Sokoban => Box::new(SokobanGame::new()),
            GameType::Solitaire => Box::new(SolitaireGame::new()),
            GameType::SpaceInvaders => Box::new(SpaceInvadersGame::new()),
            GameType::Sudoku => Box::new(SudokuGame::new()),
            GameType::Teachgammon => Box::new(TeachgammonGame::new()),
            GameType::Tetris => Box::new(TetrisGame::new()),
            GameType::Tetro => Box::new(TetroGame::new()),
            GameType::Tggw => Box::new(TggwGame::new()),
            GameType::Tint => Box::new(TintGame::new()),
            GameType::Tome => Box::new(TomeGame::new()),
            GameType::Trek => Box::new(TrekGame::new()),
            GameType::Vitetris => Box::new(VitetrisGame::new()),
            GameType::Wordle => Box::new(WordleGame::new()),
            GameType::Worm => Box::new(WormGame::new()),
            GameType::Wump => Box::new(WumpGame::new()),
            GameType::Wumpus => Box::new(WumpusGame::new()),
            GameType::Yahtzee => Box::new(YahtzeeGame::new()),
            GameType::Zangband => Box::new(ZangbandGame::new()),
            GameType::Zork => Box::new(ZorkGame::new()),
        }
    }

    fn get_game_preview(game_type: GameType) -> (Vec<&'static str>, &'static str, &'static str) {
        match game_type {
            GameType::Adom => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                   ADOM                  │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Adventure => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                ADVENTURE                │", "  └─────────────────────────────────────────┘"],
                "An immersive interactive text adventure! Wander rooms, interact with items, solve puzzles, and complete your quest.",
                "Select Choice: 1, 2, 3, 4  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Alienrl => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                 ALIENRL                 │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::AllureStars => (
                vec!["  ┌─────────────────────────────────────────┐", "  │               ALLURE STARS              │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Angband => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                 ANGBAND                 │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Arithmetic => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                ARITHMETIC               │", "  └─────────────────────────────────────────┘"],
                "A fast-paced interactive utility challenge. Train your brain, convert values, or solve ciphers against a ticking clock!",
                "Type Input: Number Keys  •  Delete: Backspace  •  Submit: Enter  •  Quit: Escape",
            ),
            GameType::AsciiPatrol => (
                vec!["  ┌─────────────────────────────────────────┐", "  │               ASCII PATROL              │", "  └─────────────────────────────────────────┘"],
                "Fast-paced real-time retro arcade action! Dodge obstacles, shoot projectiles, time your jumps, and survive to score big.",
                "Jump: W / Space / Up  •  Fire: F / Enter  •  Restart: R  •  Quit: Escape",
            ),
            GameType::AsciiSector => (
                vec!["  ┌─────────────────────────────────────────┐", "  │               ASCII SECTOR              │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Atc => (
                vec!["  ┌─────────────────────────────────────────┐", "  │          AIR TRAFFIC CONTROLLER         │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Backgammon => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                BACKGAMMON               │", "  └─────────────────────────────────────────┘"],
                "A classic strategy board or card game recreated in retro terminal style. Play your hands, place your bets, or out-maneuver the AI.",
                "Hit/Select: H / Enter  •  Stand/Pass: S / Space  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Bastet => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                  BASTET                 │", "  └─────────────────────────────────────────┘"],
                "Fast-paced real-time retro arcade action! Dodge obstacles, shoot projectiles, time your jumps, and survive to score big.",
                "Jump: W / Space / Up  •  Fire: F / Enter  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Battleship => (
                vec!["  ██████╗  █████╗ ████████╗████████╗██╗     ███████╗", "  ██╔══██╗██╔══██╗╚══██╔══╝╚══██╔══╝██║     ██╔════╝", "  ██████╔╝███████║   ██║      ██║   ██║     █████╗  ", "  ██╔══██╗██╔══██║   ██║      ██║   ██║     ██╔══╝  ", "  ██████╔╝██║  ██║   ██║      ██║   ███████╗███████╗"],
                "A classic grid-based tactical warfare game. Position your battleships on the 10x10 ocean grid and take turns with the AI targeting and firing salvos to sink the enemy fleet.",
                "Move Cursor: Arrows / WASD  •  Rotate Ship: R  •  Confirm/Fire: Enter/Space  •  Pause: P  •  Quit: Escape",
            ),
            GameType::Battlestar => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                BATTLESTAR               │", "  └─────────────────────────────────────────┘"],
                "An immersive interactive text adventure! Wander rooms, interact with items, solve puzzles, and complete your quest.",
                "Select Choice: 1, 2, 3, 4  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Bcd => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                BCD BINARY               │", "  └─────────────────────────────────────────┘"],
                "A fast-paced interactive utility challenge. Train your brain, convert values, or solve ciphers against a ticking clock!",
                "Type Input: Number Keys  •  Delete: Backspace  •  Submit: Enter  •  Quit: Escape",
            ),
            GameType::Blackjack => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                BLACKJACK                │", "  └─────────────────────────────────────────┘"],
                "A classic strategy board or card game recreated in retro terminal style. Play your hands, place your bets, or out-maneuver the AI.",
                "Hit/Select: H / Enter  •  Stand/Pass: S / Space  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Boggle => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                  BOGGLE                 │", "  └─────────────────────────────────────────┘"],
                "A classic strategy board or card game recreated in retro terminal style. Play your hands, place your bets, or out-maneuver the AI.",
                "Hit/Select: H / Enter  •  Stand/Pass: S / Space  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Brogue => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                  BROGUE                 │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Caesar => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                  CAESAR                 │", "  └─────────────────────────────────────────┘"],
                "A fast-paced interactive utility challenge. Train your brain, convert values, or solve ciphers against a ticking clock!",
                "Type Input: Number Keys  •  Delete: Backspace  •  Submit: Enter  •  Quit: Escape",
            ),
            GameType::Canfield => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                 CANFIELD                │", "  └─────────────────────────────────────────┘"],
                "A classic strategy board or card game recreated in retro terminal style. Play your hands, place your bets, or out-maneuver the AI.",
                "Hit/Select: H / Enter  •  Stand/Pass: S / Space  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Cataclysm => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                CATACLYSM                │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::CavesOfQud => (
                vec!["  ┌─────────────────────────────────────────┐", "  │               CAVES OF QUD              │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Cfscores => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                 CFSCORES                │", "  └─────────────────────────────────────────┘"],
                "A fast-paced interactive utility challenge. Train your brain, convert values, or solve ciphers against a ticking clock!",
                "Type Input: Number Keys  •  Delete: Backspace  •  Submit: Enter  •  Quit: Escape",
            ),
            GameType::Checkers => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                 CHECKERS                │", "  └─────────────────────────────────────────┘"],
                "A classic strategy board or card game recreated in retro terminal style. Play your hands, place your bets, or out-maneuver the AI.",
                "Hit/Select: H / Enter  •  Stand/Pass: S / Space  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Chess => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                  CHESS                  │", "  └─────────────────────────────────────────┘"],
                "A classic strategy board or card game recreated in retro terminal style. Play your hands, place your bets, or out-maneuver the AI.",
                "Hit/Select: H / Enter  •  Stand/Pass: S / Space  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Cmatrix => (
                vec!["  ┌─────────────────────────────────────────┐", "  │              CMATRIX HACKER             │", "  └─────────────────────────────────────────┘"],
                "Fast-paced real-time retro arcade action! Dodge obstacles, shoot projectiles, time your jumps, and survive to score big.",
                "Jump: W / Space / Up  •  Fire: F / Enter  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Countmail => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                COUNTMAIL                │", "  └─────────────────────────────────────────┘"],
                "A fast-paced interactive utility challenge. Train your brain, convert values, or solve ciphers against a ticking clock!",
                "Type Input: Number Keys  •  Delete: Backspace  •  Submit: Enter  •  Quit: Escape",
            ),
            GameType::Cribbage => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                 CRIBBAGE                │", "  └─────────────────────────────────────────┘"],
                "A classic strategy board or card game recreated in retro terminal style. Play your hands, place your bets, or out-maneuver the AI.",
                "Hit/Select: H / Enter  •  Stand/Pass: S / Space  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Ctris => (
                vec!["  ┌─────────────────────────────────────────┐", "  │            CTRIS COLOR MATCH            │", "  └─────────────────────────────────────────┘"],
                "Fast-paced real-time retro arcade action! Dodge obstacles, shoot projectiles, time your jumps, and survive to score big.",
                "Jump: W / Space / Up  •  Fire: F / Enter  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Dab => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                   DAB                   │", "  └─────────────────────────────────────────┘"],
                "A classic strategy board or card game recreated in retro terminal style. Play your hands, place your bets, or out-maneuver the AI.",
                "Hit/Select: H / Enter  •  Stand/Pass: S / Space  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Dcss => (
                vec!["  ┌─────────────────────────────────────────┐", "  │               DCSS DUNGEON              │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Doomrl => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                  DOOMRL                 │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::DwarfFortress => (
                vec!["  ┌─────────────────────────────────────────┐", "  │              DWARF FORTRESS             │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::EndOfEden => (
                vec!["  ┌─────────────────────────────────────────┐", "  │               END OF EDEN               │", "  └─────────────────────────────────────────┘"],
                "A classic strategy board or card game recreated in retro terminal style. Play your hands, place your bets, or out-maneuver the AI.",
                "Hit/Select: H / Enter  •  Stand/Pass: S / Space  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Firewall => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                 FIREWALL                │", "  └─────────────────────────────────────────┘"],
                "Fast-paced real-time retro arcade action! Dodge obstacles, shoot projectiles, time your jumps, and survive to score big.",
                "Jump: W / Space / Up  •  Fire: F / Enter  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Freecell => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                 FREECELL                │", "  └─────────────────────────────────────────┘"],
                "A classic strategy board or card game recreated in retro terminal style. Play your hands, place your bets, or out-maneuver the AI.",
                "Hit/Select: H / Enter  •  Stand/Pass: S / Space  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Game2048 => (
                vec!["  ██████╗  ██████╗  ██╗  ██╗  ██████╗ ", "  ╚════██╗██╔═══██╗██║  ██║██╔════██╗", "   █████╔╝██║   ██║███████║╚██████╔╝", "  ██╔═══╝ ██║   ██║╚════██║██╔═══██╗", "  ███████╗╚██████╔╝     ██║╚██████╔╝"],
                "Slide numeric tiles on a 4x4 grid. When two matching values touch, they combine into a double-value tile! Work your way up to make the prestigious 2048 block.",
                "Slide: Arrow Keys / WASD  •  Pause: P  •  Quit Arcade: Escape",
            ),
            GameType::GoFish => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                 GO FISH                 │", "  └─────────────────────────────────────────┘"],
                "A classic strategy board or card game recreated in retro terminal style. Play your hands, place your bets, or out-maneuver the AI.",
                "Hit/Select: H / Enter  •  Stand/Pass: S / Space  •  Restart: R  •  Quit: Escape",
            ),
            GameType::GoGame => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                 GO GAME                 │", "  └─────────────────────────────────────────┘"],
                "A classic strategy board or card game recreated in retro terminal style. Play your hands, place your bets, or out-maneuver the AI.",
                "Hit/Select: H / Enter  •  Stand/Pass: S / Space  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Gomoku => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                  GOMOKU                 │", "  └─────────────────────────────────────────┘"],
                "A classic strategy board or card game recreated in retro terminal style. Play your hands, place your bets, or out-maneuver the AI.",
                "Hit/Select: H / Enter  •  Stand/Pass: S / Space  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Gorched => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                 GORCHED                 │", "  └─────────────────────────────────────────┘"],
                "Fast-paced real-time retro arcade action! Dodge obstacles, shoot projectiles, time your jumps, and survive to score big.",
                "Jump: W / Space / Up  •  Fire: F / Enter  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Greed => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                  GREED                  │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Hack => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                   HACK                  │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Hangman => (
                vec!["  ██╗  ██╗ █████╗ ███╗   ██╗ ██████╗███╗   ███╗ █████╗ ███╗   ██╗", "  ██║  ██║██╔══██╗████╗  ██║██╔════╝████╗ ████║██╔══██╗████╗  ██║", "  ██║ █╗ ██║██║   ██║██████╔╝██║  ██║██║     █████╗  ", "  ██║███╗██║██║   ██║██╔══██╗██║  ██║██║     ██╔══╝  ", "  ╚███╔███╔╝╚██████╔╝██║  ██║██████╔╝███████╗███████╗"],
                "The timeless word-guessing challenge! Guess letters one by one to reveal the secret word. Each incorrect guess adds a piece to the classic ASCII gallows art.",
                "Guess Letter: A-Z  •  Pause: P  •  Quit Game: Escape",
            ),
            GameType::Harmonist => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                HARMONIST                │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Hunt => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                   HUNT                  │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Larn => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                   LARN                  │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Mastermind => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                MASTERMIND               │", "  └─────────────────────────────────────────┘"],
                "A classic strategy board or card game recreated in retro terminal style. Play your hands, place your bets, or out-maneuver the AI.",
                "Hit/Select: H / Enter  •  Stand/Pass: S / Space  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Mazeventure => (
                vec!["  ┌─────────────────────────────────────────┐", "  │               MAZEVENTURE               │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Mille => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                  MILLE                  │", "  └─────────────────────────────────────────┘"],
                "A classic strategy board or card game recreated in retro terminal style. Play your hands, place your bets, or out-maneuver the AI.",
                "Hit/Select: H / Enter  •  Stand/Pass: S / Space  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Minesweeper => (
                vec!["  ███╗   ███╗██╗███╗   ██╗███████╗", "  ████╗ ████║██║████╗  ██║██╔════╝", "  ██╔████╔██║██║██╔██╗ ██║█████╗  ", "  ██║╚██╔╝██║██║██║╚██╗██║██╔══╝  ", "  ██║ ╚═╝ ██║██║██║ ╚████║███████╗"],
                "A pure logic deduction puzzle. Clean the safe grid cells without setting off any hidden mines. Use flags to pinpoint mines and clear pockets with cascading opens.",
                "Move Cursor: Arrow Keys / WASD  •  Dig Cell: Space / Enter  •  Toggle Flag: F  •  Pause: P",
            ),
            GameType::Momodora => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                 MOMODORA                │", "  └─────────────────────────────────────────┘"],
                "Fast-paced real-time retro arcade action! Dodge obstacles, shoot projectiles, time your jumps, and survive to score big.",
                "Jump: W / Space / Up  •  Fire: F / Enter  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Monopoly => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                 MONOPOLY                │", "  └─────────────────────────────────────────┘"],
                "A classic strategy board or card game recreated in retro terminal style. Play your hands, place your bets, or out-maneuver the AI.",
                "Hit/Select: H / Enter  •  Stand/Pass: S / Space  •  Restart: R  •  Quit: Escape",
            ),
            GameType::MoonBuggy => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                MOON BUGGY               │", "  └─────────────────────────────────────────┘"],
                "Fast-paced real-time retro arcade action! Dodge obstacles, shoot projectiles, time your jumps, and survive to score big.",
                "Jump: W / Space / Up  •  Fire: F / Enter  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Moria => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                  MORIA                  │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Morse => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                  MORSE                  │", "  └─────────────────────────────────────────┘"],
                "A fast-paced interactive utility challenge. Train your brain, convert values, or solve ciphers against a ticking clock!",
                "Type Input: Number Keys  •  Delete: Backspace  •  Submit: Enter  •  Quit: Escape",
            ),
            GameType::Nethack => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                 NETHACK                 │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Ninja => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                  NINJA                  │", "  └─────────────────────────────────────────┘"],
                "Fast-paced real-time retro arcade action! Dodge obstacles, shoot projectiles, time your jumps, and survive to score big.",
                "Jump: W / Space / Up  •  Fire: F / Enter  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Number => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                  NUMBER                 │", "  └─────────────────────────────────────────┘"],
                "A fast-paced interactive utility challenge. Train your brain, convert values, or solve ciphers against a ticking clock!",
                "Type Input: Number Keys  •  Delete: Backspace  •  Submit: Enter  •  Quit: Escape",
            ),
            GameType::Omega => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                  OMEGA                  │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Othello => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                 OTHELLO                 │", "  └─────────────────────────────────────────┘"],
                "A classic strategy board or card game recreated in retro terminal style. Play your hands, place your bets, or out-maneuver the AI.",
                "Hit/Select: H / Enter  •  Stand/Pass: S / Space  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Pacman => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                  PACMAN                 │", "  └─────────────────────────────────────────┘"],
                "Fast-paced real-time retro arcade action! Dodge obstacles, shoot projectiles, time your jumps, and survive to score big.",
                "Jump: W / Space / Up  •  Fire: F / Enter  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Phantasia => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                PHANTASIA                │", "  └─────────────────────────────────────────┘"],
                "An immersive interactive text adventure! Wander rooms, interact with items, solve puzzles, and complete your quest.",
                "Select Choice: 1, 2, 3, 4  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Pig => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                   PIG                   │", "  └─────────────────────────────────────────┘"],
                "A classic strategy board or card game recreated in retro terminal style. Play your hands, place your bets, or out-maneuver the AI.",
                "Hit/Select: H / Enter  •  Stand/Pass: S / Space  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Pipes => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                  PIPES                  │", "  └─────────────────────────────────────────┘"],
                "Fast-paced real-time retro arcade action! Dodge obstacles, shoot projectiles, time your jumps, and survive to score big.",
                "Jump: W / Space / Up  •  Fire: F / Enter  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Pokete => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                  POKETE                 │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Pong => (
                vec!["  ██████╗  ██████╗ ███╗   ██╗ ██████╗ ", "  ██╔══██╗██╔═══██╗████╗  ██║██╔════╝ ", "  ██████╔╝██║   ██║██╔██╗ ██║██║  ███╗", "  ██╔═══╝ ██║   ██║██║╚██╗██║██║   ██║", "  ██║     ╚██████╔╝██║ ╚████║╚██████╔╝"],
                "The grandfather of video games, rebuilt in full glory! Control your left paddle in real-time to deflect the ball past the computer opponent. Features smooth ball spin physics.",
                "Move Paddle: Up/Down Arrows / W/S  •  Pause: P  •  Quit Game: Escape",
            ),
            GameType::Primes => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                  PRIMES                 │", "  └─────────────────────────────────────────┘"],
                "A fast-paced interactive utility challenge. Train your brain, convert values, or solve ciphers against a ticking clock!",
                "Type Input: Number Keys  •  Delete: Backspace  •  Submit: Enter  •  Quit: Escape",
            ),
            GameType::ProgressCLI95 => (
                vec!["  ┌─────────────────────────────────────────┐", "  │              PROGRESSCLI95              │", "  └─────────────────────────────────────────┘"],
                "Fast-paced real-time retro arcade action! Dodge obstacles, shoot projectiles, time your jumps, and survive to score big.",
                "Jump: W / Space / Up  •  Fire: F / Enter  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Quiz => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                   QUIZ                  │", "  └─────────────────────────────────────────┘"],
                "A fast-paced interactive utility challenge. Train your brain, convert values, or solve ciphers against a ticking clock!",
                "Type Input: Number Keys  •  Delete: Backspace  •  Submit: Enter  •  Quit: Escape",
            ),
            GameType::Rain => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                   RAIN                  │", "  └─────────────────────────────────────────┘"],
                "Fast-paced real-time retro arcade action! Dodge obstacles, shoot projectiles, time your jumps, and survive to score big.",
                "Jump: W / Space / Up  •  Fire: F / Enter  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Robots => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                  ROBOTS                 │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Roguelike => (
                vec!["  ██████╗  ██████╗  ██████╗ ██╗   ██╗███████╗", "  ██╔══██╗██╔═══██╗██╔════╝ ██║   ██║██╔════╝", "  ██████╔╝██║   ██║██║  ███╗██║   ██║█████╗  ", "  ██╔══██╗██║   ██║██║   ██║██║   ██║██╔══╝  ", "  ██║  ██║╚██████╔╝╚██████╔╝╚██████╔╝███████╗"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find swords/shields, fight goblins/orcs/trolls, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Skip Turn: Space / Period  •  Pause: P",
            ),
            GameType::Rot13 => (
                vec!["  ┌─────────────────────────────────────────┐", "  │              ROT13 DECIPHER             │", "  └─────────────────────────────────────────┘"],
                "A fast-paced interactive utility challenge. Train your brain, convert values, or solve ciphers against a ticking clock!",
                "Type Input: Number Keys  •  Delete: Backspace  •  Submit: Enter  •  Quit: Escape",
            ),
            GameType::Rps => (
                vec!["  ┌─────────────────────────────────────────┐", "  │              RPS TOURNAMENT             │", "  └─────────────────────────────────────────┘"],
                "A classic strategy board or card game recreated in retro terminal style. Play your hands, place your bets, or out-maneuver the AI.",
                "Hit/Select: H / Enter  •  Stand/Pass: S / Space  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Sail => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                   SAIL                  │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::ShootEm => (
                vec!["  ┌─────────────────────────────────────────┐", "  │             SHOOT 'EM BATCH             │", "  └─────────────────────────────────────────┘"],
                "Fast-paced real-time retro arcade action! Dodge obstacles, shoot projectiles, time your jumps, and survive to score big.",
                "Jump: W / Space / Up  •  Fire: F / Enter  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Sil => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                   SIL                   │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Snake => (
                vec!["  ██████╗███╗   ██╗ █████╗ ██╗  ██╗███████╗", "  ██╔════╝████╗  ██║██╔══██╗██║  ██║██╔════╝", "  ███████╗██╔██╗ ██║███████║███████║█████╗  ", "  ╚════██║██║╚██╗██║██╔══██║██╔══██║██╔══╝  ", "  ███████║██║ ╚████║██║  ██║██║  ██║███████╗"],
                "A classic real-time retro arcade game. Control the snake to eat food and grow longer, but avoid crashing into walls or yourself! Includes dynamic speed scaling.",
                "Move: Arrow Keys / WASD  •  Pause: P  •  Quit Game: Escape",
            ),
            GameType::Snscore => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                 SNSCORE                 │", "  └─────────────────────────────────────────┘"],
                "A fast-paced interactive utility challenge. Train your brain, convert values, or solve ciphers against a ticking clock!",
                "Type Input: Number Keys  •  Delete: Backspace  •  Submit: Enter  •  Quit: Escape",
            ),
            GameType::Sokoban => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                 SOKOBAN                 │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Solitaire => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                SOLITAIRE                │", "  └─────────────────────────────────────────┘"],
                "A classic strategy board or card game recreated in retro terminal style. Play your hands, place your bets, or out-maneuver the AI.",
                "Hit/Select: H / Enter  •  Stand/Pass: S / Space  •  Restart: R  •  Quit: Escape",
            ),
            GameType::SpaceInvaders => (
                vec!["  ███████╗██████╗  █████╗  ██████╗███████╗", "  ██╔════╝██╔══██╗██╔══██╗██╔════╝██╔════╝", "  ███████╗██████╔╝███████║██║     █████╗  ", "  ╚════██║██╔═══╝ ██╔══██║██║     ██╔══╝  ", "  ███████║██║     ██║  ██║╚██████╗███████╗"],
                "Retro real-time action shooter! Defend your base ship from swarms of descending invaders. Hide behind bunkers, fire lasers, and survive waves of incoming threats.",
                "Move Ship: Left/Right Arrow or A/D  •  Shoot Laser: Space / Up Arrow / W  •  Pause: P",
            ),
            GameType::Sudoku => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                  SUDOKU                 │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Teachgammon => (
                vec!["  ┌─────────────────────────────────────────┐", "  │               TEACHGAMMON               │", "  └─────────────────────────────────────────┘"],
                "A classic strategy board or card game recreated in retro terminal style. Play your hands, place your bets, or out-maneuver the AI.",
                "Hit/Select: H / Enter  •  Stand/Pass: S / Space  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Tetris => (
                vec!["  ██████╗████████╗██████╗ ██╗███████╗", "  ╚══██╔══╝╚══██╔══╝██╔══██╗██║██╔════╝", "     ██║      ██║   ██████╔╝██║███████╗", "     ██║      ██║   ██╔══██╗██║╚════██║", "     ██║      ██║   ██║  ██║██║███████║"],
                "A classic real-time tile match game! Fit falling tetrominoes together to clear complete horizontal lines. Features increasing levels, speeds, and block previews.",
                "Move: Left/Right Arrow  •  Rotate: Up Arrow  •  Soft Drop: Down Arrow  •  Hard Drop: Space  •  Hold: C / Shift  •  Pause: P",
            ),
            GameType::Tetro => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                  TETRO                  │", "  └─────────────────────────────────────────┘"],
                "Fast-paced real-time retro arcade action! Dodge obstacles, shoot projectiles, time your jumps, and survive to score big.",
                "Jump: W / Space / Up  •  Fire: F / Enter  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Tggw => (
                vec!["  ┌─────────────────────────────────────────┐", "  │           THE GROUND GIVES WAY          │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Tint => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                   TINT                  │", "  └─────────────────────────────────────────┘"],
                "Fast-paced real-time retro arcade action! Dodge obstacles, shoot projectiles, time your jumps, and survive to score big.",
                "Jump: W / Space / Up  •  Fire: F / Enter  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Tome => (
                vec!["  ┌─────────────────────────────────────────┐", "  │            TALES OF MAJ'EYAL            │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Trek => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                   TREK                  │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Vitetris => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                 VITETRIS                │", "  └─────────────────────────────────────────┘"],
                "Fast-paced real-time retro arcade action! Dodge obstacles, shoot projectiles, time your jumps, and survive to score big.",
                "Jump: W / Space / Up  •  Fire: F / Enter  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Wordle => (
                vec!["  ██╗    ██╗ ██████╗ ██████╗ ██████╗ ██╗     ███████╗", "  ██║    ██║██╔═══██╗██╔══██╗██╔══██╗██║     ██╔════╝", "  ██║ █╗ ██║██║   ██║██████╔╝██║  ██║██║     █████╗  ", "  ██║███╗██║██║   ██║██╔══██╗██║  ██║██║     ██╔══╝  ", "  ╚███╔███╔╝╚██████╔╝██║  ██║██████╔╝███████╗███████╗"],
                "Challenge your vocabulary with this 5-letter word game! You have 6 attempts to guess the secret word. Features colorful elimination keyboard grids and letter status indicators.",
                "Type Letters: A-Z  •  Delete: Backspace  •  Submit Word: Enter  •  Pause: P  •  Quit Game: Escape",
            ),
            GameType::Worm => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                   WORM                  │", "  └─────────────────────────────────────────┘"],
                "Fast-paced real-time retro arcade action! Dodge obstacles, shoot projectiles, time your jumps, and survive to score big.",
                "Jump: W / Space / Up  •  Fire: F / Enter  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Wump => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                   WUMP                  │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Wumpus => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                  WUMPUS                 │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Yahtzee => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                 YAHTZEE                 │", "  └─────────────────────────────────────────┘"],
                "A classic strategy board or card game recreated in retro terminal style. Play your hands, place your bets, or out-maneuver the AI.",
                "Hit/Select: H / Enter  •  Stand/Pass: S / Space  •  Restart: R  •  Quit: Escape",
            ),
            GameType::Zangband => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                 ZANGBAND                │", "  └─────────────────────────────────────────┘"],
                "A turn-based mini-RPG dungeon crawler. Wander randomized rooms, gather health potions, find weapons, fight monsters, and go deep down the stairs.",
                "Move/Attack: Arrow Keys / WASD  •  Restart: R  •  Quit Game: Escape",
            ),
            GameType::Zork => (
                vec!["  ┌─────────────────────────────────────────┐", "  │                   ZORK                  │", "  └─────────────────────────────────────────┘"],
                "An immersive interactive text adventure! Wander rooms, interact with items, solve puzzles, and complete your quest.",
                "Select Choice: 1, 2, 3, 4  •  Restart: R  •  Quit Game: Escape",
            ),
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
                // Tab category switching: Left / Right / A / D
                KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
                    if self.active_category_idx > 0 {
                        self.active_category_idx -= 1;
                    } else {
                        self.active_category_idx = self.categories.len() - 1;
                    }
                    self.active_games = Self::get_games_for_category(self.categories[self.active_category_idx]);
                    self.selected_index = 0;
                    self.scroll_offset = 0;
                }
                KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
                    if self.active_category_idx < self.categories.len() - 1 {
                        self.active_category_idx += 1;
                    } else {
                        self.active_category_idx = 0;
                    }
                    self.active_games = Self::get_games_for_category(self.categories[self.active_category_idx]);
                    self.selected_index = 0;
                    self.scroll_offset = 0;
                }
                // Game item selection: Up / Down / W / S
                KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                    if !self.active_games.is_empty() {
                        if self.selected_index > 0 {
                            self.selected_index -= 1;
                        } else {
                            self.selected_index = self.active_games.len() - 1;
                        }
                        
                        // Adjust scroll offset
                        if self.selected_index < self.scroll_offset {
                            self.scroll_offset = self.selected_index;
                        } else if self.selected_index >= self.scroll_offset + 10 {
                            self.scroll_offset = self.selected_index - 9;
                        }
                    }
                }
                KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                    if !self.active_games.is_empty() {
                        if self.selected_index < self.active_games.len() - 1 {
                            self.selected_index += 1;
                        } else {
                            self.selected_index = 0;
                        }

                        // Adjust scroll offset
                        if self.selected_index >= self.scroll_offset + 10 {
                            self.scroll_offset = self.selected_index - 9;
                        } else if self.selected_index < self.scroll_offset {
                            self.scroll_offset = self.selected_index;
                        }
                    }
                }
                KeyCode::Enter | KeyCode::Char(' ') => {
                    if !self.active_games.is_empty() {
                        let selected = self.active_games[self.selected_index];
                        self.active_game = Some(Self::create_game(selected));
                        self.active_game_type = Some(selected);
                    }
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
                .title(" RUST RETRO ARCADE 100-GAME mega CABINET ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
            
            frame.render_widget(outer_block, area);

            let inner_area = area.inner(&ratatui::layout::Margin { horizontal: 2, vertical: 1 });

            // Layout split: Header, Tabs, Center, Footer
            let main_layouts = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(4), // Glowing neon retro header
                    Constraint::Length(3), // Tabs category bar
                    Constraint::Min(12),   // Game list & Preview panel
                    Constraint::Length(3), // Footer buttons
                ])
                .split(inner_area);

            let header_area = main_layouts[0];
            let tabs_area = main_layouts[1];
            let center_area = main_layouts[2];
            let footer_area = main_layouts[3];

            // 1. Draw glowing header
            let header_content = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled(" 👾 🕹️  1 0 0   G A M E S   R E T R O   A R C A D E   M E G A   C A B I N E T  🕹️ 👾 ", 
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                    ),
                ]),
            ];
            let header_paragraph = Paragraph::new(header_content).alignment(Alignment::Center);
            frame.render_widget(header_paragraph, header_area);

            // 2. Draw Category Tabs Bar
            let mut tab_spans = Vec::new();
            tab_spans.push(Span::styled("   ", Style::default()));
            for (idx, cat) in self.categories.iter().enumerate() {
                let is_active = idx == self.active_category_idx;
                if is_active {
                    tab_spans.push(Span::styled(format!(" [ {cat} ] "), Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)));
                } else {
                    tab_spans.push(Span::styled(format!("   {cat}   "), Style::default().fg(Color::DarkGray)));
                }
                tab_spans.push(Span::styled("  │  ", Style::default().fg(Color::Rgb(50, 50, 50))));
            }
            if !tab_spans.is_empty() {
                tab_spans.pop(); // Remove trailing separator
            }
            
            let tabs_paragraph = Paragraph::new(Line::from(tab_spans))
                .block(Block::default().borders(Borders::BOTTOM).border_style(Style::default().fg(Color::Rgb(50, 50, 50))));
            frame.render_widget(tabs_paragraph, tabs_area);

            // 3. Draw Center Split: Left = Menu list, Right = Game detail card
            let center_split = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(40), // Left List
                    Constraint::Percentage(60), // Right Card
                ])
                .split(center_area);

            let list_area = center_split[0];
            let card_area = center_split[1];

            // 3a. Draw Game List Panel
            let list_block = Block::default()
                .title(format!(" GAMES IN {} ", self.categories[self.active_category_idx].to_uppercase()))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::White));
            
            let list_inner = list_block.inner(list_area);
            frame.render_widget(list_block, list_area);

            let mut game_rows = Vec::new();
            
            // Viewport pagination
            let viewport_size = 10;
            let display_games = &self.active_games[self.scroll_offset..std::cmp::min(self.scroll_offset + viewport_size, self.active_games.len())];

            if self.scroll_offset > 0 {
                game_rows.push(Line::from(Span::styled("        ▲ (more above)", Style::default().fg(Color::Yellow))));
            } else {
                game_rows.push(Line::from(""));
            }

            for (idx_in_viewport, game) in display_games.iter().enumerate() {
                let actual_idx = self.scroll_offset + idx_in_viewport;
                let is_selected = actual_idx == self.selected_index;
                let mut spans = Vec::new();

                if is_selected {
                    spans.push(Span::styled(" ▶  ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
                    spans.push(Span::styled(
                        format!("{:<25}", game.name()),
                        Style::default().fg(Color::Yellow).bg(Color::Rgb(30, 30, 0)).add_modifier(Modifier::BOLD)
                    ));
                } else {
                    spans.push(Span::styled("    ", Style::default().fg(Color::DarkGray)));
                    spans.push(Span::styled(
                        format!("{:<25}", game.name()),
                        Style::default().fg(Color::Gray)
                    ));
                }

                game_rows.push(Line::from(spans));
            }

            if self.scroll_offset + viewport_size < self.active_games.len() {
                game_rows.push(Line::from(Span::styled("        ▼ (more below)", Style::default().fg(Color::Yellow))));
            } else {
                game_rows.push(Line::from(""));
            }

            let list_paragraph = Paragraph::new(game_rows);
            frame.render_widget(list_paragraph, list_inner);

            // 3b. Draw Preview Card details panel
            if !self.active_games.is_empty() {
                let selected_game = self.active_games[self.selected_index];
                let high_score = self.high_scores.get_score(selected_game.name());

                let card_block = Block::default()
                    .title(" GAME PREVIEW ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Cyan));
                
                let card_inner = card_block.inner(card_area);
                frame.render_widget(card_block, card_area);

                // Detailed descriptions and stylized logo art
                let (logo_art, desc, controls) = Self::get_game_preview(selected_game);

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
            }

            // 4. Draw Footer Keymap Panel
            let footer_content = vec![
                Line::from(vec![
                    Span::styled(" [◀▶/AD] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Span::styled("Switch Category   ", Style::default().fg(Color::White)),
                    Span::styled(" [▲▼/WS] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Span::styled("Select Game   ", Style::default().fg(Color::White)),
                    Span::styled(" [Enter/Space] ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                    Span::styled("Start   ", Style::default().fg(Color::White)),
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
