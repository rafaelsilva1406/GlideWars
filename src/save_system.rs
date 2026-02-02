use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::assets::Continent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub player_name: Option<String>,
    pub unlocked_continents: Vec<String>,
    pub high_scores: HashMap<String, u32>,
    pub best_times: HashMap<String, f32>,
    pub settings: Settings,
    pub total_play_time: f32,
    pub total_deaths: u32,
    pub total_boss_kills: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub sound_volume: f32,
    pub music_volume: f32,
    pub difficulty: u8, // 0=Easy, 1=Normal, 2=Hard
}

impl Default for SaveData {
    fn default() -> Self {
        let mut unlocked = Vec::new();
        unlocked.push("Tutorial".to_string()); // Tutorial always unlocked

        Self {
            player_name: None,
            unlocked_continents: unlocked,
            high_scores: HashMap::new(),
            best_times: HashMap::new(),
            settings: Settings::default(),
            total_play_time: 0.0,
            total_deaths: 0,
            total_boss_kills: 0,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            sound_volume: 100.0,
            music_volume: 100.0,
            difficulty: 1, // Normal
        }
    }
}

impl SaveData {
    /// Check if a continent is unlocked
    pub fn is_continent_unlocked(&self, continent: &Continent) -> bool {
        let continent_name = continent.name();
        self.unlocked_continents.contains(&continent_name.to_string())
    }

    /// Unlock a continent
    pub fn unlock_continent(&mut self, continent: Continent) {
        let continent_name = continent.name().to_string();
        if !self.unlocked_continents.contains(&continent_name) {
            self.unlocked_continents.push(continent_name);
            println!("Unlocked continent: {}", continent.name());
        }
    }

    /// Update high score for a continent
    pub fn update_high_score(&mut self, continent: Continent, score: u32) -> bool {
        let continent_name = continent.name().to_string();
        let current_high = self.high_scores.get(&continent_name).copied().unwrap_or(0);

        if score > current_high {
            self.high_scores.insert(continent_name.clone(), score);
            println!("New high score for {}: {}", continent_name, score);
            true
        } else {
            false
        }
    }

    /// Update best time for a continent
    pub fn update_best_time(&mut self, continent: Continent, time: f32) -> bool {
        let continent_name = continent.name().to_string();
        let current_best = self.best_times.get(&continent_name).copied().unwrap_or(f32::MAX);

        if time < current_best {
            self.best_times.insert(continent_name.clone(), time);
            println!("New best time for {}: {:.1}s", continent_name, time);
            true
        } else {
            false
        }
    }

    /// Get high score for a continent
    pub fn get_high_score(&self, continent: &Continent) -> u32 {
        let continent_name = continent.name().to_string();
        self.high_scores.get(&continent_name).copied().unwrap_or(0)
    }

    /// Get best time for a continent
    pub fn get_best_time(&self, continent: &Continent) -> Option<f32> {
        let continent_name = continent.name().to_string();
        self.best_times.get(&continent_name).copied()
    }

    /// Update settings
    pub fn update_settings(&mut self, sound_volume: f32, music_volume: f32, difficulty: u8) {
        self.settings.sound_volume = sound_volume;
        self.settings.music_volume = music_volume;
        self.settings.difficulty = difficulty;
    }

    /// Add play time
    pub fn add_play_time(&mut self, time: f32) {
        self.total_play_time += time;
    }

    /// Increment death counter
    pub fn record_death(&mut self) {
        self.total_deaths += 1;
    }

    /// Increment boss kill counter
    pub fn record_boss_kill(&mut self) {
        self.total_boss_kills += 1;
    }
}

pub struct SaveManager {
    save_path: PathBuf,
    current_save: SaveData,
}

impl SaveManager {
    pub fn new() -> Self {
        let save_path = Self::get_save_path();

        // Try to load existing save, otherwise use default
        let current_save = Self::load_from_path(&save_path).unwrap_or_default();

        Self {
            save_path,
            current_save,
        }
    }

    /// Get the save file path (platform-specific)
    fn get_save_path() -> PathBuf {
        // For desktop: use user data directory
        // For web: this will be handled differently (localStorage)
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
            path.push(".glidewars");
            fs::create_dir_all(&path).ok();
            path.push("save.json");
            path
        }

        #[cfg(target_arch = "wasm32")]
        {
            // For WASM, we'll use localStorage through JS
            PathBuf::from("save.json")
        }
    }

    /// Load save data from file
    fn load_from_path(path: &PathBuf) -> Result<SaveData, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let save_data = serde_json::from_str(&contents)?;
        println!("Loaded save from: {}", path.display());
        Ok(save_data)
    }

    /// Save data to file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self.current_save)?;
        fs::write(&self.save_path, json)?;
        println!("Saved game to: {}", self.save_path.display());
        Ok(())
    }

    /// Get reference to current save data
    pub fn data(&self) -> &SaveData {
        &self.current_save
    }

    /// Get mutable reference to current save data
    pub fn data_mut(&mut self) -> &mut SaveData {
        &mut self.current_save
    }

    /// Auto-save (called periodically)
    pub fn auto_save(&self) {
        if let Err(e) = self.save() {
            eprintln!("Auto-save failed: {}", e);
        }
    }
}

impl Default for SaveManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_data_default() {
        let save = SaveData::default();
        assert_eq!(save.unlocked_continents.len(), 1);
        assert!(save.unlocked_continents.contains(&"Tutorial".to_string()));
        assert_eq!(save.settings.difficulty, 1);
        assert_eq!(save.settings.sound_volume, 100.0);
    }

    #[test]
    fn test_unlock_continent() {
        let mut save = SaveData::default();
        assert_eq!(save.unlocked_continents.len(), 1);

        save.unlock_continent(Continent::NorthAmerica);
        assert_eq!(save.unlocked_continents.len(), 2);
        assert!(save.is_continent_unlocked(&Continent::NorthAmerica));
    }

    #[test]
    fn test_unlock_same_continent_twice() {
        let mut save = SaveData::default();
        save.unlock_continent(Continent::NorthAmerica);
        save.unlock_continent(Continent::NorthAmerica);

        // Should only be added once
        assert_eq!(save.unlocked_continents.len(), 2);
    }

    #[test]
    fn test_high_score_update() {
        let mut save = SaveData::default();

        // First score is always new
        assert!(save.update_high_score(Continent::Tutorial, 1000));
        assert_eq!(save.get_high_score(&Continent::Tutorial), 1000);

        // Higher score updates
        assert!(save.update_high_score(Continent::Tutorial, 2000));
        assert_eq!(save.get_high_score(&Continent::Tutorial), 2000);

        // Lower score doesn't update
        assert!(!save.update_high_score(Continent::Tutorial, 1500));
        assert_eq!(save.get_high_score(&Continent::Tutorial), 2000);
    }

    #[test]
    fn test_best_time_update() {
        let mut save = SaveData::default();

        // First time is always new
        assert!(save.update_best_time(Continent::Tutorial, 100.0));
        assert_eq!(save.get_best_time(&Continent::Tutorial), Some(100.0));

        // Faster time updates
        assert!(save.update_best_time(Continent::Tutorial, 80.0));
        assert_eq!(save.get_best_time(&Continent::Tutorial), Some(80.0));

        // Slower time doesn't update
        assert!(!save.update_best_time(Continent::Tutorial, 90.0));
        assert_eq!(save.get_best_time(&Continent::Tutorial), Some(80.0));
    }

    #[test]
    fn test_settings_update() {
        let mut save = SaveData::default();
        save.update_settings(75.0, 50.0, 2);

        assert_eq!(save.settings.sound_volume, 75.0);
        assert_eq!(save.settings.music_volume, 50.0);
        assert_eq!(save.settings.difficulty, 2);
    }

    #[test]
    fn test_statistics_tracking() {
        let mut save = SaveData::default();

        save.add_play_time(120.5);
        save.record_death();
        save.record_death();
        save.record_boss_kill();

        assert_eq!(save.total_play_time, 120.5);
        assert_eq!(save.total_deaths, 2);
        assert_eq!(save.total_boss_kills, 1);
    }

    #[test]
    fn test_serialization() {
        let save = SaveData::default();
        let json = serde_json::to_string(&save).unwrap();
        let loaded: SaveData = serde_json::from_str(&json).unwrap();

        assert_eq!(save.unlocked_continents.len(), loaded.unlocked_continents.len());
        assert_eq!(save.settings.difficulty, loaded.settings.difficulty);
    }
}
