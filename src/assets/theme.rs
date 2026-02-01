use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub player: PlayerModel,
    pub enemies: HashMap<String, EnemyModel>,
    pub ui: UIColorScheme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerModel {
    pub body_color: [u8; 3],
    pub wing_color: [u8; 3],
    pub size_multiplier: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemyModel {
    pub color: [u8; 3],
    pub size: [f32; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIColorScheme {
    pub primary_color: [u8; 3],
    pub secondary_color: [u8; 3],
    pub health_color: [u8; 3],
    pub danger_color: [u8; 3],
    pub text_color: [u8; 3],
}

impl Theme {
    pub fn default() -> Self {
        let mut enemies = HashMap::new();

        enemies.insert(
            "drone".to_string(),
            EnemyModel {
                color: [255, 100, 100],
                size: [0.5, 0.5, 0.8],
            },
        );

        enemies.insert(
            "seeker".to_string(),
            EnemyModel {
                color: [255, 0, 0],
                size: [0.7, 0.4, 0.7],
            },
        );

        enemies.insert(
            "zigzag".to_string(),
            EnemyModel {
                color: [255, 150, 0],
                size: [0.6, 0.3, 0.9],
            },
        );

        enemies.insert(
            "turret".to_string(),
            EnemyModel {
                color: [180, 0, 0],
                size: [0.8, 0.8, 0.8],
            },
        );

        Self {
            name: "Default".to_string(),
            player: PlayerModel {
                body_color: [0, 200, 255],
                wing_color: [0, 150, 200],
                size_multiplier: 1.0,
            },
            enemies,
            ui: UIColorScheme {
                primary_color: [0, 255, 255],
                secondary_color: [255, 255, 255],
                health_color: [0, 255, 0],
                danger_color: [255, 0, 0],
                text_color: [255, 255, 255],
            },
        }
    }

    pub fn neon() -> Self {
        let mut enemies = HashMap::new();

        enemies.insert(
            "drone".to_string(),
            EnemyModel {
                color: [255, 0, 255],
                size: [0.5, 0.5, 0.8],
            },
        );

        enemies.insert(
            "seeker".to_string(),
            EnemyModel {
                color: [255, 0, 128],
                size: [0.7, 0.4, 0.7],
            },
        );

        enemies.insert(
            "zigzag".to_string(),
            EnemyModel {
                color: [255, 128, 0],
                size: [0.6, 0.3, 0.9],
            },
        );

        enemies.insert(
            "turret".to_string(),
            EnemyModel {
                color: [200, 0, 255],
                size: [0.8, 0.8, 0.8],
            },
        );

        Self {
            name: "Neon".to_string(),
            player: PlayerModel {
                body_color: [0, 255, 255],
                wing_color: [255, 0, 255],
                size_multiplier: 1.0,
            },
            enemies,
            ui: UIColorScheme {
                primary_color: [0, 255, 255],
                secondary_color: [255, 0, 255],
                health_color: [0, 255, 0],
                danger_color: [255, 0, 0],
                text_color: [255, 255, 255],
            },
        }
    }

    pub fn classic() -> Self {
        let mut enemies = HashMap::new();

        enemies.insert(
            "drone".to_string(),
            EnemyModel {
                color: [200, 0, 0],
                size: [0.5, 0.5, 0.8],
            },
        );

        enemies.insert(
            "seeker".to_string(),
            EnemyModel {
                color: [180, 0, 0],
                size: [0.7, 0.4, 0.7],
            },
        );

        enemies.insert(
            "zigzag".to_string(),
            EnemyModel {
                color: [220, 100, 0],
                size: [0.6, 0.3, 0.9],
            },
        );

        enemies.insert(
            "turret".to_string(),
            EnemyModel {
                color: [150, 0, 0],
                size: [0.8, 0.8, 0.8],
            },
        );

        Self {
            name: "Classic".to_string(),
            player: PlayerModel {
                body_color: [0, 150, 255],
                wing_color: [0, 100, 200],
                size_multiplier: 1.0,
            },
            enemies,
            ui: UIColorScheme {
                primary_color: [0, 200, 255],
                secondary_color: [200, 200, 200],
                health_color: [0, 200, 0],
                danger_color: [200, 0, 0],
                text_color: [200, 200, 200],
            },
        }
    }

    pub fn minimal() -> Self {
        let mut enemies = HashMap::new();

        enemies.insert(
            "drone".to_string(),
            EnemyModel {
                color: [100, 100, 100],
                size: [0.5, 0.5, 0.8],
            },
        );

        enemies.insert(
            "seeker".to_string(),
            EnemyModel {
                color: [120, 120, 120],
                size: [0.7, 0.4, 0.7],
            },
        );

        enemies.insert(
            "zigzag".to_string(),
            EnemyModel {
                color: [140, 140, 140],
                size: [0.6, 0.3, 0.9],
            },
        );

        enemies.insert(
            "turret".to_string(),
            EnemyModel {
                color: [80, 80, 80],
                size: [0.8, 0.8, 0.8],
            },
        );

        Self {
            name: "Minimal".to_string(),
            player: PlayerModel {
                body_color: [200, 200, 200],
                wing_color: [150, 150, 150],
                size_multiplier: 1.0,
            },
            enemies,
            ui: UIColorScheme {
                primary_color: [200, 200, 200],
                secondary_color: [150, 150, 150],
                health_color: [180, 180, 180],
                danger_color: [100, 100, 100],
                text_color: [200, 200, 200],
            },
        }
    }

    pub fn get_player_body_color(&self) -> Color {
        Color::from_rgba(
            self.player.body_color[0],
            self.player.body_color[1],
            self.player.body_color[2],
            255,
        )
    }

    pub fn get_player_wing_color(&self) -> Color {
        Color::from_rgba(
            self.player.wing_color[0],
            self.player.wing_color[1],
            self.player.wing_color[2],
            255,
        )
    }

    pub fn get_enemy_color(&self, enemy_type: &str) -> Color {
        if let Some(enemy) = self.enemies.get(enemy_type) {
            Color::from_rgba(enemy.color[0], enemy.color[1], enemy.color[2], 255)
        } else {
            WHITE
        }
    }

    pub fn get_ui_primary_color(&self) -> Color {
        Color::from_rgba(
            self.ui.primary_color[0],
            self.ui.primary_color[1],
            self.ui.primary_color[2],
            255,
        )
    }

    pub fn get_ui_health_color(&self) -> Color {
        Color::from_rgba(
            self.ui.health_color[0],
            self.ui.health_color[1],
            self.ui.health_color[2],
            255,
        )
    }

    pub fn get_ui_danger_color(&self) -> Color {
        Color::from_rgba(
            self.ui.danger_color[0],
            self.ui.danger_color[1],
            self.ui.danger_color[2],
            255,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_theme() {
        let theme = Theme::default();
        assert_eq!(theme.name, "Default");
        assert_eq!(theme.enemies.len(), 4);
    }

    #[test]
    fn test_neon_theme() {
        let theme = Theme::neon();
        assert_eq!(theme.name, "Neon");
    }

    #[test]
    fn test_color_conversion() {
        let theme = Theme::default();
        let color = theme.get_player_body_color();
        assert_eq!(color.r, 0.0);
    }
}
