use macroquad::prelude::*;

mod player;
mod terrain;
mod enemy;
mod powerup;
mod camera_system;
mod game_state;
mod input_manager;
mod scene_manager;
mod assets;
mod level;
mod checkpoint;
mod boss;

use player::Player;
use terrain::TerrainManager;
use enemy::EnemyManager;
use powerup::PowerupManager;
use camera_system::GameCamera;
use game_state::GameState;
use input_manager::InputManager;
use scene_manager::SceneManager;
use assets::{AssetManager, Continent};
use level::LevelManager;
use checkpoint::CheckpointManager;
use boss::{Boss, BossType};

#[macroquad::main("Glide Wars")]
async fn main() {
    let mut scene_manager = SceneManager::new();
    let mut input_manager = InputManager::new();
    let asset_manager = AssetManager::default();

    // Game state
    let mut player = Player::new();
    let mut terrain = TerrainManager::new();
    let mut enemies = EnemyManager::new();
    let mut powerups = PowerupManager::new();
    let mut camera = GameCamera::new();

    // Level management
    let mut level_manager: Option<LevelManager> = None;
    let mut checkpoint_manager = CheckpointManager::new();
    let mut boss: Option<Boss> = None;
    let mut current_continent = Continent::Tutorial;

    loop {
        clear_background(BLACK);

        let dt = get_frame_time();

        // Update input
        let input = input_manager.update();

        // Handle screen resize
        let current_width = screen_width();
        let current_height = screen_height();
        input_manager.resize(current_width, current_height);

        // Update scene manager
        scene_manager.update(dt, &input_manager);

        // Handle states
        match scene_manager.current_state() {
            GameState::Splash => {
                draw_splash_screen();
            }

            GameState::MainMenu => {
                draw_main_menu(&input, &mut scene_manager);
            }

            GameState::Options => {
                draw_options_menu(&input, &mut scene_manager);
            }

            GameState::LevelSelect => {
                draw_level_select(&input, &mut scene_manager);
            }

            GameState::Tutorial | GameState::InGame => {
                // Initialize level manager if not present
                if level_manager.is_none() {
                    level_manager = Some(LevelManager::new(current_continent));
                    checkpoint_manager.clear();
                    boss = None;
                }

                let level_mgr = level_manager.as_mut().unwrap();

                // Update level
                level_mgr.update(dt, player.position().z);

                // Create checkpoints
                if level_mgr.should_create_checkpoint() {
                    checkpoint_manager.create_checkpoint(
                        player.position().z,
                        &player,
                        scene_manager.scene_data().score,
                        level_mgr.elapsed_time(),
                    );
                    level_mgr.mark_checkpoint_created();
                }

                // Spawn boss if time
                if level_mgr.should_spawn_boss() && boss.is_none() {
                    let boss_type = BossType::from_continent(current_continent);
                    let spawn_pos = player.position() + vec3(0.0, 5.0, 30.0);
                    boss = Some(Boss::new(boss_type, spawn_pos));
                    scene_manager.request_transition(GameState::BossFight);
                }

                // Check if level complete
                if level_mgr.is_complete() && boss.as_ref().map_or(true, |b| b.is_defeated()) {
                    scene_manager.request_transition(GameState::LevelComplete);
                }

                // Update game
                player.update(dt);
                terrain.update(dt, &player);
                enemies.update(dt, &player);

                let score = scene_manager.scene_data().score;
                let mut score_mut = score;
                powerups.update(dt, &player, &mut score_mut);
                scene_manager.scene_data_mut().score = score_mut;

                // Check collisions
                if terrain.check_collision(&player) || enemies.check_collision(&player) {
                    player.take_damage(10.0);
                    if player.is_dead() {
                        checkpoint_manager.start_respawn();
                        scene_manager.request_transition(GameState::Checkpoint);
                    }
                }

                // Check powerup collection
                powerups.check_collection(&mut player, &mut scene_manager.scene_data_mut().score);

                // Update camera
                camera.update(&player);

                // Render 3D scene
                set_camera(camera.get_camera());

                terrain.draw();
                enemies.draw();
                powerups.draw();
                player.draw();

                // Render 2D UI
                set_default_camera();

                draw_hud_with_level(&player, scene_manager.scene_data().score, level_mgr, &checkpoint_manager, &asset_manager, current_continent);

                // Back to menu
                if input.back {
                    scene_manager.request_transition(GameState::MainMenu);
                    level_manager = None;
                }
            }

            GameState::BossFight => {
                if let Some(ref mut boss_instance) = boss {
                    // Update boss
                    boss_instance.update(dt, player.position());

                    // Boss collision with player
                    if boss_instance.check_collision_with_player(player.position()) {
                        player.take_damage(20.0 * dt); // Continuous damage
                    }

                    // Boss projectiles hit player
                    if boss_instance.check_projectile_collision(player.position()) {
                        player.take_damage(15.0);
                    }

                    // Player projectiles hit boss
                    let projectiles = player.get_projectiles();
                    for (idx, proj) in projectiles.iter().enumerate() {
                        if boss_instance.check_hit_by_player_projectile(proj.position) {
                            // Add score for hitting boss
                            scene_manager.scene_data_mut().score += 50;
                        }
                    }

                    // Check if boss defeated
                    if boss_instance.is_defeated() {
                        scene_manager.scene_data_mut().score += 5000; // Big bonus for defeating boss
                        if let Some(ref level_mgr) = level_manager {
                            if level_mgr.is_complete() {
                                scene_manager.request_transition(GameState::LevelComplete);
                            }
                        }
                    }

                    // Check if player died
                    if player.is_dead() {
                        checkpoint_manager.start_respawn();
                        scene_manager.request_transition(GameState::Checkpoint);
                    }

                    // Update player and camera
                    player.update(dt);
                    camera.update(&player);

                    // Render 3D scene
                    set_camera(camera.get_camera());

                    terrain.draw();
                    player.draw();
                    boss_instance.draw();

                    // Render 2D UI
                    set_default_camera();

                    if let Some(ref level_mgr) = level_manager {
                        draw_hud_with_level(&player, scene_manager.scene_data().score, level_mgr, &checkpoint_manager, &asset_manager, current_continent);
                        draw_boss_health_bar(boss_instance);
                    }

                    if input.back {
                        scene_manager.request_transition(GameState::MainMenu);
                        level_manager = None;
                        boss = None;
                    }
                } else {
                    // No boss, transition back to game
                    scene_manager.request_transition(GameState::InGame);
                }
            }

            GameState::Checkpoint => {
                // Update respawn timer
                if checkpoint_manager.update_respawn(dt) {
                    // Respawn ready
                    checkpoint_manager.restore_player_state(&mut player, &mut scene_manager.scene_data_mut().score);

                    // Reset terrain to checkpoint position
                    if let Some(checkpoint_pos) = checkpoint_manager.get_last_checkpoint_position() {
                        terrain.reset_to_position(checkpoint_pos);
                    }

                    // Go back to appropriate state
                    if boss.is_some() && !boss.as_ref().unwrap().is_defeated() {
                        scene_manager.request_transition(GameState::BossFight);
                    } else {
                        scene_manager.request_transition(GameState::InGame);
                    }
                }

                draw_checkpoint_screen(&checkpoint_manager);

                // Allow manual restart
                if input.confirm {
                    checkpoint_manager.cancel_respawn();
                    checkpoint_manager.restore_player_state(&mut player, &mut scene_manager.scene_data_mut().score);

                    // Reset terrain to checkpoint position
                    if let Some(checkpoint_pos) = checkpoint_manager.get_last_checkpoint_position() {
                        terrain.reset_to_position(checkpoint_pos);
                    }

                    if boss.is_some() && !boss.as_ref().unwrap().is_defeated() {
                        scene_manager.request_transition(GameState::BossFight);
                    } else {
                        scene_manager.request_transition(GameState::InGame);
                    }
                }

                if input.back {
                    scene_manager.request_transition(GameState::MainMenu);
                    level_manager = None;
                    checkpoint_manager.clear();
                }
            }

            GameState::LevelComplete => {
                draw_level_complete(scene_manager.scene_data().score);

                if input.confirm {
                    // Reset for next level
                    level_manager = None;
                    checkpoint_manager.clear();
                    boss = None;
                    player = Player::new();
                    terrain = TerrainManager::new();
                    enemies = EnemyManager::new();
                    powerups = PowerupManager::new();

                    scene_manager.request_transition(GameState::LevelSelect);
                }

                if input.back {
                    level_manager = None;
                    checkpoint_manager.clear();
                    boss = None;
                    scene_manager.request_transition(GameState::MainMenu);
                }
            }

            GameState::GameOver => {
                draw_game_over(scene_manager.scene_data().score);

                if input.confirm {
                    // Reset game
                    player = Player::new();
                    terrain = TerrainManager::new();
                    enemies = EnemyManager::new();
                    powerups = PowerupManager::new();
                    checkpoint_manager.clear();
                    level_manager = Some(LevelManager::new(current_continent));
                    boss = None;
                    scene_manager.scene_data_mut().reset_score();
                    scene_manager.request_transition(GameState::InGame);
                }

                if input.back {
                    level_manager = None;
                    checkpoint_manager.clear();
                    boss = None;
                    scene_manager.request_transition(GameState::MainMenu);
                }
            }
        }

        // Draw mobile controls
        input_manager.draw();

        next_frame().await
    }
}

fn draw_hud(player: &Player, score: u32) {
    // Retro-style HUD with 80's aesthetic
    let hud_color = Color::from_rgba(0, 255, 255, 255); // Cyan like old arcade games

    // Health bar
    draw_text("HEALTH", 20.0, 30.0, 20.0, hud_color);
    draw_rectangle(20.0, 35.0, 200.0, 15.0, Color::from_rgba(40, 40, 40, 255));
    draw_rectangle(20.0, 35.0, player.health() * 2.0, 15.0, Color::from_rgba(0, 255, 0, 255));
    draw_rectangle_lines(20.0, 35.0, 200.0, 15.0, 2.0, hud_color);

    // Score
    draw_text(&format!("SCORE: {:08}", score), 20.0, 70.0, 20.0, hud_color);

    // Weapon indicator
    if let Some(weapon) = player.current_weapon() {
        draw_text(&format!("WEAPON: {}", weapon), 20.0, 95.0, 20.0, hud_color);
        draw_text(&format!("AMMO: {}", player.ammo()), 20.0, 115.0, 20.0, hud_color);
    }
}

fn draw_hud_with_level(
    player: &Player,
    score: u32,
    level_manager: &LevelManager,
    checkpoint_manager: &CheckpointManager,
    _asset_manager: &AssetManager,
    continent: Continent,
) {
    let hud_color = Color::from_rgba(0, 255, 255, 255);
    let screen_w = screen_width();

    // Health bar
    draw_text("HEALTH", 20.0, 30.0, 20.0, hud_color);
    draw_rectangle(20.0, 35.0, 200.0, 15.0, Color::from_rgba(40, 40, 40, 255));
    draw_rectangle(20.0, 35.0, player.health() * 2.0, 15.0, Color::from_rgba(0, 255, 0, 255));
    draw_rectangle_lines(20.0, 35.0, 200.0, 15.0, 2.0, hud_color);

    // Score
    draw_text(&format!("SCORE: {:08}", score), 20.0, 70.0, 20.0, hud_color);

    // Weapon indicator
    if let Some(weapon) = player.current_weapon() {
        draw_text(&format!("WEAPON: {}", weapon), 20.0, 95.0, 20.0, hud_color);
        draw_text(&format!("AMMO: {}", player.ammo()), 20.0, 115.0, 20.0, hud_color);
    }

    // Continent indicator (top center)
    let continent_text = continent.name();
    let text_width = measure_text(continent_text, None, 30, 1.0).width;
    draw_text(
        continent_text,
        screen_w / 2.0 - text_width / 2.0,
        40.0,
        30.0,
        hud_color,
    );

    // Timer (top right)
    let remaining = level_manager.remaining_time();
    let minutes = (remaining / 60.0) as u32;
    let seconds = (remaining % 60.0) as u32;
    let timer_text = format!("TIME: {:02}:{:02}", minutes, seconds);
    let timer_width = measure_text(&timer_text, None, 25, 1.0).width;
    draw_text(
        &timer_text,
        screen_w - timer_width - 20.0,
        35.0,
        25.0,
        hud_color,
    );

    // Checkpoint indicator (top right, below timer)
    let checkpoint_text = format!(
        "CHECKPOINT {}/{}",
        checkpoint_manager.checkpoint_count(),
        level_manager.total_checkpoints()
    );
    let checkpoint_width = measure_text(&checkpoint_text, None, 20, 1.0).width;
    draw_text(
        &checkpoint_text,
        screen_w - checkpoint_width - 20.0,
        60.0,
        20.0,
        Color::from_rgba(255, 255, 0, 255),
    );
}

fn draw_boss_health_bar(boss: &Boss) {
    let screen_w = screen_width();
    let bar_width = 400.0;
    let bar_height = 30.0;
    let x = screen_w / 2.0 - bar_width / 2.0;
    let y = 100.0;

    // Boss name
    let boss_type = boss.boss_type();
    let name = boss_type.name();
    let name_width = measure_text(name, None, 25, 1.0).width;
    draw_text(
        name,
        screen_w / 2.0 - name_width / 2.0,
        y - 10.0,
        25.0,
        Color::from_rgba(255, 0, 0, 255),
    );

    // Health bar background
    draw_rectangle(x, y, bar_width, bar_height, Color::from_rgba(40, 40, 40, 255));

    // Health bar fill
    let health_width = bar_width * boss.health_percentage();
    let health_color = if boss.health_percentage() > 0.5 {
        Color::from_rgba(255, 200, 0, 255)
    } else if boss.health_percentage() > 0.25 {
        Color::from_rgba(255, 100, 0, 255)
    } else {
        Color::from_rgba(255, 0, 0, 255)
    };
    draw_rectangle(x, y, health_width, bar_height, health_color);

    // Health bar border
    draw_rectangle_lines(x, y, bar_width, bar_height, 3.0, WHITE);

    // Health text
    let health_text = format!(
        "{:.0} / {:.0}",
        boss.health(),
        boss.max_health()
    );
    let health_text_width = measure_text(&health_text, None, 20, 1.0).width;
    draw_text(
        &health_text,
        screen_w / 2.0 - health_text_width / 2.0,
        y + 20.0,
        20.0,
        WHITE,
    );

    // Phase indicator
    let phase_text = format!("PHASE {}", boss.phase());
    let phase_width = measure_text(&phase_text, None, 20, 1.0).width;
    draw_text(
        &phase_text,
        screen_w / 2.0 - phase_width / 2.0,
        y + bar_height + 25.0,
        20.0,
        Color::from_rgba(255, 255, 0, 255),
    );
}

fn draw_splash_screen() {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // Draw title
    let title_text = "GLIDE WARS";
    let text_width = measure_text(title_text, None, 80, 1.0).width;
    draw_text(
        title_text,
        screen_width / 2.0 - text_width / 2.0,
        screen_height / 2.0 - 40.0,
        80.0,
        Color::from_rgba(0, 255, 255, 255)
    );

    // Loading text
    let loading_text = "LOADING...";
    let loading_width = measure_text(loading_text, None, 30, 1.0).width;
    draw_text(
        loading_text,
        screen_width / 2.0 - loading_width / 2.0,
        screen_height / 2.0 + 60.0,
        30.0,
        Color::from_rgba(255, 255, 255, 255)
    );
}

fn draw_main_menu(input: &input_manager::InputState, scene_manager: &mut SceneManager) {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // Title
    let title_text = "GLIDE WARS";
    let text_width = measure_text(title_text, None, 60, 1.0).width;
    draw_text(
        title_text,
        screen_width / 2.0 - text_width / 2.0,
        screen_height / 3.0,
        60.0,
        Color::from_rgba(0, 255, 255, 255)
    );

    // Menu options
    let start_text = "START - Press SPACE/Enter";
    let start_width = measure_text(start_text, None, 30, 1.0).width;
    draw_text(
        start_text,
        screen_width / 2.0 - start_width / 2.0,
        screen_height / 2.0,
        30.0,
        Color::from_rgba(255, 255, 255, 255)
    );

    let options_text = "OPTIONS - Press O";
    let options_width = measure_text(options_text, None, 30, 1.0).width;
    draw_text(
        options_text,
        screen_width / 2.0 - options_width / 2.0,
        screen_height / 2.0 + 50.0,
        30.0,
        Color::from_rgba(200, 200, 200, 255)
    );

    // Handle input
    if input.confirm || is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
        scene_manager.request_transition(GameState::LevelSelect);
    }

    if is_key_pressed(KeyCode::O) {
        scene_manager.request_transition(GameState::Options);
    }
}

fn draw_options_menu(input: &input_manager::InputState, scene_manager: &mut SceneManager) {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // Title
    let title_text = "OPTIONS";
    let text_width = measure_text(title_text, None, 50, 1.0).width;
    draw_text(
        title_text,
        screen_width / 2.0 - text_width / 2.0,
        screen_height / 3.0,
        50.0,
        Color::from_rgba(0, 255, 255, 255)
    );

    // Placeholder
    let placeholder = "Options menu - Coming in Phase 3";
    let placeholder_width = measure_text(placeholder, None, 25, 1.0).width;
    draw_text(
        placeholder,
        screen_width / 2.0 - placeholder_width / 2.0,
        screen_height / 2.0,
        25.0,
        Color::from_rgba(200, 200, 200, 255)
    );

    let back_text = "PRESS ESC TO GO BACK";
    let back_width = measure_text(back_text, None, 20, 1.0).width;
    draw_text(
        back_text,
        screen_width / 2.0 - back_width / 2.0,
        screen_height / 2.0 + 100.0,
        20.0,
        Color::from_rgba(255, 255, 255, 255)
    );

    // Handle input
    if input.back {
        scene_manager.request_transition(GameState::MainMenu);
    }
}

fn draw_level_select(input: &input_manager::InputState, scene_manager: &mut SceneManager) {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // Title
    let title_text = "SELECT CONTINENT";
    let text_width = measure_text(title_text, None, 50, 1.0).width;
    draw_text(
        title_text,
        screen_width / 2.0 - text_width / 2.0,
        screen_height / 4.0,
        50.0,
        Color::from_rgba(0, 255, 255, 255)
    );

    // Tutorial button
    let tutorial_text = "START TUTORIAL - Press T";
    let tutorial_width = measure_text(tutorial_text, None, 30, 1.0).width;
    draw_text(
        tutorial_text,
        screen_width / 2.0 - tutorial_width / 2.0,
        screen_height / 2.0,
        30.0,
        Color::from_rgba(255, 255, 0, 255)
    );

    // Placeholder for continents
    let placeholder = "Level selection globe - Coming in Phase 3";
    let placeholder_width = measure_text(placeholder, None, 25, 1.0).width;
    draw_text(
        placeholder,
        screen_width / 2.0 - placeholder_width / 2.0,
        screen_height / 2.0 + 60.0,
        25.0,
        Color::from_rgba(200, 200, 200, 255)
    );

    let back_text = "PRESS ESC TO GO BACK";
    let back_width = measure_text(back_text, None, 20, 1.0).width;
    draw_text(
        back_text,
        screen_width / 2.0 - back_width / 2.0,
        screen_height / 2.0 + 140.0,
        20.0,
        Color::from_rgba(255, 255, 255, 255)
    );

    // Handle input
    if input.back {
        scene_manager.request_transition(GameState::MainMenu);
    }

    if is_key_pressed(KeyCode::T) {
        scene_manager.request_transition(GameState::Tutorial);
    }

    if input.confirm {
        // For now, just start the game directly
        scene_manager.request_transition(GameState::InGame);
    }
}

fn draw_checkpoint_screen(checkpoint_manager: &CheckpointManager) {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // Semi-transparent overlay
    draw_rectangle(0.0, 0.0, screen_width, screen_height, Color::from_rgba(0, 0, 0, 180));

    // Title
    let title_text = "CHECKPOINT RESPAWN";
    let text_width = measure_text(title_text, None, 50, 1.0).width;
    draw_text(
        title_text,
        screen_width / 2.0 - text_width / 2.0,
        screen_height / 2.0 - 60.0,
        50.0,
        Color::from_rgba(255, 255, 0, 255)
    );

    // Countdown
    if let Some(countdown) = checkpoint_manager.respawn_time_remaining() {
        let countdown_text = format!("RESPAWN IN: {:.0}s", countdown);
        let countdown_width = measure_text(&countdown_text, None, 40, 1.0).width;
        draw_text(
            &countdown_text,
            screen_width / 2.0 - countdown_width / 2.0,
            screen_height / 2.0,
            40.0,
            Color::from_rgba(255, 255, 255, 255)
        );
    }

    // Checkpoint info
    if let Some(checkpoint_pos) = checkpoint_manager.get_last_checkpoint_position() {
        let info_text = format!("Last checkpoint at Z={:.0}", checkpoint_pos);
        let info_width = measure_text(&info_text, None, 20, 1.0).width;
        draw_text(
            &info_text,
            screen_width / 2.0 - info_width / 2.0,
            screen_height / 2.0 + 40.0,
            20.0,
            Color::from_rgba(150, 150, 150, 255)
        );
    }

    // Instructions
    let restart_text = "PRESS SPACE TO RESPAWN NOW | ESC FOR MENU";
    let restart_width = measure_text(restart_text, None, 20, 1.0).width;
    draw_text(
        restart_text,
        screen_width / 2.0 - restart_width / 2.0,
        screen_height / 2.0 + 80.0,
        20.0,
        Color::from_rgba(200, 200, 200, 255)
    );
}

fn draw_level_complete(score: u32) {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // Semi-transparent overlay
    draw_rectangle(0.0, 0.0, screen_width, screen_height, Color::from_rgba(0, 0, 0, 180));

    // Title
    let title_text = "LEVEL COMPLETE!";
    let text_width = measure_text(title_text, None, 60, 1.0).width;
    draw_text(
        title_text,
        screen_width / 2.0 - text_width / 2.0,
        screen_height / 2.0 - 60.0,
        60.0,
        Color::from_rgba(0, 255, 0, 255)
    );

    // Score
    let score_text = format!("SCORE: {:08}", score);
    let score_width = measure_text(&score_text, None, 40, 1.0).width;
    draw_text(
        &score_text,
        screen_width / 2.0 - score_width / 2.0,
        screen_height / 2.0 + 10.0,
        40.0,
        Color::from_rgba(0, 255, 255, 255)
    );

    // Continue text
    let continue_text = "PRESS SPACE TO CONTINUE";
    let continue_width = measure_text(continue_text, None, 25, 1.0).width;
    draw_text(
        continue_text,
        screen_width / 2.0 - continue_width / 2.0,
        screen_height / 2.0 + 80.0,
        25.0,
        Color::from_rgba(255, 255, 255, 255)
    );
}

fn draw_game_over(score: u32) {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // Semi-transparent overlay
    draw_rectangle(0.0, 0.0, screen_width, screen_height, Color::from_rgba(0, 0, 0, 180));

    // Retro game over text
    let game_over_text = "GAME OVER";
    let text_width = measure_text(game_over_text, None, 60, 1.0).width;
    draw_text(
        game_over_text,
        screen_width / 2.0 - text_width / 2.0,
        screen_height / 2.0 - 40.0,
        60.0,
        Color::from_rgba(255, 0, 0, 255)
    );

    let score_text = format!("FINAL SCORE: {:08}", score);
    let score_width = measure_text(&score_text, None, 30, 1.0).width;
    draw_text(
        &score_text,
        screen_width / 2.0 - score_width / 2.0,
        screen_height / 2.0 + 20.0,
        30.0,
        Color::from_rgba(0, 255, 255, 255)
    );

    let restart_text = "PRESS SPACE TO RESTART | ESC FOR MENU";
    let restart_width = measure_text(restart_text, None, 20, 1.0).width;
    draw_text(
        restart_text,
        screen_width / 2.0 - restart_width / 2.0,
        screen_height / 2.0 + 60.0,
        20.0,
        Color::from_rgba(255, 255, 255, 255)
    );
}

fn draw_placeholder_text(text: &str) {
    let screen_width = screen_width();
    let screen_height = screen_height();

    let text_width = measure_text(text, None, 30, 1.0).width;
    draw_text(
        text,
        screen_width / 2.0 - text_width / 2.0,
        screen_height / 2.0,
        30.0,
        Color::from_rgba(255, 255, 255, 255)
    );
}
