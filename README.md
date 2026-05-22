# 🕹️ Rust Terminal Retro Arcade Console (100-Game Mega Cabinet)

An extremely polished, premium retro arcade dashboard containing **100 classic terminal games** in a single, lightweight binary. Built entirely in idiomatic Rust using `ratatui` for double-buffered, flicker-free rendering, and `crossterm` for cross-platform terminal control and non-blocking real-time keyboard navigation.

The menu launcher features a premium category-based tab navigation system, smooth horizontal switching, viewport pagination, and dynamic high-fidelity preview panels displaying gorgeous custom ASCII art logo cards for flagship games, details, personal high scores, and instructions.

---

## 📂 Game Categories

The mega cabinet organizes 100 games into 5 cohesive categories:

### 1. 🕹️ Action & Arcade
* **🧱 Tetris**: Classic real-time tile matching! Fit falling tetrominoes together to clear complete horizontal lines. Features holding slots, next block previews, level speeds, and ghost piece shadows.
* **👽 Space Invaders**: Real-time arcade shooter! Defend your shield bunkers, glide your defender ship, and blast descending swarms of invaders.
* **🐍 Snake**: Control a neon-green snake to consume glowing red food. Grows longer and speeds up as you eat.
* **Tron**: Real-time light-cycle duels with persistent energy trails and accelerating survival pressure.
* **👾 Moon Buggy, AsciiPatrol, Bastet, Pipes, CMatrix, and more action titles!**

### 2. 🗡️ Roguelikes & RPGs
* ** Rogue / Roguelike**: Turn-based RPG adventure. Wander procedurally generated rooms, pick up weapons and shields, drink potions, slay trolls, level up stats, and descend down the stairs.
* **🛡️ Dwarf Fortress**: Fortress management simulation. Manage resources, direct dwarf builders, and build up defenses.
* **🧬 Caves of Qud, NetHack, DCSS, Angband, Brogue, Cataclysm DDA, Larn, ADOM, and 26 more deep dungeon explorers!**

### 3. 🎲 Board & Card Games
* **🚢 Battleship**: Turn-based marine fleet warfare! Arrange your five tactical military ships on a 10x10 ocean grid and battle adaptive target AI.
* **👑 Chess**: Play a complete game of chess against a built-in minimax AI engine.
* **🃏 Blackjack, Solitaire, FreeCell, Othello, Checkers, Gomoku, Cribbage, Backgammon, and 15 more tabletop classics!**

### 4. 📜 Text Adventures
* **🗺️ Adventure (Colossal Cave)**: Immersive text adventure with rooms, items, puzzles, and interactive choices.
* **🚀 Battlestar**: Navigate a crashed starship, manage systems, search ruins, and escape safely.
* **🧙 Phantasia & Zork**: Explore legendary fantasy realms, cast spells, gather artifacts, and level up.

### 5. 🧠 Brain & Speed Utilities
* **🔢 2048 / Term2048**: Slide and merge matching power-of-2 values on a grid.
* **💣 Minesweeper**: Pure logic grid deduction. Dig safe spaces, flag mines, and trigger cascading reveals.
* **Nudoku / Sudoku**: Terminal number-grid logic puzzles for fast keyboard solving.
* **🔤 Wordle**: Six attempts to guess the secret 5-letter word with live status coloring.
* **🪓 Hangman**: Traditional letter deduction word game with dynamic gallows ASCII illustrations.
* **⚡ Primes, Morse, BCD, Caesar Cipher, Rot13, and other speed-calculation challenges!**

---

## 🛠️ Build & Run Instructions

### Prerequisites
Make sure you have the Rust compiler and Cargo installed. (Rust 1.70+ recommended).

### 🚀 Compile and Run
To run the arcade console directly:
```bash
cargo run --release
```

To compile to a single, optimized executable:
```bash
cargo build --release
```
The compiled binary will be placed at `./target/release/rust-project-one` (or `rust-project-one.exe` on Windows).

---

## 🎹 Global & Game Controls

### 🕹️ Cabinet Launcher Menu
* **Left / Right Arrow (or A / D)**: Switch between genre categories.
* **Up / Down Arrow (or W / S)**: Navigate the paginated game selection list.
* **Enter / Spacebar**: Launch and play the highlighted game.
* **T**: Open theme settings.
* **Escape / Q**: Exit the retro arcade cabinet.

### Theme Settings
* **Left / Right Arrow (or A / D)**: Cycle through Classic Neon, Amber CRT, Matrix, Ocean, and Monochrome themes.
* **Enter / Escape / T**: Return to the cabinet launcher.
* Theme choices are saved to `arcade_settings.json` in the active directory.

### 🧱 Flagship Game Controls

#### Tetris Controls
* **Left / Right Arrow (or A / D)**: Slide the active piece left/right.
* **Up Arrow (or W)**: Rotate the active piece 90 degrees clockwise.
* **Down Arrow (or S)**: Soft drop the active piece (gives extra points).
* **Spacebar**: Hard drop the piece instantly (gives double points).
* **C / Shift (or H)**: Hold/Swap active piece.
* **P**: Pause / Resume.
* **Escape**: Quit game and return to the arcade selection launcher.

#### 🔢 2048 Controls
* **Arrow Keys / WASD**: Slide and merge tiles in that direction.
* **P**: Pause / Resume.
* **Escape**: Return to the arcade menu.

#### 💣 Minesweeper Controls
* **Arrow Keys / WASD**: Move the cursor highlight.
* **Spacebar / Enter**: Dig and reveal cell contents.
* **F**: Toggle flag marker (`⚑`) on/off (flagged cells are protected from digs).
* **P**: Pause / Resume.
* **Escape**: Return to the arcade menu.

#### 👽 Space Invaders Controls
* **Left / Right Arrow (or A / D)**: Move the defender ship.
* **Spacebar (or W / Up Arrow)**: Fire laser.
* **P**: Pause / Resume.
* **Escape**: Return to the arcade menu.

#### 🗡️ Roguelike Crawler Controls
* **Arrow Keys / WASD**: Move player `@` / Attack adjacent enemies.
* **Spacebar / Period (`.`)**: Wait a turn (monsters will still move/act).
* **P**: Pause / Resume.
* **Escape**: Return to the arcade menu.

#### 🐍 Snake Controls
* **Arrow Keys / WASD**: Steer the snake.
* **P**: Pause / Resume.
* **Escape**: Return to the arcade menu.

#### 🔤 Wordle Controls
* **A - Z Keys**: Input letters into active word row cells.
* **Backspace**: Delete the last input character in the row.
* **Enter**: Submit your 5-letter guess word.
* **P**: Pause / Resume.
* **Escape**: Return to the arcade menu.

#### 🚢 Battleship Controls
* **Arrow Keys / WASD**: Move ship hover cursor (placement) or targeting crosshair (battle).
* **R Key**: Rotate ship placement 90 degrees (vertical/horizontal toggle).
* **Spacebar / Enter**: Place ship (placement phase) or fire weapon salvo (battle phase).
* **P**: Pause / Resume.
* **Escape**: Return to the arcade menu.

#### 🪓 Hangman Controls
* **A - Z Keys**: Guess letters.
* **P**: Pause / Resume.
* **Escape**: Return to the arcade menu.

#### Tron Controls
* **Arrow Keys / WASD**: Turn the light cycle without reversing into your own trail.
* **P**: Pause / Resume.
* **Escape**: Return to the arcade menu.

---

## 🏆 Local High Score Tracking
Scores are persistent! When you finish a game (or return to the menu), top scores are compared, saved, and serialized in `highscores.json` in the active directory. The Arcade launcher displays your high scores on the game preview screen in real-time.

Theme settings are also persistent and are stored in `arcade_settings.json` beside the high-score file.

---

## 🎨 Premium Polish Details
* **Flicker-Free double-buffering**: Using `ratatui`'s in-memory layout system which only outputs diff characters, keeping drawing extremely smooth during rapid real-time updates.
* **Category Navigation**: 5 distinct pages separating action arcade, rogelikes, puzzles, text stories, and utilities.
* **Theme Settings**: Press `T` from the launcher to switch between five color palettes without leaving the terminal.
* **First-Click Safety (Minesweeper)**: Mines are only generated *after* your first dig, ensuring a safe start pocket is opened.
* **Wall Kicks (Tetris)**: Smooth rotations near boundary walls rather than blocking rotation completely.
* **Degrading Barriers (Space Invaders)**: Bunker blocks take up to three hits and degrade visually (`██` -> `▒▒` -> `░░` -> blank).
* **Bump Combat & Combat Logs (Roguelike)**: Turn-based interactions write colored messages (damage dealt, items grabbed, level up) dynamically into a scrollable sidebar text box.
* **Aesthetic Alphabet tracker (Wordle)**: Active keyboard status matrix helps you easily keep track of correct, misplaced, and dead letters.
* **AI Salvo Predictions (Battleship)**: Enemy computer AI utilizes adaptive search methods to target surrounding fields after scoring a ship hit.
* **Tron Light Trails**: Real-time cycle movement leaves persistent paths that become hazards as the duel accelerates.
