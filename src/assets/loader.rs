use std::path::PathBuf;
use serde_json;

use super::{Theme, ContinentAssets, Continent};

#[derive(Debug)]
pub enum AssetError {
    FileNotFound(String),
    ParseError(String),
    InvalidData(String),
}

impl std::fmt::Display for AssetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AssetError::FileNotFound(path) => write!(f, "File not found: {}", path),
            AssetError::ParseError(err) => write!(f, "Parse error: {}", err),
            AssetError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
        }
    }
}

impl std::error::Error for AssetError {}

pub struct AssetLoader {
    assets_path: PathBuf,
}

impl AssetLoader {
    pub fn new<P: Into<PathBuf>>(assets_path: P) -> Self {
        Self {
            assets_path: assets_path.into(),
        }
    }

    pub fn load_theme(&self, name: &str) -> Result<Theme, AssetError> {
        let path = self.assets_path.join("themes").join(format!("{}.json", name));

        #[cfg(not(target_arch = "wasm32"))]
        {
            let contents = std::fs::read_to_string(&path)
                .map_err(|_| AssetError::FileNotFound(path.display().to_string()))?;

            let theme: Theme = serde_json::from_str(&contents)
                .map_err(|e| AssetError::ParseError(e.to_string()))?;

            Ok(theme)
        }

        #[cfg(target_arch = "wasm32")]
        {
            // For WASM, themes would need to be embedded or fetched via HTTP
            // For now, return built-in themes
            match name {
                "default" => Ok(Theme::default()),
                "neon" => Ok(Theme::neon()),
                "classic" => Ok(Theme::classic()),
                "minimal" => Ok(Theme::minimal()),
                _ => Err(AssetError::FileNotFound(format!("Theme not found: {}", name))),
            }
        }
    }

    pub fn load_continent(&self, continent: Continent) -> Result<ContinentAssets, AssetError> {
        let filename = match continent {
            Continent::Tutorial => "tutorial",
            Continent::NorthAmerica => "north_america",
            Continent::SouthAmerica => "south_america",
            Continent::Europe => "europe",
            Continent::Asia => "asia",
            Continent::Africa => "africa",
            Continent::Oceania => "oceania",
        };

        let path = self.assets_path.join("continents").join(format!("{}.json", filename));

        #[cfg(not(target_arch = "wasm32"))]
        {
            let contents = std::fs::read_to_string(&path)
                .map_err(|_| AssetError::FileNotFound(path.display().to_string()))?;

            let assets: ContinentAssets = serde_json::from_str(&contents)
                .map_err(|e| AssetError::ParseError(e.to_string()))?;

            Ok(assets)
        }

        #[cfg(target_arch = "wasm32")]
        {
            // For WASM, return default continent assets
            Ok(ContinentAssets::default_for_continent(&continent))
        }
    }

    pub fn validate_assets(&self) -> Vec<AssetError> {
        let mut errors = Vec::new();

        // Validate all built-in themes
        let theme_names = ["default", "neon", "classic", "minimal"];
        for name in theme_names {
            if let Err(e) = self.load_theme(name) {
                errors.push(e);
            }
        }

        // Validate all continent assets
        for continent in Continent::all() {
            if let Err(e) = self.load_continent(continent) {
                errors.push(e);
            }
        }

        errors
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn hot_reload(&self) -> Result<(), AssetError> {
        // Check if any asset files have been modified
        // This would require tracking file modification times
        // Simplified version for now
        Ok(())
    }
}

impl Default for AssetLoader {
    fn default() -> Self {
        Self::new("assets")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_loader_creation() {
        let loader = AssetLoader::new("assets");
        assert_eq!(loader.assets_path.to_str().unwrap(), "assets");
    }

    #[test]
    #[cfg(target_arch = "wasm32")]
    fn test_load_builtin_theme() {
        let loader = AssetLoader::default();
        assert!(loader.load_theme("default").is_ok());
        assert!(loader.load_theme("neon").is_ok());
    }

    #[test]
    fn test_validate_assets() {
        let loader = AssetLoader::default();
        let errors = loader.validate_assets();
        // In WASM or with default built-ins, should have no errors
        #[cfg(target_arch = "wasm32")]
        assert_eq!(errors.len(), 0);
    }
}
