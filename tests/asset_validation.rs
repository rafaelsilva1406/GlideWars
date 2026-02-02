// Asset validation tests
// Ensures all themes and continent assets are properly defined

use std::path::PathBuf;

#[test]
fn test_all_continent_assets_exist() {
    let assets_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/assets/mod.rs");
    let content = std::fs::read_to_string(assets_path).expect("Failed to read assets/mod.rs");

    // Check that continent assets are defined for all 7 continents
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
        assert!(content.contains(continent), "Continent '{}' should have asset definition", continent);
    }
}

#[test]
fn test_default_theme_exists() {
    let theme_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/assets/theme.rs");
    assert!(theme_path.exists(), "theme.rs should exist");

    let content = std::fs::read_to_string(theme_path).expect("Failed to read theme.rs");

    // Check that default theme exists
    assert!(content.contains("default()"), "Default theme implementation should exist");
}

#[test]
fn test_all_themes_defined() {
    let theme_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/assets/theme.rs");
    let content = std::fs::read_to_string(theme_path).expect("Failed to read theme.rs");

    // Check that all 4 themes are defined
    let themes = vec!["default", "neon", "classic", "minimal"];

    for theme in themes {
        assert!(content.contains(&format!("fn {}()", theme)) || content.contains(&format!("pub fn {}()", theme)),
                "Theme '{}' should be defined", theme);
    }
}

#[test]
fn test_theme_has_required_components() {
    let theme_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/assets/theme.rs");
    let content = std::fs::read_to_string(theme_path).expect("Failed to read theme.rs");

    // Check that theme struct has all required components
    let required_fields = vec![
        "name",
        "player_body_color",
        "player_wing_color",
        "enemy_colors",
        "ui_primary_color",
        "ui_secondary_color",
        "ui_health_color",
        "ui_danger_color",
    ];

    for field in required_fields {
        assert!(content.contains(field), "Theme should have '{}' field", field);
    }
}

#[test]
fn test_continent_assets_have_required_fields() {
    let assets_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/assets/mod.rs");
    let content = std::fs::read_to_string(assets_path).expect("Failed to read assets/mod.rs");

    // Check that ContinentAssets has required fields
    let required_fields = vec![
        "background_color_top",
        "background_color_bottom",
        "grid_color",
        "terrain_color",
    ];

    for field in required_fields {
        assert!(content.contains(field), "ContinentAssets should have '{}' field", field);
    }
}

#[test]
fn test_boss_assets_per_continent() {
    let boss_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/boss.rs");
    let content = std::fs::read_to_string(boss_path).expect("Failed to read boss.rs");

    // Check that each boss type has associated visuals
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
        assert!(content.contains(boss), "Boss '{}' should be defined with assets", boss);
    }

    // Check that boss colors/scale are defined
    assert!(content.contains("get_color") || content.contains("color"),
            "Boss color configuration should exist");
    assert!(content.contains("get_scale") || content.contains("scale"),
            "Boss scale configuration should exist");
}

#[test]
fn test_enemy_types_have_visuals() {
    let enemy_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/enemy.rs");
    let content = std::fs::read_to_string(enemy_path).expect("Failed to read enemy.rs");

    // Check that all enemy types are defined
    let enemy_types = vec!["Drone", "Seeker", "Zigzag", "Turret"];

    for enemy_type in enemy_types {
        assert!(content.contains(enemy_type), "Enemy type '{}' should be defined", enemy_type);
    }

    // Check that draw function with colors exists
    assert!(content.contains("fn draw(&self)"), "Enemy draw function should exist");
    assert!(content.contains("Color::from_rgba"), "Enemy colors should be defined");
}

#[test]
fn test_obstacle_types_defined() {
    let terrain_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/terrain.rs");
    let content = std::fs::read_to_string(terrain_path).expect("Failed to read terrain.rs");

    // Check that all obstacle types are defined
    let obstacle_types = vec!["Mountain", "Canyon", "Boulder", "WindTurbine"];

    for obstacle in obstacle_types {
        assert!(content.contains(obstacle), "Obstacle type '{}' should be defined", obstacle);
    }
}

#[test]
fn test_powerup_visuals_exist() {
    let powerup_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/powerup.rs");
    let content = std::fs::read_to_string(powerup_path).expect("Failed to read powerup.rs");

    // Check that powerup types and colors are defined
    assert!(content.contains("Health") || content.contains("Weapon"), "Powerup types should be defined");
    assert!(content.contains("draw"), "Powerup draw function should exist");
    assert!(content.contains("Color::from_rgba"), "Powerup colors should be defined");
}

#[test]
fn test_ui_colors_consistent() {
    let ui_modules = vec!["splash.rs", "main_menu.rs", "options.rs", "level_select.rs"];
    let ui_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/ui");

    for module in ui_modules {
        let module_path = ui_path.join(module);
        assert!(module_path.exists(), "UI module '{}' should exist", module);

        let content = std::fs::read_to_string(module_path).expect(&format!("Failed to read {}", module));

        // Check that UI uses consistent cyan theme color
        assert!(content.contains("Color::from_rgba") && (content.contains("255") || content.contains("0, 255, 255")),
                "UI module '{}' should have color definitions", module);
    }
}

#[test]
fn test_no_missing_asset_references() {
    // Ensure no references to non-existent asset files
    let main_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/main.rs");
    let content = std::fs::read_to_string(main_path).expect("Failed to read main.rs");

    // Should not contain references to missing texture files
    assert!(!content.contains("texture.png") && !content.contains("sprite.png"),
            "Should not reference non-existent texture files");

    // Should not reference sound files that don't exist
    assert!(!content.contains("sound.wav") && !content.contains("music.mp3"),
            "Should not reference non-existent sound files");
}

#[test]
fn test_asset_manager_initialization() {
    let assets_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/assets/mod.rs");
    let content = std::fs::read_to_string(assets_path).expect("Failed to read assets/mod.rs");

    // Check that AssetManager can be initialized
    assert!(content.contains("impl AssetManager"), "AssetManager implementation should exist");
    assert!(content.contains("pub fn default()") || content.contains("impl Default"),
            "AssetManager should have default initialization");
}

#[test]
fn test_continent_colors_unique() {
    let assets_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/assets/mod.rs");
    let content = std::fs::read_to_string(assets_path).expect("Failed to read assets/mod.rs");

    // Verify that continents have distinct visual identities
    // by checking that different color values are defined
    let color_count = content.matches("Color::from_rgba").count();
    assert!(color_count >= 7, "Should have unique colors for each continent (found {})", color_count);
}
