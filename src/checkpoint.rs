use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use crate::player::Player;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub position: [f32; 3],  // Store as array for serialization
    pub health: f32,
    pub weapon_type: String,
    pub ammo: u32,
    pub score: u32,
}

impl PlayerState {
    pub fn from_player(player: &Player, score: u32) -> Self {
        let pos = player.position();
        Self {
            position: [pos.x, pos.y, pos.z],
            health: player.health(),
            weapon_type: player.current_weapon().unwrap_or("None").to_string(),
            ammo: player.ammo(),
            score,
        }
    }

    pub fn position_vec3(&self) -> Vec3 {
        vec3(self.position[0], self.position[1], self.position[2])
    }
}

#[derive(Debug, Clone)]
pub struct Checkpoint {
    pub position: f32,           // Z-axis position
    pub player_state: PlayerState,
    pub time_created: f32,       // Game time when checkpoint was created
}

impl Checkpoint {
    pub fn new(position: f32, player_state: PlayerState, time_created: f32) -> Self {
        Self {
            position,
            player_state,
            time_created,
        }
    }
}

pub struct CheckpointManager {
    checkpoints: Vec<Checkpoint>,
    active_checkpoint_index: usize,
    respawn_timer: Option<f32>,  // 60 second countdown
    respawn_duration: f32,
}

impl CheckpointManager {
    pub fn new() -> Self {
        Self {
            checkpoints: Vec::new(),
            active_checkpoint_index: 0,
            respawn_timer: None,
            respawn_duration: 60.0,
        }
    }

    pub fn create_checkpoint(&mut self, position: f32, player: &Player, score: u32, game_time: f32) {
        let player_state = PlayerState::from_player(player, score);
        let checkpoint = Checkpoint::new(position, player_state, game_time);

        self.checkpoints.push(checkpoint);
        self.active_checkpoint_index = self.checkpoints.len() - 1;

        // Only show checkpoint messages in debug builds
        #[cfg(debug_assertions)]
        println!(
            "Checkpoint {} created at Z={:.1}",
            self.active_checkpoint_index + 1,
            position
        );
    }

    pub fn start_respawn(&mut self) {
        self.respawn_timer = Some(self.respawn_duration);
    }

    pub fn update_respawn(&mut self, dt: f32) -> bool {
        if let Some(ref mut timer) = self.respawn_timer {
            *timer -= dt;
            if *timer <= 0.0 {
                self.respawn_timer = None;
                return true; // Respawn ready
            }
        }
        false
    }

    pub fn cancel_respawn(&mut self) {
        self.respawn_timer = None;
    }

    pub fn is_respawning(&self) -> bool {
        self.respawn_timer.is_some()
    }

    pub fn respawn_time_remaining(&self) -> Option<f32> {
        self.respawn_timer
    }

    pub fn get_active_checkpoint(&self) -> Option<&Checkpoint> {
        self.checkpoints.get(self.active_checkpoint_index)
    }

    pub fn restore_player_state(&self, player: &mut Player, score: &mut u32) -> bool {
        if let Some(checkpoint) = self.get_active_checkpoint() {
            // Restore player state from checkpoint
            let pos = checkpoint.player_state.position_vec3();
            let health = checkpoint.player_state.health;
            let weapon = match checkpoint.player_state.weapon_type.as_str() {
                "LASER" => crate::player::Weapon::Laser,
                "MISSILE" => crate::player::Weapon::Missile,
                "SPREAD" => crate::player::Weapon::Spread,
                _ => crate::player::Weapon::None,
            };
            let ammo = checkpoint.player_state.ammo;

            player.restore_from_checkpoint(pos, health, weapon, ammo);
            *score = checkpoint.player_state.score;

            #[cfg(debug_assertions)]
            println!(
                "Respawned at checkpoint {} (Z={:.1})",
                self.active_checkpoint_index + 1,
                checkpoint.position
            );

            true
        } else {
            false
        }
    }

    pub fn checkpoint_count(&self) -> usize {
        self.checkpoints.len()
    }

    pub fn active_checkpoint_number(&self) -> usize {
        self.active_checkpoint_index + 1
    }

    pub fn clear(&mut self) {
        self.checkpoints.clear();
        self.active_checkpoint_index = 0;
        self.respawn_timer = None;
    }

    pub fn get_last_checkpoint_position(&self) -> Option<f32> {
        self.get_active_checkpoint().map(|cp| cp.position)
    }
}

impl Default for CheckpointManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::Player;

    #[test]
    fn test_checkpoint_creation() {
        let mut manager = CheckpointManager::new();
        let player = Player::new();

        manager.create_checkpoint(100.0, &player, 500, 30.0);

        assert_eq!(manager.checkpoint_count(), 1);
        assert_eq!(manager.active_checkpoint_number(), 1);
    }

    #[test]
    fn test_respawn_timer() {
        let mut manager = CheckpointManager::new();

        manager.start_respawn();
        assert!(manager.is_respawning());
        assert_eq!(manager.respawn_time_remaining(), Some(60.0));

        // Update timer
        let respawn_ready = manager.update_respawn(30.0);
        assert!(!respawn_ready);
        assert!(manager.is_respawning());

        // Complete respawn
        let respawn_ready = manager.update_respawn(35.0);
        assert!(respawn_ready);
        assert!(!manager.is_respawning());
    }

    #[test]
    fn test_cancel_respawn() {
        let mut manager = CheckpointManager::new();

        manager.start_respawn();
        assert!(manager.is_respawning());

        manager.cancel_respawn();
        assert!(!manager.is_respawning());
    }

    #[test]
    fn test_multiple_checkpoints() {
        let mut manager = CheckpointManager::new();
        let player = Player::new();

        manager.create_checkpoint(100.0, &player, 500, 30.0);
        manager.create_checkpoint(200.0, &player, 1000, 60.0);
        manager.create_checkpoint(300.0, &player, 1500, 90.0);

        assert_eq!(manager.checkpoint_count(), 3);
        assert_eq!(manager.active_checkpoint_number(), 3);

        let active = manager.get_active_checkpoint().unwrap();
        assert_eq!(active.position, 300.0);
    }

    #[test]
    fn test_clear_checkpoints() {
        let mut manager = CheckpointManager::new();
        let player = Player::new();

        manager.create_checkpoint(100.0, &player, 500, 30.0);
        manager.start_respawn();

        manager.clear();

        assert_eq!(manager.checkpoint_count(), 0);
        assert!(!manager.is_respawning());
    }

    #[test]
    fn test_player_state_capture() {
        let mut player = Player::new();
        player.set_weapon(crate::player::Weapon::Laser, 50);

        let state = PlayerState::from_player(&player, 1000);

        assert_eq!(state.health, 100.0);
        assert_eq!(state.weapon_type, "LASER");
        assert_eq!(state.ammo, 50);
        assert_eq!(state.score, 1000);
    }
}
