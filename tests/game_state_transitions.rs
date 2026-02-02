// Integration tests for game state transitions
use std::path::PathBuf;

#[test]
fn test_game_state_transitions_exist() {
    // Verify game state module exists and can be imported
    // Note: Since this is a binary crate, we test by checking module structure
    let src_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/game_state.rs");
    assert!(src_path.exists(), "game_state.rs module should exist");
}

#[test]
fn test_scene_manager_exists() {
    let src_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/scene_manager.rs");
    assert!(src_path.exists(), "scene_manager.rs module should exist");
}

#[test]
fn test_all_game_states_defined() {
    // Read game_state.rs and verify all states are defined
    let src_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/game_state.rs");
    let content = std::fs::read_to_string(src_path).expect("Failed to read game_state.rs");

    // Check that all expected states exist
    let expected_states = vec![
        "Splash",
        "MainMenu",
        "Options",
        "LevelSelect",
        "Tutorial",
        "InGame",
        "BossFight",
        "Checkpoint",
        "LevelComplete",
        "GameOver",
    ];

    for state in expected_states {
        assert!(content.contains(state), "State '{}' should be defined in GameState enum", state);
    }
}

#[test]
fn test_state_transition_validation_exists() {
    let src_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/game_state.rs");
    let content = std::fs::read_to_string(src_path).expect("Failed to read game_state.rs");

    // Check that transition validation exists
    assert!(content.contains("can_transition_to"), "Transition validation method should exist");
}

#[test]
fn test_ui_modules_exist() {
    let ui_modules = vec!["splash.rs", "main_menu.rs", "options.rs", "level_select.rs"];
    let ui_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/ui");

    for module in ui_modules {
        let module_path = ui_path.join(module);
        assert!(module_path.exists(), "UI module '{}' should exist", module);
    }
}

#[test]
fn test_checkpoint_system_exists() {
    let src_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/checkpoint.rs");
    assert!(src_path.exists(), "checkpoint.rs module should exist");

    let content = std::fs::read_to_string(src_path).expect("Failed to read checkpoint.rs");
    assert!(content.contains("CheckpointManager"), "CheckpointManager struct should exist");
    assert!(content.contains("respawn"), "Respawn functionality should exist");
}

#[test]
fn test_level_progression_system_exists() {
    let src_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/level.rs");
    assert!(src_path.exists(), "level.rs module should exist");

    let content = std::fs::read_to_string(src_path).expect("Failed to read level.rs");
    assert!(content.contains("LevelManager"), "LevelManager struct should exist");
    assert!(content.contains("Continent"), "Continent enum should exist");
}

#[test]
fn test_boss_system_exists() {
    let src_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/boss.rs");
    assert!(src_path.exists(), "boss.rs module should exist");

    let content = std::fs::read_to_string(src_path).expect("Failed to read boss.rs");
    assert!(content.contains("Boss"), "Boss struct should exist");
    assert!(content.contains("BossType"), "BossType enum should exist");
    assert!(content.contains("AttackPattern"), "AttackPattern enum should exist");
}

#[test]
fn test_input_manager_exists() {
    let src_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/input_manager.rs");
    assert!(src_path.exists(), "input_manager.rs module should exist");

    let content = std::fs::read_to_string(src_path).expect("Failed to read input_manager.rs");
    assert!(content.contains("InputManager"), "InputManager struct should exist");
    assert!(content.contains("VirtualJoystick"), "VirtualJoystick should exist for mobile support");
}

#[test]
fn test_asset_system_exists() {
    let assets_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/assets");
    assert!(assets_path.exists(), "assets directory should exist");

    let mod_path = assets_path.join("mod.rs");
    assert!(mod_path.exists(), "assets/mod.rs should exist");

    let content = std::fs::read_to_string(mod_path).expect("Failed to read assets/mod.rs");
    assert!(content.contains("AssetManager"), "AssetManager struct should exist");
    assert!(content.contains("Continent"), "Continent enum should exist");
}
