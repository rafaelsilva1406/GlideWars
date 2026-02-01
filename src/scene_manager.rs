use macroquad::prelude::*;
use crate::game_state::{GameState, GameStateManager};
use crate::input_manager::InputManager;

pub struct SceneData {
    // Stores any data that needs to persist across scenes
    pub score: u32,
    pub selected_level: Option<String>,
    pub respawn_countdown: Option<f32>,
}

impl SceneData {
    pub fn new() -> Self {
        Self {
            score: 0,
            selected_level: None,
            respawn_countdown: None,
        }
    }

    pub fn reset_score(&mut self) {
        self.score = 0;
    }
}

pub struct SceneManager {
    state_manager: GameStateManager,
    scene_data: SceneData,
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            state_manager: GameStateManager::new(),
            scene_data: SceneData::new(),
        }
    }

    pub fn current_state(&self) -> GameState {
        self.state_manager.current_state()
    }

    pub fn request_transition(&mut self, new_state: GameState) {
        if self.state_manager.can_transition(self.state_manager.current_state(), new_state) {
            self.state_manager.request_transition(new_state);
            self.on_state_enter(new_state);
        } else {
            println!(
                "Invalid transition from {:?} to {:?}",
                self.state_manager.current_state(),
                new_state
            );
        }
    }

    pub fn force_transition(&mut self, new_state: GameState) {
        self.state_manager.force_transition(new_state);
        self.on_state_enter(new_state);
    }

    pub fn update(&mut self, dt: f32, _input: &InputManager) {
        self.state_manager.update(dt);

        // Handle automatic transitions based on time
        match self.state_manager.current_state() {
            GameState::Splash => {
                // Auto-transition to main menu after 2 seconds
                if self.state_manager.time_in_current_state() > 2.0 {
                    self.request_transition(GameState::MainMenu);
                }
            }
            GameState::Checkpoint => {
                // Handle respawn countdown
                if let Some(countdown) = self.scene_data.respawn_countdown.as_mut() {
                    *countdown -= dt;
                    if *countdown <= 0.0 {
                        // Respawn back to game
                        self.scene_data.respawn_countdown = None;
                        self.request_transition(GameState::InGame);
                    }
                }
            }
            _ => {}
        }
    }

    fn on_state_enter(&mut self, state: GameState) {
        match state {
            GameState::Checkpoint => {
                // Start 60-second countdown
                self.scene_data.respawn_countdown = Some(60.0);
            }
            GameState::InGame => {
                // Clear respawn countdown if entering game
                self.scene_data.respawn_countdown = None;
            }
            GameState::MainMenu => {
                // Could reset some data here if needed
            }
            _ => {}
        }
    }

    pub fn scene_data(&self) -> &SceneData {
        &self.scene_data
    }

    pub fn scene_data_mut(&mut self) -> &mut SceneData {
        &mut self.scene_data
    }

    pub fn time_in_current_state(&self) -> f32 {
        self.state_manager.time_in_current_state()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_manager_creation() {
        let manager = SceneManager::new();
        assert_eq!(manager.current_state(), GameState::Splash);
    }

    #[test]
    fn test_valid_transition() {
        let mut manager = SceneManager::new();
        manager.request_transition(GameState::MainMenu);
        manager.update(0.016, &InputManager::new());
        assert_eq!(manager.current_state(), GameState::MainMenu);
    }

    #[test]
    fn test_invalid_transition_rejected() {
        let mut manager = SceneManager::new();
        manager.request_transition(GameState::InGame);
        manager.update(0.016, &InputManager::new());
        // Should still be in Splash because transition is invalid
        assert_eq!(manager.current_state(), GameState::Splash);
    }

    #[test]
    fn test_checkpoint_countdown() {
        let mut manager = SceneManager::new();
        manager.force_transition(GameState::Checkpoint);

        // Check countdown starts at 60 seconds
        assert!(manager.scene_data().respawn_countdown.is_some());
        let initial_countdown = manager.scene_data().respawn_countdown.unwrap();
        assert_eq!(initial_countdown, 60.0);

        // Update and check countdown decreases
        manager.update(1.0, &InputManager::new());
        let after_update = manager.scene_data().respawn_countdown.unwrap();
        assert!(after_update < initial_countdown);
    }

    #[test]
    fn test_auto_transition_from_splash() {
        let mut manager = SceneManager::new();

        // Update for more than 2 seconds
        for _ in 0..150 {
            // 150 frames at ~60fps = 2.5 seconds
            manager.update(0.016, &InputManager::new());
        }

        // Should have transitioned to MainMenu
        assert_eq!(manager.current_state(), GameState::MainMenu);
    }
}
