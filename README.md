# Glide Wars

A retro-style 3D flight survival game built with Rust and macroquad. Navigate your glider through treacherous terrain across multiple continents, defeat bosses, and compete for high scores!

## 🚀 Quick Start for New Players

1. **Download & Run**: `cargo run --release`
2. **Start Game**: Press any key at splash → START → Tutorial
3. **Read Instructions**: Tutorial screen explains all controls
4. **Basic Controls**: WASD to move, SHIFT to boost, SPACE to shoot
5. **Objective**: Survive 4 minutes, collect powerups, defeat the boss
6. **Full Guide**: See **[GAMEPLAY_GUIDE.md](GAMEPLAY_GUIDE.md)** for detailed walkthrough

### What Makes Glide Wars Fun?
- **Speed Boost**: Tactical energy management adds strategy
- **Flying Rings**: Skill-based scoring for precise pilots
- **Drone Companion**: AI sidekick helps clear enemies
- **Boss Fights**: Dynamic battles that match your speed
- **Atmospheric**: Clouds and air trails make you feel like you're really flying!

## Features

### Core Gameplay
- **3D Flight Mechanics**: Smooth glider controls with realistic physics and speed boost
- **7 Continents**: Tutorial + 6 unique continents with distinct themes
- **Boss Battles**: Epic boss fights that stay in front of you with multiple phases
- **Checkpoint System**: Save progress every 50-300 units and respawn safely
- **Flying Rings**: Skill-based bonus scoring - fly through cyan rings for +100 points
- **Power-ups**: Collect weapons, health, ammo, and drone companions
- **Drone Companion**: AI sidekick that helps clear enemies (35% spawn rate, 30s duration)
- **Atmospheric Effects**: Dynamic clouds and air trail particles for immersion
- **Progressive Difficulty**: Each continent gets harder with unique challenges

### Technical Features
- **Cross-Platform**: Desktop (Windows, macOS, Linux) and Web (WASM)
- **Mobile Support**: Touch controls with virtual joystick
- **Retro Aesthetic**: 80s arcade-inspired visuals with modern 3D graphics
- **Asset Management**: Themeable UI and continent-specific assets
- **State Machine**: Robust game state management system

## 📖 Documentation

- **[Gameplay Guide](GAMEPLAY_GUIDE.md)**: Complete guide for new players with tips and strategies
- **[Changelog](CHANGELOG.md)**: Version history and recent updates
- **[Testing Guide](TESTING.md)**: Information about the testing infrastructure
- **[Persistence System](PERSISTENCE.md)**: Details on save system and data storage

## Quick Start

### Prerequisites

- **Rust**: Install from [rust-lang.org](https://www.rust-lang.org/tools/install)
- **Git**: For cloning the repository

### Running the Game (Desktop)

```bash
# Clone the repository
git clone https://github.com/yourusername/glidewars.git
cd glidewars

# Run the game
cargo run --release
```

### Building for Web (WASM)

```bash
# Install wasm target
rustup target add wasm32-unknown-unknown

# Build for WASM
cargo build --release --target wasm32-unknown-unknown

# The output will be in target/wasm32-unknown-unknown/release/glidewars.wasm
```

## Controls

### Desktop
- **Movement**: WASD or Arrow Keys
  - **W/Up**: Climb (counter gravity)
  - **S/Down**: Dive faster
  - **A/Left**: Move left
  - **D/Right**: Move right
- **Speed Boost**: SHIFT or TAB (drains boost energy, 1.8x speed)
- **Shoot**: Space
- **Pause/Menu**: ESC
- **Menu Navigation**: Enter/Space to confirm, ESC to go back

### Mobile/Touch
- **Virtual Joystick**: Bottom-left corner for movement
- **Fire Button**: Bottom-right corner to shoot
- **Auto-detected**: Game automatically switches between desktop and mobile controls

### Advanced Tips
- **Boost Management**: Boost drains 50 energy/sec, recharges 20 energy/sec when not boosting
- **Ring Collection**: Fly through cyan rings for bonus points - edge detection is forgiving
- **Drone Companion**: Green glowing powerup spawns a friendly AI that shoots enemies for 30 seconds
- **Air Trails**: Visual feedback when moving vertically - shows your air resistance

## Game Structure

### Continents
1. **Tutorial**: Learn the basics (4 minutes)
   - **7 checkpoints** before boss (every 300 units)
   - Tutorial instructions screen explains all mechanics
   - Boss spawns at 3:30 (Training Drone)
   - Forgiving difficulty to learn controls
2. **North America**: Mountain terrain (5 minutes)
3. **South America**: Jungle environment (5 minutes)
4. **Europe**: Urban landscapes (5 minutes)
5. **Asia**: Eastern-inspired terrain (5 minutes)
6. **Africa**: Desert environment (5 minutes)
7. **Oceania**: Ocean-based challenges (5 minutes)

Each continent features:
- Time-based progression (4-5 minutes)
- Multiple checkpoints (tutorial: 7, others: varies)
- Unique boss battle at ~90% mark
- Progressive difficulty scaling
- Flying rings for bonus scoring
- Atmospheric clouds and visual effects

### Boss System
- **7 Unique Bosses**: One per continent
- **3 Attack Phases**: Based on boss health (100%, 66%, 33%)
- **Multiple Attack Patterns**: Projectile barrages, charge attacks, circular shots
- **Visual Feedback**: Health bars, phase indicators, special effects

## Development

### Project Structure

```
glidewars/
├── src/
│   ├── main.rs              # Entry point and game loop
│   ├── player.rs            # Player mechanics
│   ├── enemy.rs             # Enemy AI
│   ├── boss.rs              # Boss system
│   ├── terrain.rs           # Terrain generation
│   ├── level.rs             # Level management
│   ├── checkpoint.rs        # Checkpoint system
│   ├── game_state.rs        # State machine
│   ├── scene_manager.rs     # Scene coordination
│   ├── input_manager.rs     # Input handling (desktop + mobile)
│   ├── assets/              # Asset management
│   │   ├── mod.rs
│   │   ├── theme.rs
│   │   └── loader.rs
│   └── ...
├── assets/                  # Game assets
│   ├── themes/              # Visual themes
│   └── continents/          # Continent configurations
├── terraform/               # Infrastructure as Code
├── .github/workflows/       # CI/CD pipelines
├── Dockerfile               # Container definition
├── docker-compose.yml       # Local deployment
└── README.md
```

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Deployment

### Local with Docker

```bash
# Build and run with docker-compose
docker-compose up --build

# Access at http://localhost:8080
```

### Azure Deployment

See [terraform/README.md](terraform/README.md) for detailed Azure deployment instructions.

Quick steps:
1. Configure Azure credentials
2. Update `terraform/terraform.tfvars`
3. Run `terraform init && terraform apply`
4. Build and push Docker image
5. Access game at the output URL

**Estimated Cost**: ~$18/month for basic setup

## CI/CD

The project includes three GitHub Actions workflows:

1. **test.yml**: Runs on push/PR, checks code quality and builds WASM
2. **deploy.yml**: Deploys to Azure on push to main
3. **release.yml**: Creates releases for tagged commits

### Required Secrets

For Azure deployment, configure these GitHub secrets:
- `ACR_LOGIN_SERVER`: Azure Container Registry URL
- `ACR_USERNAME`: ACR username
- `ACR_PASSWORD`: ACR password
- `AZURE_CREDENTIALS`: Azure service principal JSON
- `AZURE_RESOURCE_GROUP`: Resource group name

## Roadmap

### Completed ✅
**Core Systems**
- ✅ Core game architecture with state machine
- ✅ Input management (desktop + mobile)
- ✅ Level system with 7 continents
- ✅ Checkpoint system with safe respawning
- ✅ Boss battles with multiple phases and dynamic positioning
- ✅ Asset management system
- ✅ Docker containerization
- ✅ Terraform infrastructure
- ✅ CI/CD pipelines

**Gameplay Features**
- ✅ Speed boost system (SHIFT/TAB key)
- ✅ Flying rings for skill-based scoring
- ✅ Drone companion AI sidekick
- ✅ Tutorial with comprehensive instructions
- ✅ Magnetic powerup collection
- ✅ Save system with high scores and best times

**Visual Polish**
- ✅ Atmospheric clouds (decorative, no collision)
- ✅ Air trail particle effects
- ✅ Enhanced UI screens (splash, menus, level select)
- ✅ Clean production builds (debug messages hidden)

### Planned ⏳
- ⏳ Rotating globe level selection (currently 2D map)
- ⏳ Parallax scrolling backgrounds per continent
- ⏳ Session tracking and online leaderboards
- ⏳ Backend API for global scores
- ⏳ Sound effects and music
- ⏳ Achievement system
- ⏳ Additional powerups and weapons

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [macroquad](https://github.com/not-fl3/macroquad) - Easy-to-use game engine
- Inspired by classic arcade flight games
- Retro aesthetic inspired by 1980s vector graphics

## Support

- **Issues**: Report bugs or request features via GitHub Issues
- **Discussions**: Join conversations in GitHub Discussions

---

Made with ❤️ and Rust 🦀
