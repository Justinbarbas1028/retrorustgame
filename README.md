# 🕹️ Rust Terminal Retro Arcade Console (Mega 10-Game Edition)

An extremely polished, premium retro arcade dashboard containing **10 classic terminal games** in a single, lightweight binary. Built entirely in idiomatic Rust using `ratatui` for double-buffered, flicker-free rendering, and `crossterm` for cross-platform terminal control and non-blocking real-time keyboard navigation.

---

## 🎮 Games Included

1. **🧱 Tetris**: Real-time tile match game! Fit falling tetrominoes together to clear complete horizontal lines. Features holding slots, next block previews, level-based speeds, and ghost piece shadows.
2. **🔢 2048**: Slide and merge matching power-of-2 values on a grid. Combine identical tiles to build up to the legendary 2048 block. Fully styled with colorful tile blocks.
3. **💣 Minesweeper**: Pure logic grid deduction. Dig safe spaces, flag hidden mines, and rely on cascading revelations with standard Windows-style color-coded indicator counts. Includes a first-click safety mechanism!
4. **👽 Space Invaders**: Real-time arcade shooter action! Defend your shield bunkers, glide your base ship, and blast descending swarms of invaders. Swarms accelerate as they shrink!
5. **🗡️ Roguelike Dungeon Crawler**: A turn-based RPG adventure. Wander procedurally generated rooms, pick up weapons and shields, drink potions, slay trolls, level up stats, and descend down the stairs.
6. **🐍 Snake**: Control a neon-green snake to consume glowing red food. Grows longer and speeds up as you eat, testing your reflexes to avoid colliding with boundaries or your own tail!
7. **🔤 Wordle**: The legendary 5-letter word game! You have 6 attempts to guess the secret word. Features colorful elimination keyboard grids, letter status indicators, and word banks.
8. **🚢 Battleship**: Turn-based marine fleet warfare! Arrange your five tactical military ships on a 10x10 ocean grid and take turns trading target coordinate fires against a responsive computer AI.
9. **🪓 Hangman**: Traditional letter deduction word game. Guess characters one by one to reveal the secret word. Gallows ASCII illustrations update with every incorrect guess.
10. **🏓 Pong**: The legendary grandfather of arcade games! Move your left paddle real-time to bounce and spin the ball past the computer AI's paddle. Matches played to 5 points with physics acceleration.

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

### 🕹️ Arcade Launcher Menu
* **Up / Down Arrow (or W / S)**: Navigate the games selection list.
* **Enter / Space**: Launch and play the highlighted game.
* **Escape / Q**: Exit the retro arcade cabinet.

### 🧱 Tetris Controls
* **Left / Right Arrow (or A / D)**: Slide the active piece left/right.
* **Up Arrow (or W)**: Rotate the active piece 90 degrees clockwise.
* **Down Arrow (or S)**: Soft drop the active piece (gives extra points).
* **Spacebar**: Hard drop the piece instantly (gives double points).
* **C / Shift (or H)**: Hold/Swap active piece.
* **P**: Pause / Resume.
* **Escape**: Quit game and return to the arcade selection launcher.

### 🔢 2048 Controls
* **Arrow Keys / WASD**: Slide and merge tiles in that direction.
* **P**: Pause / Resume.
* **Escape**: Return to the arcade menu.

### 💣 Minesweeper Controls
* **Arrow Keys / WASD**: Move the cursor highlight.
* **Spacebar / Enter**: Dig and reveal cell contents.
* **F**: Toggle flag marker (`⚑`) on/off (flagged cells are protected from digs).
* **P**: Pause / Resume.
* **Escape**: Return to the arcade menu.

### 👽 Space Invaders Controls
* **Left / Right Arrow (or A / D)**: Move the defender ship.
* **Spacebar (or W / Up Arrow)**: Fire laser.
* **P**: Pause / Resume.
* **Escape**: Return to the arcade menu.

### 🗡️ Roguelike Crawler Controls
* **Arrow Keys / WASD**: Move player `@` / Attack adjacent enemies.
* **Spacebar / Period (`.`)**: Wait a turn (monsters will still move/act).
* **P**: Pause / Resume.
* **Escape**: Return to the arcade menu.

### 🐍 Snake Controls
* **Arrow Keys / WASD**: Steer the snake.
* **P**: Pause / Resume.
* **Escape**: Return to the arcade menu.

### 🔤 Wordle Controls
* **A - Z Keys**: Input letters into active word row cells.
* **Backspace**: Delete the last input character in the row.
* **Enter**: Submit your 5-letter guess word.
* **P**: Pause / Resume.
* **Escape**: Return to the arcade menu.

### 🚢 Battleship Controls
* **Arrow Keys / WASD**: Move ship hover cursor (placement) or targeting crosshair (battle).
* **R Key**: Rotate ship placement 90 degrees (vertical/horizontal toggle).
* **Spacebar / Enter**: Place ship (placement phase) or fire weapon salvo (battle phase).
* **P**: Pause / Resume.
* **Escape**: Return to the arcade menu.

### 🪓 Hangman Controls
* **A - Z Keys**: Guess letters.
* **P**: Pause / Resume.
* **Escape**: Return to the arcade menu.

### 🏓 Pong Controls
* **Up / Down Arrow (or W / S)**: Move player paddle up/down in real-time.
* **P**: Pause / Resume.
* **Escape**: Return to the arcade menu.

---

## 🏆 Local High Score Tracking
Scores are persistent! When you finish a game (or return to the menu), top scores are compared, saved, and serialized in `highscores.json` in the active directory. The Arcade launcher displays your high scores on the game preview screen in real-time.

---

## 🎨 Premium Polish Details
* **Flicker-Free double-buffering**: Using `ratatui`'s in-memory layout system which only outputs diff characters, keeping drawing extremely smooth during rapid real-time updates.
* **First-Click Safety (Minesweeper)**: Mines are only generated *after* your first dig, ensuring a safe start pocket is opened.
* **Wall Kicks (Tetris)**: Smooth rotations near boundary walls rather than blocking rotation completely.
* **Degrading Barriers (Space Invaders)**: Bunker blocks take up to three hits and degrade visually (`██` -> `▒▒` -> `░░` -> blank).
* **Bump Combat & Combat Logs (Roguelike)**: Turn-based interactions write colored messages (damage dealt, items grabbed, level up) dynamically into a scrollable sidebar text box.
* **Aesthetic Alphabet tracker (Wordle)**: Active keyboard status matrix helps you easily keep track of correct, misplaced, and dead letters.
* **AI Salvo Predictions (Battleship)**: Enemy computer AI utilizes adaptive search methods to target surrounding fields after scoring a ship hit.
* **Physics Bounce Angle (Pong)**: Bouncing balls accelerate and adjust spin vectors depending on where on the paddle the ball collides.
