# Glide Wars

A retro-style 3D flight survival game built with Rust and macroquad. Navigate your glider through treacherous terrain across multiple continents, defeat bosses, and compete for high scores!

## Features

### Core Gameplay
- **3D Flight Mechanics**: Smooth glider controls with realistic physics
- **7 Continents**: Tutorial + 6 unique continents with distinct themes
- **Boss Battles**: Epic boss fights with multiple phases and attack patterns
- **Checkpoint System**: Save progress and respawn at checkpoints
- **Power-ups**: Collect weapons, health, and score multipliers
- **Progressive Difficulty**: Each continent gets harder with unique challenges

### Technical Features
- **Cross-Platform**: Desktop (Windows, macOS, Linux) and Web (WASM)
- **Mobile Support**: Touch controls with virtual joystick
- **Retro Aesthetic**: 80s arcade-inspired visuals with modern 3D graphics
- **Asset Management**: Themeable UI and continent-specific assets
- **State Machine**: Robust game state management system

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
- **Shoot**: Space
- **Menu Navigation**: Enter/Space to confirm, ESC to go back

### Mobile/Touch
- **Virtual Joystick**: Bottom-left corner for movement
- **Fire Button**: Bottom-right corner to shoot
- **Auto-detected**: Game automatically switches between desktop and mobile controls

## Game Structure

### Continents
1. **Tutorial**: Learn the basics (4 minutes)
2. **North America**: Mountain terrain
3. **South America**: Jungle environment
4. **Europe**: Urban landscapes
5. **Asia**: Eastern-inspired terrain
6. **Africa**: Desert environment
7. **Oceania**: Ocean-based challenges

Each continent features:
- 5-minute gameplay duration
- 3 checkpoints
- Unique boss battle
- Progressive difficulty scaling

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

### Completed (Phase 1-5, 8)
- ✅ Core game architecture
- ✅ State machine system
- ✅ Input management (desktop + mobile)
- ✅ Level system with 7 continents
- ✅ Checkpoint system with respawning
- ✅ Boss battles with multiple phases
- ✅ Asset management system
- ✅ Docker containerization
- ✅ Terraform infrastructure
- ✅ CI/CD pipelines

### Planned (Phase 3, 6, 7)
- ⏳ Enhanced UI screens (splash, menus, level select)
- ⏳ Rotating globe level selection
- ⏳ Tutorial level implementation
- ⏳ Parallax scrolling backgrounds
- ⏳ Save system with persistence
- ⏳ Session tracking and leaderboards
- ⏳ Backend API for scores
- ⏳ Sound effects and music
- ⏳ Achievement system

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
