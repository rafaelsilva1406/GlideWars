use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Splash,
    MainMenu,
    Options,
    LevelSelect,
    TutorialInstructions, // Tutorial instructions screen
    Tutorial,
    InGame,
    BossFight,
    Checkpoint,      // Respawn countdown
    LevelComplete,
    GameOver,
}

pub struct GameStateManager {
    current_state: GameState,
    previous_state: Option<GameState>,
    state_transition_time: f32,
    pending_transition: Option<GameState>,
}

impl GameStateManager {
    pub fn new() -> Self {
        Self {
            current_state: GameState::Splash,
            previous_state: None,
            state_transition_time: 0.0,
            pending_transition: None,
        }
    }

    pub fn current_state(&self) -> GameState {
        self.current_state
    }

    pub fn previous_state(&self) -> Option<GameState> {
        self.previous_state
    }

    pub fn request_transition(&mut self, new_state: GameState) {
        if new_state != self.current_state {
            self.pending_transition = Some(new_state);
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.state_transition_time += dt;

        // Process pending transitions
        if let Some(new_state) = self.pending_transition {
            self.transition_to(new_state);
            self.pending_transition = None;
        }
    }

    fn transition_to(&mut self, new_state: GameState) {
        #[cfg(debug_assertions)]
        println!("State transition: {:?} -> {:?}", self.current_state, new_state);
        self.previous_state = Some(self.current_state);
        self.current_state = new_state;
        self.state_transition_time = 0.0;
    }

    pub fn time_in_current_state(&self) -> f32 {
        self.state_transition_time
    }

    pub fn can_transition(&self, from: GameState, to: GameState) -> bool {
        // Define valid state transitions
        match (from, to) {
            // From Splash
            (GameState::Splash, GameState::MainMenu) => true,

            // From MainMenu
            (GameState::MainMenu, GameState::Options) => true,
            (GameState::MainMenu, GameState::LevelSelect) => true,

            // From Options
            (GameState::Options, GameState::MainMenu) => true,

            // From LevelSelect
            (GameState::LevelSelect, GameState::MainMenu) => true,
            (GameState::LevelSelect, GameState::TutorialInstructions) => true,
            (GameState::LevelSelect, GameState::InGame) => true,

            // From TutorialInstructions
            (GameState::TutorialInstructions, GameState::Tutorial) => true,
            (GameState::TutorialInstructions, GameState::LevelSelect) => true,

            // From Tutorial
            (GameState::Tutorial, GameState::InGame) => true,
            (GameState::Tutorial, GameState::GameOver) => true,
            (GameState::Tutorial, GameState::Checkpoint) => true,
            (GameState::Tutorial, GameState::LevelComplete) => true,

            // From InGame
            (GameState::InGame, GameState::BossFight) => true,
            (GameState::InGame, GameState::Checkpoint) => true,
            (GameState::InGame, GameState::GameOver) => true,
            (GameState::InGame, GameState::MainMenu) => true,

            // From BossFight
            (GameState::BossFight, GameState::LevelComplete) => true,
            (GameState::BossFight, GameState::InGame) => true, // Continue after boss defeat
            (GameState::BossFight, GameState::Checkpoint) => true,
            (GameState::BossFight, GameState::GameOver) => true,
            (GameState::BossFight, GameState::MainMenu) => true,

            // From Checkpoint
            (GameState::Checkpoint, GameState::InGame) => true,
            (GameState::Checkpoint, GameState::BossFight) => true,
            (GameState::Checkpoint, GameState::GameOver) => true,
            (GameState::Checkpoint, GameState::MainMenu) => true,

            // From LevelComplete
            (GameState::LevelComplete, GameState::LevelSelect) => true,
            (GameState::LevelComplete, GameState::MainMenu) => true,

            // From GameOver
            (GameState::GameOver, GameState::MainMenu) => true,
            (GameState::GameOver, GameState::LevelSelect) => true,
            (GameState::GameOver, GameState::InGame) => true, // Restart level

            // Same state is always allowed
            _ if from == to => true,

            // All other transitions are invalid
            _ => false,
        }
    }

    pub fn force_transition(&mut self, new_state: GameState) {
        self.transition_to(new_state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let manager = GameStateManager::new();
        assert_eq!(manager.current_state(), GameState::Splash);
        assert_eq!(manager.previous_state(), None);
    }

    #[test]
    fn test_valid_transition() {
        let mut manager = GameStateManager::new();
        assert!(manager.can_transition(GameState::Splash, GameState::MainMenu));
        manager.request_transition(GameState::MainMenu);
        manager.update(0.016);
        assert_eq!(manager.current_state(), GameState::MainMenu);
        assert_eq!(manager.previous_state(), Some(GameState::Splash));
    }

    #[test]
    fn test_invalid_transition() {
        let manager = GameStateManager::new();
        assert!(!manager.can_transition(GameState::Splash, GameState::InGame));
    }

    #[test]
    fn test_time_in_state() {
        let mut manager = GameStateManager::new();
        manager.update(0.5);
        assert_eq!(manager.time_in_current_state(), 0.5);
        manager.request_transition(GameState::MainMenu);
        manager.update(0.016);
        assert!(manager.time_in_current_state() < 0.1);
    }

    #[test]
    fn test_splash_to_main_menu_flow() {
        let mut manager = GameStateManager::new();
        manager.request_transition(GameState::MainMenu);
        manager.update(0.016);
        assert_eq!(manager.current_state(), GameState::MainMenu);
    }

    #[test]
    fn test_game_loop_transitions() {
        let mut manager = GameStateManager::new();

        // Splash -> MainMenu
        manager.request_transition(GameState::MainMenu);
        manager.update(0.016);
        assert_eq!(manager.current_state(), GameState::MainMenu);

        // MainMenu -> LevelSelect
        manager.request_transition(GameState::LevelSelect);
        manager.update(0.016);
        assert_eq!(manager.current_state(), GameState::LevelSelect);

        // LevelSelect -> InGame
        manager.request_transition(GameState::InGame);
        manager.update(0.016);
        assert_eq!(manager.current_state(), GameState::InGame);

        // InGame -> BossFight
        manager.request_transition(GameState::BossFight);
        manager.update(0.016);
        assert_eq!(manager.current_state(), GameState::BossFight);

        // BossFight -> LevelComplete
        manager.request_transition(GameState::LevelComplete);
        manager.update(0.016);
        assert_eq!(manager.current_state(), GameState::LevelComplete);
    }
}
