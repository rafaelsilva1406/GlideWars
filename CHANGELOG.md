# Changelog

All notable changes to Glide Wars will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Speed Boost System**
  - Press SHIFT or TAB to boost at 1.8x speed
  - Drains 50 energy/sec, recharges 20 energy/sec
  - Visual boost energy bar in HUD (cyan/blue/gray)
  - Affects both forward and horizontal movement

- **Flying Rings Challenge**
  - Cyan glowing rings spawn ahead of player
  - Fly through for +100 bonus points
  - Forgiving edge detection (5-unit collection radius)
  - Rings rotate slowly for visual effect
  - Spawns every ~5 seconds

- **Drone Companion Sidekick**
  - Green glowing powerup spawns friendly AI drone
  - 35% spawn rate (highest special powerup chance)
  - Active for 30 seconds after collection
  - Two AI behaviors: Follow Player / Clear Ahead
  - Auto-shoots enemies every 0.5 seconds
  - Invulnerable to all damage
  - +25 points per enemy hit by drone
  - +300 points for collecting powerup
  - Timer displayed in HUD with color coding
  - Deactivates on player death/respawn

- **Atmospheric Effects**
  - Decorative clouds in the sky (no collision)
  - Fluffy cloud formations made of overlapping spheres
  - Semi-transparent (5-15% opacity)
  - Drift slowly for ambient movement
  - Spawn every 5 seconds in small clusters (1-3)
  - Air trail particle effects when moving vertically
  - Light blue-white line streaks behind player
  - Particles fade out over 0.5 seconds
  - Only emit when vertical velocity > 1.0

- **Tutorial Instructions Screen**
  - Comprehensive pre-game instructions
  - Four sections: Controls, Objectives, Hazards, Tips
  - Mentions all game mechanics including boost and rings
  - Press SPACE to start, ESC to go back
  - Stops timer until player confirms ready

- **Documentation**
  - Complete GAMEPLAY_GUIDE.md with walkthrough
  - Updated README.md with all new features
  - CHANGELOG.md for version tracking

### Changed
- **Tutorial Balancing**
  - Reduced checkpoints from ~40 to **7 before boss**
  - Changed checkpoint interval from 50 to 300 units
  - Tutorial now properly teaches mechanics in 4 minutes
  - Boss spawns at 3:30 mark (was 3:30 but checkpoints were too frequent)

- **Boss System Improvements**
  - Boss now **matches player's forward velocity** (including boost)
  - Maintains 15-25 unit distance ahead of player
  - Boss no longer falls behind or gets left behind
  - More engaging and dynamic boss fights
  - Boss spawning now includes velocity parameter

- **Powerup Collection**
  - Magnetic pull system with 7-unit range
  - Visual feedback (extra glow) when powerup is being pulled
  - Increased collection radius from 1.5 to 2.0 units
  - Powerups now have velocity for smooth attraction

- **Ring Collection**
  - Forgiving collection radius (ring radius + 1.0 tolerance)
  - Z-axis tolerance increased to 3.0 units
  - Edge detection instead of requiring center hit
  - Updated tutorial instructions to reflect easier collection

- **Level Completion System**
  - Level timer now continues during boss fight
  - Auto-completes at 4:00 (tutorial) even if boss not defeated
  - Proper transition to LevelComplete state
  - Saves high scores and unlocks next continent

- **Production Build Quality**
  - Debug messages hidden in release builds with `#[cfg(debug_assertions)]`
  - No more checkpoint spam in console
  - No more state transition messages
  - Clean output for production
  - Debug info still available in development builds

- **HUD Improvements**
  - Boost energy bar added below health bar
  - Drone companion timer displayed when active
  - Color-coded indicators (green/yellow/orange/red)
  - Cleaner panel layout with better spacing

### Fixed
- Boss fight not completing levels properly
- Player flying past boss during combat
- Respawn death loop with obstacles spawning too close
- Tutorial instructions not showing before gameplay
- State machine blocking tutorial selection
- Powerups difficult to collect (too precise)
- Ring collection requiring exact center hit
- Level timer stopping during boss fight

### Performance
- Optimized cloud system (reduced density and opacity)
- Cloud size reduced by 50% to improve visibility
- Spawn rate decreased from 3s to 5s intervals
- Fewer clouds per cluster (1-3 instead of 2-5)
- Particle system with proper cleanup
- Background task cleanup on state transitions

## [0.1.0] - 2024-XX-XX

### Added
- Initial release with core gameplay
- 7 continents with unique themes
- Boss battles with 3-phase system
- Checkpoint save system
- Powerup system (health, weapons, ammo)
- Mobile touch controls
- Save/load game data
- High scores and best times
- Docker containerization
- Azure deployment infrastructure
- CI/CD pipelines
- Asset management system

[Unreleased]: https://github.com/yourusername/glidewars/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/glidewars/releases/tag/v0.1.0
