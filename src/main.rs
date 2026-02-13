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
mod ui;
mod save_system;
mod rings;
mod drone_companion;
mod clouds;
mod air_particles;

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
use ui::{SplashScreen, MainMenu, OptionsMenu, LevelSelectScreen, TutorialInstructions};
use rings::RingManager;
use drone_companion::DroneCompanion;
use clouds::CloudManager;
use air_particles::AirParticleSystem;
use save_system::SaveManager;

#[macroquad::main("Glide Wars")]
async fn main() {
    let mut scene_manager = SceneManager::new();
    let mut input_manager = InputManager::new();
    let asset_manager = AssetManager::default();
    let mut save_manager = SaveManager::new();

    // Game state
    let mut player = Player::new();
    let mut terrain = TerrainManager::new();
    let mut enemies = EnemyManager::new();
    let mut powerups = PowerupManager::new();
    let mut rings = RingManager::new();
    let mut drone = DroneCompanion::new();
    let mut clouds = CloudManager::new();
    let mut air_particles = AirParticleSystem::new();
    let mut camera = GameCamera::new();

    // Level management
    // UI screens
    let mut splash_screen = SplashScreen::new();
    let mut main_menu = MainMenu::new();
    let mut options_menu = OptionsMenu::new();
    let mut level_select_screen = LevelSelectScreen::new();
    let mut tutorial_instructions = TutorialInstructions::new();

    // Apply saved settings to options menu
    options_menu.set_from_settings(
        save_manager.data().settings.sound_volume,
        save_manager.data().settings.music_volume,
        save_manager.data().settings.difficulty as usize,
    );

    let mut level_manager: Option<LevelManager> = None;
    let mut checkpoint_manager = CheckpointManager::new();
    let mut boss: Option<Boss> = None;
    let mut current_continent = Continent::Tutorial;
    let mut level_select_synced = false;

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
                splash_screen.update(dt);
                splash_screen.draw();

                // Skip on any key press
                if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                    splash_screen.skip();
                }

                if splash_screen.is_completed() {
                    scene_manager.request_transition(GameState::MainMenu);
                }
            }

            GameState::MainMenu => {
                let action = main_menu.update(dt);
                main_menu.draw();

                match action {
                    ui::main_menu::MenuAction::Start => {
                        scene_manager.request_transition(GameState::LevelSelect);
                    }
                    ui::main_menu::MenuAction::Options => {
                        scene_manager.request_transition(GameState::Options);
                    }
                    _ => {}
                }
            }

            GameState::Options => {
                let action = options_menu.update(dt);
                options_menu.draw();

                match action {
                    ui::options::OptionsAction::Back => {
                        // Save settings
                        save_manager.data_mut().update_settings(
                            options_menu.get_sound_volume(),
                            options_menu.get_music_volume(),
                            options_menu.get_difficulty() as u8,
                        );
                        save_manager.auto_save();

                        scene_manager.request_transition(GameState::MainMenu);
                    }
                    _ => {}
                }
            }

            GameState::LevelSelect => {
                // Sync unlocked continents with save data on first entry
                if !level_select_synced {
                    level_select_screen.sync_with_save(&save_manager.data().unlocked_continents);
                    level_select_synced = true;
                }

                let action = level_select_screen.update(dt);
                level_select_screen.draw();

                match action {
                    ui::level_select::LevelSelectAction::StartLevel(continent) => {
                        current_continent = continent;
                        if continent == Continent::Tutorial {
                            // Show tutorial instructions first
                            tutorial_instructions.reset();
                            scene_manager.request_transition(GameState::TutorialInstructions);
                        } else {
                            scene_manager.request_transition(GameState::InGame);
                        }
                    }
                    ui::level_select::LevelSelectAction::Back => {
                        scene_manager.request_transition(GameState::MainMenu);
                    }
                    _ => {}
                }
            }

            GameState::TutorialInstructions => {
                let action = tutorial_instructions.update(dt);
                tutorial_instructions.draw();

                match action {
                    ui::tutorial_instructions::TutorialAction::Start => {
                        scene_manager.request_transition(GameState::Tutorial);
                    }
                    ui::tutorial_instructions::TutorialAction::Back => {
                        scene_manager.request_transition(GameState::LevelSelect);
                    }
                    _ => {}
                }
            }

            GameState::Tutorial | GameState::InGame => {
                // Initialize level manager if not present
                if level_manager.is_none() {
                    level_manager = Some(LevelManager::new(current_continent));
                    checkpoint_manager.clear();
                    rings.reset();
                    drone.deactivate();
                    clouds.clear();
                    air_particles.clear();
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
                    #[cfg(debug_assertions)]
                    println!("=== BOSS SPAWNED: {} at {:.1}s ===", boss_type.name(), level_mgr.elapsed_time());
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
                rings.update(dt, &player);
                drone.update(dt, &player);
                clouds.update(dt, &player);

                // Emit air particles when player moves vertically
                air_particles.emit(player.position(), player.velocity().y);
                air_particles.update(dt);

                let score = scene_manager.scene_data().score;
                let mut score_mut = score;
                powerups.update(dt, &player, &mut score_mut);
                scene_manager.scene_data_mut().score = score_mut;

                // Check collisions
                if terrain.check_collision(&player) || enemies.check_collision(&player) {
                    player.take_damage(10.0);
                    if player.is_dead() {
                        save_manager.data_mut().record_death();
                        checkpoint_manager.start_respawn();
                        scene_manager.request_transition(GameState::Checkpoint);
                    }
                }

                // Check powerup collection
                if let Some(powerup_type) = powerups.check_collection(&mut player, &mut scene_manager.scene_data_mut().score) {
                    if matches!(powerup_type, powerup::PowerupType::DroneCompanion) {
                        drone.activate(player.position());
                    }
                }

                // Check ring collection
                rings.check_collection(&player, &mut scene_manager.scene_data_mut().score);

                // Drone projectiles hit enemies
                if drone.is_active() {
                    let mut projectiles_to_remove = Vec::new();
                    let drone_projectiles = drone.get_projectiles();
                    for (idx, proj) in drone_projectiles.iter().enumerate() {
                        if enemies.check_projectile_hit(proj.position) {
                            projectiles_to_remove.push(idx);
                            scene_manager.scene_data_mut().score += 25;
                        }
                    }
                    // Remove collected projectiles (reverse order to preserve indices)
                    for idx in projectiles_to_remove.iter().rev() {
                        drone.clear_projectile(*idx);
                    }
                }

                // Update camera
                camera.update(&player);

                // Render 3D scene
                set_camera(camera.get_camera());

                clouds.draw(); // Background layer
                terrain.draw();
                enemies.draw();
                powerups.draw();
                rings.draw();
                drone.draw();
                air_particles.draw(); // Air trail effects
                player.draw();

                // Render 2D UI
                set_default_camera();

                draw_hud_with_level(&player, scene_manager.scene_data().score, level_mgr, &checkpoint_manager, &asset_manager, current_continent, &save_manager, &drone);

                // Back to menu
                if input.back {
                    scene_manager.request_transition(GameState::MainMenu);
                    level_manager = None;
                }
            }

            GameState::BossFight => {
                if let Some(ref mut boss_instance) = boss {
                    // Update level timer
                    if let Some(ref mut level_mgr) = level_manager {
                        level_mgr.update(dt, player.position().z);

                        // Check if level time is up (even if boss not defeated)
                        if level_mgr.is_complete() {
                            scene_manager.scene_data_mut().score += 5000; // Bonus for completing level
                            scene_manager.request_transition(GameState::LevelComplete);
                        }
                    }

                    // Update boss
                    boss_instance.update(dt, player.position(), player.velocity());

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

                        // Check if level is complete
                        if let Some(ref level_mgr) = level_manager {
                            if level_mgr.is_complete() {
                                // Level time is up, proceed to completion
                                scene_manager.request_transition(GameState::LevelComplete);
                            } else {
                                // Boss defeated but level continues, go back to InGame
                                #[cfg(debug_assertions)]
                                println!("Boss defeated! Continuing level...");
                                scene_manager.request_transition(GameState::InGame);
                            }
                        }
                    }

                    // Check if player died
                    if player.is_dead() {
                        save_manager.data_mut().record_death();
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
                        draw_hud_with_level(&player, scene_manager.scene_data().score, level_mgr, &checkpoint_manager, &asset_manager, current_continent, &save_manager, &drone);
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

                    // Deactivate drone companion on respawn
                    drone.deactivate();

                    // Reset terrain to checkpoint position and clear around player
                    if let Some(checkpoint_pos) = checkpoint_manager.get_last_checkpoint_position() {
                        terrain.reset_to_position(checkpoint_pos);
                        // Clear obstacles and enemies in a LARGE safe radius around spawn point
                        let clear_radius = 50.0; // Doubled safe zone radius for better recovery
                        terrain.clear_around_position(player.position(), clear_radius);
                        enemies.clear_around_position(player.position(), clear_radius);

                        // Pause spawning for 3 seconds after respawn
                        terrain.pause_spawning(3.0);
                        enemies.pause_spawning(3.0);
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

                    // Deactivate drone companion on respawn
                    drone.deactivate();

                    // Reset terrain to checkpoint position and clear around player
                    if let Some(checkpoint_pos) = checkpoint_manager.get_last_checkpoint_position() {
                        terrain.reset_to_position(checkpoint_pos);
                        // Clear obstacles and enemies in a LARGE safe radius around spawn point
                        let clear_radius = 50.0; // Doubled safe zone radius for better recovery
                        terrain.clear_around_position(player.position(), clear_radius);
                        enemies.clear_around_position(player.position(), clear_radius);

                        // Pause spawning for 3 seconds after respawn
                        terrain.pause_spawning(3.0);
                        enemies.pause_spawning(3.0);
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
                    // Save progress
                    let final_score = scene_manager.scene_data().score;
                    if let Some(ref lvl_mgr) = level_manager {
                        let completion_time = lvl_mgr.elapsed_time();

                        // Update high score
                        save_manager.data_mut().update_high_score(current_continent, final_score);

                        // Update best time
                        save_manager.data_mut().update_best_time(current_continent, completion_time);

                        // Unlock next continent
                        let next_continent = get_next_continent(current_continent);
                        if let Some(next) = next_continent {
                            save_manager.data_mut().unlock_continent(next);
                        }

                        // Record boss kill if defeated
                        if boss.as_ref().map_or(false, |b| b.is_defeated()) {
                            save_manager.data_mut().record_boss_kill();
                        }
                    }

                    // Save to disk
                    save_manager.auto_save();

                    // Reset flag so level select resyncs
                    level_select_synced = false;

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

        // Track play time during active gameplay
        if matches!(scene_manager.current_state(), GameState::InGame | GameState::Tutorial | GameState::BossFight) {
            save_manager.data_mut().add_play_time(dt);
        }

        next_frame().await
    }
}

fn get_next_continent(current: Continent) -> Option<Continent> {
    match current {
        Continent::Tutorial => Some(Continent::NorthAmerica),
        Continent::NorthAmerica => Some(Continent::SouthAmerica),
        Continent::SouthAmerica => Some(Continent::Europe),
        Continent::Europe => Some(Continent::Asia),
        Continent::Asia => Some(Continent::Africa),
        Continent::Africa => Some(Continent::Oceania),
        Continent::Oceania => None, // Last continent
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
    save_manager: &SaveManager,
    drone: &DroneCompanion,
) {
    let hud_color = Color::from_rgba(0, 255, 255, 255);
    let screen_w = screen_width();

    // === LEFT PANEL ===
    // Panel background
    draw_rectangle(10.0, 10.0, 220.0, 165.0, Color::from_rgba(0, 10, 20, 200));
    draw_rectangle_lines(10.0, 10.0, 220.0, 165.0, 2.0, hud_color);

    // Health bar
    draw_text("HEALTH", 20.0, 30.0, 18.0, hud_color);
    draw_rectangle(20.0, 35.0, 200.0, 15.0, Color::from_rgba(40, 40, 40, 255));
    let health_color = if player.health() > 50.0 {
        Color::from_rgba(0, 255, 0, 255)
    } else if player.health() > 25.0 {
        Color::from_rgba(255, 255, 0, 255)
    } else {
        Color::from_rgba(255, 0, 0, 255)
    };
    draw_rectangle(20.0, 35.0, player.health() * 2.0, 15.0, health_color);
    draw_rectangle_lines(20.0, 35.0, 200.0, 15.0, 2.0, hud_color);

    // Boost energy bar
    draw_text("BOOST", 20.0, 68.0, 18.0, hud_color);
    draw_rectangle(20.0, 73.0, 200.0, 12.0, Color::from_rgba(40, 40, 40, 255));
    let boost_percentage = player.boost_energy() / player.boost_max_energy();
    let boost_width = 200.0 * boost_percentage;
    let boost_color = if boost_percentage > 0.5 {
        Color::from_rgba(0, 200, 255, 255) // Cyan when full
    } else if boost_percentage > 0.25 {
        Color::from_rgba(100, 150, 255, 255) // Blue when medium
    } else {
        Color::from_rgba(150, 150, 150, 255) // Gray when low
    };
    draw_rectangle(20.0, 73.0, boost_width, 12.0, boost_color);
    draw_rectangle_lines(20.0, 73.0, 200.0, 12.0, 2.0, hud_color);

    // Score
    draw_text(&format!("SCORE: {:08}", score), 20.0, 100.0, 18.0, hud_color);

    // High score
    let high_score = save_manager.data().get_high_score(&continent);
    draw_text(&format!("HIGH:  {:08}", high_score), 20.0, 120.0, 16.0, Color::from_rgba(255, 215, 0, 255));

    // Weapon indicator
    if let Some(weapon) = player.current_weapon() {
        draw_text(&format!("WEAPON: {}", weapon), 20.0, 145.0, 16.0, hud_color);
        draw_text(&format!("AMMO: {}", player.ammo()), 20.0, 165.0, 16.0, hud_color);
    }

    // Drone companion indicator
    if drone.is_active() {
        let drone_y = if player.current_weapon().is_some() { 190.0 } else { 145.0 };
        let remaining = drone.remaining_time();
        let drone_color = if remaining > 15.0 {
            Color::from_rgba(0, 255, 100, 255)
        } else if remaining > 5.0 {
            Color::from_rgba(255, 255, 0, 255)
        } else {
            Color::from_rgba(255, 100, 0, 255)
        };
        draw_text(&format!("DRONE: {:.0}s", remaining), 20.0, drone_y, 16.0, drone_color);
    }

    // === TOP CENTER - CONTINENT PANEL ===
    let continent_text = continent.name();
    let text_width = measure_text(continent_text, None, 32, 1.0).width;
    let panel_width = text_width + 40.0;
    let panel_x = screen_w / 2.0 - panel_width / 2.0;

    draw_rectangle(panel_x, 10.0, panel_width, 50.0, Color::from_rgba(0, 10, 20, 200));
    draw_rectangle_lines(panel_x, 10.0, panel_width, 50.0, 2.0, hud_color);
    draw_text(
        continent_text,
        screen_w / 2.0 - text_width / 2.0,
        42.0,
        32.0,
        hud_color,
    );

    // === RIGHT PANEL - TIMER AND STATS ===
    let right_panel_width = 200.0;
    let right_panel_x = screen_w - right_panel_width - 10.0;

    draw_rectangle(right_panel_x, 10.0, right_panel_width, 120.0, Color::from_rgba(0, 10, 20, 200));
    draw_rectangle_lines(right_panel_x, 10.0, right_panel_width, 120.0, 2.0, hud_color);

    // Timer
    let remaining = level_manager.remaining_time();
    let minutes = (remaining / 60.0) as u32;
    let seconds = (remaining % 60.0) as u32;
    let timer_color = if remaining < 60.0 {
        Color::from_rgba(255, 0, 0, 255) // Red warning
    } else {
        hud_color
    };
    draw_text("TIME", right_panel_x + 10.0, 32.0, 18.0, hud_color);
    draw_text(
        &format!("{:02}:{:02}", minutes, seconds),
        right_panel_x + 10.0,
        55.0,
        28.0,
        timer_color,
    );

    // Checkpoint
    draw_text("CHECKPOINT", right_panel_x + 10.0, 80.0, 16.0, hud_color);
    draw_text(
        &format!("{}/{}", checkpoint_manager.checkpoint_count(), level_manager.total_checkpoints()),
        right_panel_x + 10.0,
        100.0,
        22.0,
        Color::from_rgba(255, 255, 0, 255),
    );

    // Best time (if exists)
    if let Some(best_time) = save_manager.data().get_best_time(&continent) {
        let best_mins = (best_time / 60.0) as u32;
        let best_secs = (best_time % 60.0) as u32;
        draw_text(
            &format!("BEST: {:02}:{:02}", best_mins, best_secs),
            right_panel_x + 10.0,
            120.0,
            14.0,
            Color::from_rgba(0, 255, 0, 255),
        );
    }
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

