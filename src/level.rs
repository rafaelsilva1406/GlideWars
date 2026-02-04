use macroquad::prelude::*;
use crate::assets::Continent;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LevelConfig {
    pub continent: Continent,
    pub duration: f32,              // Total level time in seconds (300 = 5 min)
    pub difficulty_curve: f32,      // Multiplier for enemy spawn rate/difficulty
    pub checkpoint_interval: f32,   // Distance between checkpoints (Z-axis)
    pub boss_spawn_time: f32,       // When to spawn boss (seconds)
}

impl LevelConfig {
    pub fn for_continent(continent: Continent) -> Self {
        match continent {
            Continent::Tutorial => Self {
                continent,
                duration: 240.0,  // 4 minutes
                difficulty_curve: 0.5,
                checkpoint_interval: 300.0,  // 7 checkpoints before boss
                boss_spawn_time: 210.0,  // 3:30 mark
            },
            Continent::NorthAmerica => Self {
                continent,
                duration: 300.0,  // 5 minutes
                difficulty_curve: 1.0,
                checkpoint_interval: 75.0,
                boss_spawn_time: 270.0,  // 4:30 mark
            },
            Continent::SouthAmerica => Self {
                continent,
                duration: 300.0,
                difficulty_curve: 1.2,
                checkpoint_interval: 75.0,
                boss_spawn_time: 270.0,
            },
            Continent::Europe => Self {
                continent,
                duration: 300.0,
                difficulty_curve: 1.4,
                checkpoint_interval: 75.0,
                boss_spawn_time: 270.0,
            },
            Continent::Asia => Self {
                continent,
                duration: 300.0,
                difficulty_curve: 1.6,
                checkpoint_interval: 75.0,
                boss_spawn_time: 270.0,
            },
            Continent::Africa => Self {
                continent,
                duration: 300.0,
                difficulty_curve: 1.8,
                checkpoint_interval: 75.0,
                boss_spawn_time: 270.0,
            },
            Continent::Oceania => Self {
                continent,
                duration: 300.0,
                difficulty_curve: 2.0,
                checkpoint_interval: 75.0,
                boss_spawn_time: 270.0,
            },
        }
    }
}

pub struct LevelManager {
    config: LevelConfig,
    elapsed_time: f32,
    player_distance: f32,  // Total distance traveled (Z-axis)
    boss_spawned: bool,
    level_complete: bool,
    last_checkpoint_distance: f32,
}

impl LevelManager {
    pub fn new(continent: Continent) -> Self {
        Self {
            config: LevelConfig::for_continent(continent),
            elapsed_time: 0.0,
            player_distance: 0.0,
            boss_spawned: false,
            level_complete: false,
            last_checkpoint_distance: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, player_z_position: f32) {
        self.elapsed_time += dt;
        self.player_distance = player_z_position;

        // Check if boss should spawn
        if !self.boss_spawned && self.elapsed_time >= self.config.boss_spawn_time {
            self.boss_spawned = true;
        }

        // Check if level is complete
        if self.elapsed_time >= self.config.duration {
            self.level_complete = true;
        }
    }

    pub fn should_spawn_boss(&self) -> bool {
        self.boss_spawned && !self.level_complete
    }

    pub fn is_complete(&self) -> bool {
        self.level_complete
    }

    pub fn should_create_checkpoint(&self) -> bool {
        // Create checkpoint every checkpoint_interval distance
        self.player_distance - self.last_checkpoint_distance >= self.config.checkpoint_interval
    }

    pub fn mark_checkpoint_created(&mut self) {
        self.last_checkpoint_distance = self.player_distance;
    }

    pub fn elapsed_time(&self) -> f32 {
        self.elapsed_time
    }

    pub fn remaining_time(&self) -> f32 {
        (self.config.duration - self.elapsed_time).max(0.0)
    }

    pub fn config(&self) -> &LevelConfig {
        &self.config
    }

    pub fn difficulty_multiplier(&self) -> f32 {
        // Gradually increase difficulty over time
        let progress = self.elapsed_time / self.config.duration;
        self.config.difficulty_curve * (1.0 + progress * 0.5)
    }

    pub fn reset(&mut self) {
        self.elapsed_time = 0.0;
        self.player_distance = 0.0;
        self.boss_spawned = false;
        self.level_complete = false;
        self.last_checkpoint_distance = 0.0;
    }

    pub fn checkpoint_count(&self) -> u32 {
        (self.player_distance / self.config.checkpoint_interval) as u32
    }

    pub fn total_checkpoints(&self) -> u32 {
        // Estimate total checkpoints based on expected travel distance
        let expected_distance = 10.0 * self.config.duration; // Assuming 10 units/sec speed
        (expected_distance / self.config.checkpoint_interval) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_config_tutorial() {
        let config = LevelConfig::for_continent(Continent::Tutorial);
        assert_eq!(config.duration, 240.0);
        assert_eq!(config.difficulty_curve, 0.5);
    }

    #[test]
    fn test_level_manager_creation() {
        let manager = LevelManager::new(Continent::NorthAmerica);
        assert_eq!(manager.elapsed_time(), 0.0);
        assert!(!manager.is_complete());
        assert!(!manager.should_spawn_boss());
    }

    #[test]
    fn test_boss_spawn_timing() {
        let mut manager = LevelManager::new(Continent::NorthAmerica);

        // Before boss spawn time
        manager.update(200.0, 0.0);
        assert!(!manager.should_spawn_boss());

        // After boss spawn time
        manager.update(100.0, 0.0);
        assert!(manager.should_spawn_boss());
    }

    #[test]
    fn test_level_completion() {
        let mut manager = LevelManager::new(Continent::Tutorial);

        // Complete the level
        manager.update(250.0, 0.0);
        assert!(manager.is_complete());
    }

    #[test]
    fn test_checkpoint_creation() {
        let mut manager = LevelManager::new(Continent::NorthAmerica);

        // Move player forward
        manager.update(10.0, 80.0);
        assert!(manager.should_create_checkpoint());

        // Mark checkpoint created
        manager.mark_checkpoint_created();
        assert!(!manager.should_create_checkpoint());
    }

    #[test]
    fn test_difficulty_multiplier() {
        let mut manager = LevelManager::new(Continent::NorthAmerica);

        let initial_difficulty = manager.difficulty_multiplier();

        // Progress through level
        manager.update(150.0, 0.0);

        let mid_difficulty = manager.difficulty_multiplier();
        assert!(mid_difficulty > initial_difficulty);
    }

    #[test]
    fn test_level_reset() {
        let mut manager = LevelManager::new(Continent::Tutorial);
        manager.update(100.0, 50.0);

        manager.reset();
        assert_eq!(manager.elapsed_time(), 0.0);
        assert!(!manager.is_complete());
    }
}
