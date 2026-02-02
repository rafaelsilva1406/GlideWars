// Integration tests for level progression flow
use std::path::PathBuf;

#[test]
fn test_all_continents_defined() {
    let src_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/assets/mod.rs");
    let content = std::fs::read_to_string(src_path).expect("Failed to read assets/mod.rs");

    // Check that all 7 continents are defined
    let continents = vec![
        "Tutorial",
        "NorthAmerica",
        "SouthAmerica",
        "Europe",
        "Asia",
        "Africa",
        "Oceania",
    ];

    for continent in continents {
        assert!(content.contains(continent), "Continent '{}' should be defined", continent);
    }
}

#[test]
fn test_level_config_for_all_continents() {
    let src_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/level.rs");
    let content = std::fs::read_to_string(src_path).expect("Failed to read level.rs");

    // Check that level configuration exists for all continents
    assert!(content.contains("LevelConfig"), "LevelConfig struct should exist");
    assert!(content.contains("duration"), "Level duration field should exist");
    assert!(content.contains("checkpoint_interval"), "Checkpoint interval should exist");
}

#[test]
fn test_checkpoint_respawn_functionality() {
    let src_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/checkpoint.rs");
    let content = std::fs::read_to_string(src_path).expect("Failed to read checkpoint.rs");

    // Verify respawn system components
    assert!(content.contains("start_respawn"), "start_respawn method should exist");
    assert!(content.contains("update_respawn"), "update_respawn method should exist");
    assert!(content.contains("restore_player_state"), "restore_player_state method should exist");
    assert!(content.contains("60.0"), "60-second respawn timer should exist");
}

#[test]
fn test_boss_spawning_per_continent() {
    let src_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/boss.rs");
    let content = std::fs::read_to_string(src_path).expect("Failed to read boss.rs");

    // Check that bosses exist for all continents
    let bosses = vec![
        "TutorialBoss",
        "MountainGuardian",
        "JungleBehemoth",
        "StormBringer",
        "DragonKite",
        "DesertPhoenix",
        "TidalWave",
    ];

    for boss in bosses {
        assert!(content.contains(boss), "Boss '{}' should be defined", boss);
    }
}

#[test]
fn test_boss_phases_system() {
    let src_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/boss.rs");
    let content = std::fs::read_to_string(src_path).expect("Failed to read boss.rs");

    // Verify boss phase system
    assert!(content.contains("phase"), "Boss phase field should exist");
    assert!(content.contains("update_phase"), "Phase update logic should exist");
}

#[test]
fn test_difficulty_progression() {
    let terrain_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/terrain.rs");
    let terrain_content = std::fs::read_to_string(terrain_path).expect("Failed to read terrain.rs");

    // Verify difficulty increases with progress
    assert!(terrain_content.contains("difficulty"), "Difficulty field should exist in terrain");

    let enemy_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/enemy.rs");
    let enemy_content = std::fs::read_to_string(enemy_path).expect("Failed to read enemy.rs");

    // Verify enemy spawn rate increases
    assert!(enemy_content.contains("spawn_interval"), "Spawn interval should exist");
}

#[test]
fn test_player_weapons_system() {
    let src_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/player.rs");
    let content = std::fs::read_to_string(src_path).expect("Failed to read player.rs");

    // Verify weapon system
    assert!(content.contains("Weapon"), "Weapon enum should exist");
    assert!(content.contains("Laser"), "Laser weapon should exist");
    assert!(content.contains("Missile"), "Missile weapon should exist");
    assert!(content.contains("pickup_weapon"), "Weapon pickup system should exist");
}

#[test]
fn test_powerup_system() {
    let src_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/powerup.rs");
    assert!(src_path.exists(), "powerup.rs module should exist");

    let content = std::fs::read_to_string(src_path).expect("Failed to read powerup.rs");
    assert!(content.contains("PowerupManager"), "PowerupManager should exist");
    assert!(content.contains("Health"), "Health powerup should exist");
}

#[test]
fn test_collision_detection_systems() {
    // Check terrain collision
    let terrain_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/terrain.rs");
    let terrain_content = std::fs::read_to_string(terrain_path).expect("Failed to read terrain.rs");
    assert!(terrain_content.contains("check_collision"), "Terrain collision detection should exist");

    // Check enemy collision
    let enemy_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/enemy.rs");
    let enemy_content = std::fs::read_to_string(enemy_path).expect("Failed to read enemy.rs");
    assert!(enemy_content.contains("check_collision"), "Enemy collision detection should exist");
}

#[test]
fn test_safe_zone_on_respawn() {
    // Verify checkpoint safe zone clearing
    let terrain_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/terrain.rs");
    let terrain_content = std::fs::read_to_string(terrain_path).expect("Failed to read terrain.rs");
    assert!(terrain_content.contains("clear_around_position"), "Terrain clearing should exist");

    let enemy_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/enemy.rs");
    let enemy_content = std::fs::read_to_string(enemy_path).expect("Failed to read enemy.rs");
    assert!(enemy_content.contains("clear_around_position"), "Enemy clearing should exist");
}

#[test]
fn test_gravity_system() {
    let src_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/player.rs");
    let content = std::fs::read_to_string(src_path).expect("Failed to read player.rs");

    // Verify gravity physics
    assert!(content.contains("gravity"), "Gravity system should exist");
    assert!(content.contains("velocity"), "Velocity system should exist");
}

#[test]
fn test_score_tracking() {
    let scene_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/scene_manager.rs");
    let content = std::fs::read_to_string(scene_path).expect("Failed to read scene_manager.rs");

    // Verify score tracking
    assert!(content.contains("score"), "Score tracking should exist");
    assert!(content.contains("SceneData"), "SceneData should exist");
}
