use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod theme;
pub mod loader;

use theme::{Theme, PlayerModel, EnemyModel, UIColorScheme};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Continent {
    Tutorial,
    NorthAmerica,
    SouthAmerica,
    Europe,
    Asia,
    Africa,
    Oceania,
}

impl Continent {
    pub fn name(&self) -> &str {
        match self {
            Continent::Tutorial => "Tutorial",
            Continent::NorthAmerica => "North America",
            Continent::SouthAmerica => "South America",
            Continent::Europe => "Europe",
            Continent::Asia => "Asia",
            Continent::Africa => "Africa",
            Continent::Oceania => "Oceania",
        }
    }

    pub fn all() -> Vec<Continent> {
        vec![
            Continent::Tutorial,
            Continent::NorthAmerica,
            Continent::SouthAmerica,
            Continent::Europe,
            Continent::Asia,
            Continent::Africa,
            Continent::Oceania,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundLayer {
    pub scroll_speed: f32,
    pub tint: [u8; 3],
    pub parallax_offset: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkyGradient {
    pub top: [u8; 3],
    pub bottom: [u8; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainTheme {
    pub primary_obstacles: Vec<String>,
    pub ground_color: [u8; 3],
    pub grid_color: [u8; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossModel {
    pub boss_type: String,
    pub model_scale: f32,
    pub primary_color: [u8; 3],
    pub accent_color: [u8; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinentAssets {
    pub name: String,
    pub sky_gradient: SkyGradient,
    pub background_layers: Vec<BackgroundLayer>,
    pub terrain: TerrainTheme,
    pub boss: BossModel,
}

pub struct AssetManager {
    themes: HashMap<String, Theme>,
    current_theme: String,
    continent_assets: HashMap<Continent, ContinentAssets>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            themes: HashMap::new(),
            current_theme: "default".to_string(),
            continent_assets: HashMap::new(),
        }
    }

    pub fn load_default_theme(&mut self) {
        let default_theme = Theme::default();
        self.themes.insert("default".to_string(), default_theme);
        self.current_theme = "default".to_string();
    }

    pub fn add_theme(&mut self, name: String, theme: Theme) {
        self.themes.insert(name, theme);
    }

    pub fn set_current_theme(&mut self, name: &str) -> bool {
        if self.themes.contains_key(name) {
            self.current_theme = name.to_string();
            true
        } else {
            false
        }
    }

    pub fn current_theme(&self) -> Option<&Theme> {
        self.themes.get(&self.current_theme)
    }

    pub fn theme_names(&self) -> Vec<&String> {
        self.themes.keys().collect()
    }

    pub fn add_continent_assets(&mut self, continent: Continent, assets: ContinentAssets) {
        self.continent_assets.insert(continent, assets);
    }

    pub fn get_continent_assets(&self, continent: &Continent) -> Option<&ContinentAssets> {
        self.continent_assets.get(continent)
    }

    pub fn load_default_continent_assets(&mut self) {
        // Load default assets for all continents
        for continent in Continent::all() {
            let assets = ContinentAssets::default_for_continent(&continent);
            self.add_continent_assets(continent, assets);
        }
    }
}

impl ContinentAssets {
    pub fn default_for_continent(continent: &Continent) -> Self {
        match continent {
            Continent::Tutorial => Self {
                name: "Tutorial".to_string(),
                sky_gradient: SkyGradient {
                    top: [100, 100, 150],
                    bottom: [150, 150, 200],
                },
                background_layers: vec![
                    BackgroundLayer {
                        scroll_speed: 0.1,
                        tint: [200, 200, 220],
                        parallax_offset: 0.0,
                    },
                ],
                terrain: TerrainTheme {
                    primary_obstacles: vec!["simple_block".to_string()],
                    ground_color: [100, 100, 100],
                    grid_color: [80, 80, 80],
                },
                boss: BossModel {
                    boss_type: "TutorialBoss".to_string(),
                    model_scale: 2.0,
                    primary_color: [150, 150, 150],
                    accent_color: [200, 200, 200],
                },
            },
            Continent::NorthAmerica => Self {
                name: "North America".to_string(),
                sky_gradient: SkyGradient {
                    top: [135, 206, 235],
                    bottom: [255, 165, 0],
                },
                background_layers: vec![
                    BackgroundLayer {
                        scroll_speed: 0.1,
                        tint: [100, 100, 150],
                        parallax_offset: 0.0,
                    },
                    BackgroundLayer {
                        scroll_speed: 0.3,
                        tint: [120, 120, 140],
                        parallax_offset: 0.0,
                    },
                ],
                terrain: TerrainTheme {
                    primary_obstacles: vec!["mountain".to_string(), "boulder".to_string()],
                    ground_color: [34, 139, 34],
                    grid_color: [0, 100, 0],
                },
                boss: BossModel {
                    boss_type: "MountainGuardian".to_string(),
                    model_scale: 3.0,
                    primary_color: [139, 90, 43],
                    accent_color: [169, 169, 169],
                },
            },
            Continent::SouthAmerica => Self {
                name: "South America".to_string(),
                sky_gradient: SkyGradient {
                    top: [100, 200, 255],
                    bottom: [50, 150, 50],
                },
                background_layers: vec![
                    BackgroundLayer {
                        scroll_speed: 0.1,
                        tint: [50, 150, 50],
                        parallax_offset: 0.0,
                    },
                ],
                terrain: TerrainTheme {
                    primary_obstacles: vec!["tree".to_string(), "vine".to_string()],
                    ground_color: [20, 100, 20],
                    grid_color: [10, 80, 10],
                },
                boss: BossModel {
                    boss_type: "JungleBehemoth".to_string(),
                    model_scale: 3.5,
                    primary_color: [50, 150, 50],
                    accent_color: [100, 200, 100],
                },
            },
            Continent::Europe => Self {
                name: "Europe".to_string(),
                sky_gradient: SkyGradient {
                    top: [150, 150, 180],
                    bottom: [100, 100, 120],
                },
                background_layers: vec![
                    BackgroundLayer {
                        scroll_speed: 0.15,
                        tint: [120, 120, 150],
                        parallax_offset: 0.0,
                    },
                ],
                terrain: TerrainTheme {
                    primary_obstacles: vec!["building".to_string(), "tower".to_string()],
                    ground_color: [100, 100, 120],
                    grid_color: [80, 80, 100],
                },
                boss: BossModel {
                    boss_type: "StormBringer".to_string(),
                    model_scale: 3.2,
                    primary_color: [150, 150, 180],
                    accent_color: [200, 200, 255],
                },
            },
            Continent::Asia => Self {
                name: "Asia".to_string(),
                sky_gradient: SkyGradient {
                    top: [255, 200, 150],
                    bottom: [255, 100, 100],
                },
                background_layers: vec![
                    BackgroundLayer {
                        scroll_speed: 0.1,
                        tint: [255, 150, 100],
                        parallax_offset: 0.0,
                    },
                ],
                terrain: TerrainTheme {
                    primary_obstacles: vec!["pagoda".to_string(), "peak".to_string()],
                    ground_color: [200, 150, 100],
                    grid_color: [180, 130, 80],
                },
                boss: BossModel {
                    boss_type: "DragonKite".to_string(),
                    model_scale: 3.8,
                    primary_color: [255, 200, 50],
                    accent_color: [255, 100, 100],
                },
            },
            Continent::Africa => Self {
                name: "Africa".to_string(),
                sky_gradient: SkyGradient {
                    top: [255, 180, 100],
                    bottom: [255, 100, 50],
                },
                background_layers: vec![
                    BackgroundLayer {
                        scroll_speed: 0.12,
                        tint: [200, 150, 100],
                        parallax_offset: 0.0,
                    },
                ],
                terrain: TerrainTheme {
                    primary_obstacles: vec!["dune".to_string(), "acacia".to_string()],
                    ground_color: [220, 180, 120],
                    grid_color: [200, 160, 100],
                },
                boss: BossModel {
                    boss_type: "DesertPhoenix".to_string(),
                    model_scale: 3.5,
                    primary_color: [255, 150, 50],
                    accent_color: [255, 200, 100],
                },
            },
            Continent::Oceania => Self {
                name: "Oceania".to_string(),
                sky_gradient: SkyGradient {
                    top: [100, 200, 255],
                    bottom: [50, 150, 255],
                },
                background_layers: vec![
                    BackgroundLayer {
                        scroll_speed: 0.08,
                        tint: [100, 180, 230],
                        parallax_offset: 0.0,
                    },
                ],
                terrain: TerrainTheme {
                    primary_obstacles: vec!["wave".to_string(), "reef".to_string()],
                    ground_color: [50, 150, 200],
                    grid_color: [30, 130, 180],
                },
                boss: BossModel {
                    boss_type: "TidalWave".to_string(),
                    model_scale: 4.0,
                    primary_color: [50, 150, 255],
                    accent_color: [100, 200, 255],
                },
            },
        }
    }
}

impl Default for AssetManager {
    fn default() -> Self {
        let mut manager = Self::new();
        manager.load_default_theme();
        manager.load_default_continent_assets();
        manager
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_continent_names() {
        assert_eq!(Continent::Tutorial.name(), "Tutorial");
        assert_eq!(Continent::NorthAmerica.name(), "North America");
    }

    #[test]
    fn test_all_continents() {
        let continents = Continent::all();
        assert_eq!(continents.len(), 7);
    }

    #[test]
    fn test_asset_manager_creation() {
        let manager = AssetManager::default();
        assert!(manager.current_theme().is_some());
        assert_eq!(manager.theme_names().len(), 1);
    }

    #[test]
    fn test_continent_assets_loading() {
        let manager = AssetManager::default();
        for continent in Continent::all() {
            assert!(manager.get_continent_assets(&continent).is_some());
        }
    }

    #[test]
    fn test_theme_switching() {
        let mut manager = AssetManager::default();
        let new_theme = Theme::default();
        manager.add_theme("test_theme".to_string(), new_theme);

        assert!(manager.set_current_theme("test_theme"));
        assert_eq!(manager.theme_names().len(), 2);
    }
}
