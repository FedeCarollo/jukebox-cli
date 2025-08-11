## Overview
Adds a new Driving Game mode: a 3‑lane top‑down mini‑game rendered in Ratatui that runs independently of audio playback. The player drives a car across three lanes, dodges incoming cars, and fires projectiles straight up to destroy them. Game logic runs on a fixed timestep; audio playback (Rodio/Symphonia) remains non‑blocking and continues in the background by default.

## Controls
- W/S: move up/down
- A/D: move left/right between 3 lanes
- Space: fire projectile upward
- P: pause/resume
- Esc or Q: exit game back to main UI
- R: restart on Game Over
- M: toggle music during game (default: on)

## Mechanics
- Player starts with 3 lives; collision with an enemy reduces life and triggers ~1s invulnerability frames.
- Enemies spawn at the top with a probability that increases over time; downward speed slightly accelerates.
- Projectiles destroy the first enemy hit in their lane; +100 score each.
- Enemies that reach the bottom are removed (no penalty beyond missed score).
- Game over when lives reach 0; overlay appears with score and restart/exit options.

## Architecture
- New module tree under `src/game/`:
  - `scene.rs`: `GameScene` enum (Running, Paused, GameOver)
  - `entities.rs`: `Player`, `Enemy`, `Projectile`
  - `state.rs`: `GameState` (lanes, player, enemies, projectiles, score, lives, rng, timers)
  - `input.rs`: key mapping to `GameCmd`
  - `spawn.rs`: enemy spawn logic and difficulty curve
  - `update.rs`: per‑tick movement, collisions, cleanup
  - `draw.rs`: rendering of borders, lanes, cars, projectiles, HUD
- Integration:
  - `AppMode::DrivingGame` in `src/screen/main_loop.rs`
  - Fixed timestep loop (~33ms) with accumulator; render each tick
  - Input routed to game mapping when in game mode; jukebox controls untouched otherwise
  - Audio playback independent and non‑blocking; optional in‑game toggle (M)

## Acceptance Criteria
- New menu/control entry to start “Driving Game” (`g`) starts gameplay without crashing
- Left/right borders and three vertical lanes rendered; player moves with WASD within bounds
- Enemies spawn and move down; collisions reduce lives with invulnerability frames
- Space fires projectiles that destroy enemies and award score
- Pause works; Esc/Q returns to main UI; Game Over screen shows score and allows restart
- Works in at least 80x24; degrades gracefully on smaller sizes
- No regressions to existing MP3 playback; game runs even without a loaded track

## Demo
- Screenshot available as `screenshot.png` in repo.
- (Optional) A short GIF can be added in a follow‑up.

## README
See the new section: [Driving Game Mode](README.md#driving-game-mode).

## Notes
- Render uses ASCII with safe ANSI colors; avoids Unicode assumptions
- Basic test included for lane math monotonicity in `state.rs`

## Commits
Key commits in this branch include module scaffolding, integration, and fixed‑timestep/input/draw wiring. See commit history in this PR for details.
