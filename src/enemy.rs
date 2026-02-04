use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::player::Player;

#[derive(Clone, Copy)]
pub enum EnemyType {
    Drone,      // Flies straight
    Seeker,     // Follows player
    Zigzag,     // Moves in zigzag pattern
    Turret,     // Stationary, shoots at player
}

pub struct Enemy {
    pub position: Vec3,
    velocity: Vec3,
    enemy_type: EnemyType,
    health: f32,
    time_alive: f32,
}

impl Enemy {
    fn new(position: Vec3, enemy_type: EnemyType) -> Self {
        let velocity = match enemy_type {
            EnemyType::Drone => vec3(0.0, 0.0, -5.0),
            EnemyType::Seeker => vec3(0.0, 0.0, -3.0),
            EnemyType::Zigzag => vec3(2.0, 0.0, -4.0),
            EnemyType::Turret => vec3(0.0, 0.0, 0.0),
        };

        Self {
            position,
            velocity,
            enemy_type,
            health: 30.0,
            time_alive: 0.0,
        }
    }

    fn update(&mut self, dt: f32, player_pos: Vec3) {
        self.time_alive += dt;

        match self.enemy_type {
            EnemyType::Drone => {
                // Simple straight movement
                self.position += self.velocity * dt;
            }
            EnemyType::Seeker => {
                // Follow player
                let direction = (player_pos - self.position).normalize();
                self.velocity = direction * 4.0;
                self.position += self.velocity * dt;
            }
            EnemyType::Zigzag => {
                // Zigzag pattern
                let zigzag = (self.time_alive * 3.0).sin() * 5.0;
                self.velocity.x = zigzag;
                self.position += self.velocity * dt;
            }
            EnemyType::Turret => {
                // Stationary - just moves back relative to player
                self.velocity.z = -8.0; // Match terrain scroll
                self.position += self.velocity * dt;
            }
        }
    }

    fn draw(&self) {
        let (color, size) = match self.enemy_type {
            EnemyType::Drone => (Color::from_rgba(255, 100, 100, 255), vec3(0.5, 0.5, 0.8)),
            EnemyType::Seeker => (Color::from_rgba(255, 0, 0, 255), vec3(0.7, 0.4, 0.7)),
            EnemyType::Zigzag => (Color::from_rgba(255, 150, 0, 255), vec3(0.6, 0.3, 0.9)),
            EnemyType::Turret => (Color::from_rgba(180, 0, 0, 255), vec3(0.8, 0.8, 0.8)),
        };

        // Draw enemy body
        draw_cube(self.position, size, None, color);

        // Draw wireframe for retro effect
        draw_cube_wires(self.position, size, WHITE);

        // Draw "eye" or indicator
        let eye_pos = self.position + vec3(0.0, 0.2, 0.3);
        draw_sphere(eye_pos, 0.1, None, Color::from_rgba(255, 255, 0, 255));
    }

    fn take_damage(&mut self, damage: f32) {
        self.health -= damage;
    }

    fn is_dead(&self) -> bool {
        self.health <= 0.0
    }
}

pub struct EnemyManager {
    enemies: Vec<Enemy>,
    spawn_timer: f32,
    spawn_interval: f32,
    spawn_cooldown: f32,
}

impl EnemyManager {
    pub fn new() -> Self {
        Self {
            enemies: Vec::new(),
            spawn_timer: 0.0,
            spawn_interval: 2.0,
            spawn_cooldown: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, player: &Player) {
        let player_pos = player.position();

        // Update spawn cooldown
        if self.spawn_cooldown > 0.0 {
            self.spawn_cooldown -= dt;
        }

        // Update spawn timer (only if cooldown is 0)
        if self.spawn_cooldown <= 0.0 {
            self.spawn_timer += dt;
            if self.spawn_timer >= self.spawn_interval {
                self.spawn_enemy(player_pos);
                self.spawn_timer = 0.0;
                // Gradually increase spawn rate (but not too fast)
                self.spawn_interval = (self.spawn_interval * 0.98).max(0.8);
            }
        }

        // Update all enemies
        for enemy in &mut self.enemies {
            enemy.update(dt, player_pos);
        }

        // Check collisions with player projectiles
        let projectiles = player.get_projectiles();
        for (_proj_idx, proj) in projectiles.iter().enumerate() {
            for enemy in &mut self.enemies {
                let distance = (enemy.position - proj.position).length();
                if distance < 1.0 {
                    enemy.take_damage(20.0);
                }
            }
        }

        // Remove dead or far away enemies
        self.enemies.retain(|enemy| {
            !enemy.is_dead() && enemy.position.z > player_pos.z - 30.0
        });
    }

    fn spawn_enemy(&mut self, player_pos: Vec3) {
        // Random spawn position ahead of player
        let spawn_x = gen_range(-6.0, 6.0);
        let spawn_y = gen_range(0.0, 5.0);
        let spawn_z = player_pos.z + gen_range(40.0, 60.0);

        // Random enemy type with weighted distribution
        let enemy_type = match gen_range(0, 10) {
            0..=4 => EnemyType::Drone,
            5..=7 => EnemyType::Zigzag,
            8 => EnemyType::Seeker,
            _ => EnemyType::Turret,
        };

        self.enemies.push(Enemy::new(
            vec3(spawn_x, spawn_y, spawn_z),
            enemy_type,
        ));
    }

    pub fn draw(&self) {
        for enemy in &self.enemies {
            enemy.draw();
        }
    }

    pub fn check_collision(&self, player: &Player) -> bool {
        let player_pos = player.position();
        let collision_distance = 1.0;

        for enemy in &self.enemies {
            let distance = (enemy.position - player_pos).length();
            if distance < collision_distance {
                return true;
            }
        }

        false
    }

    pub fn check_projectile_hit(&mut self, projectile_pos: Vec3) -> bool {
        let hit_distance = 1.0;

        for enemy in &mut self.enemies {
            let distance = (enemy.position - projectile_pos).length();
            if distance < hit_distance {
                enemy.take_damage(20.0);
                return true;
            }
        }

        false
    }

    /// Clear enemies within a radius around a position (used on checkpoint respawn)
    pub fn clear_around_position(&mut self, position: Vec3, radius: f32) {
        let initial_count = self.enemies.len();
        self.enemies.retain(|enemy| {
            let distance = (enemy.position - position).length();
            distance > radius
        });
        let cleared = initial_count - self.enemies.len();
        #[cfg(debug_assertions)]
        if cleared > 0 {
            println!("Cleared {} enemies around checkpoint", cleared);
        }
    }

    /// Pause spawning for a duration (used after respawn)
    pub fn pause_spawning(&mut self, duration: f32) {
        self.spawn_cooldown = duration;
        #[cfg(debug_assertions)]
        println!("Enemy spawning paused for {:.1}s", duration);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_player(pos: Vec3) -> Player {
        let mut player = Player::new();
        player.set_position(pos);
        player
    }

    #[test]
    fn test_enemy_manager_initialization() {
        let manager = EnemyManager::new();
        assert_eq!(manager.enemies.len(), 0);
        assert_eq!(manager.spawn_interval, 2.0);
    }

    #[test]
    fn test_enemy_spawning() {
        let mut manager = EnemyManager::new();
        let player = create_test_player(vec3(0.0, 0.0, 0.0));

        // Fast-forward time to trigger spawn
        for _ in 0..3 {
            manager.update(1.0, &player);
        }

        // Should have spawned at least one enemy
        assert!(manager.enemies.len() > 0);
    }

    #[test]
    fn test_spawn_interval_decreases() {
        let mut manager = EnemyManager::new();
        let initial_interval = manager.spawn_interval;
        let player = create_test_player(vec3(0.0, 0.0, 0.0));

        // Trigger several spawns
        for _ in 0..10 {
            manager.update(3.0, &player);
        }

        // Spawn interval should decrease (but not below 0.8)
        assert!(manager.spawn_interval <= initial_interval);
        assert!(manager.spawn_interval >= 0.8);
    }

    #[test]
    fn test_enemy_cleanup() {
        let mut manager = EnemyManager::new();
        let player = create_test_player(vec3(0.0, 0.0, 100.0));

        // Add enemy far behind player
        manager.enemies.push(Enemy::new(
            vec3(0.0, 0.0, 0.0),
            EnemyType::Drone,
        ));

        manager.update(0.1, &player);

        // Enemy should be removed (too far behind)
        assert_eq!(manager.enemies.len(), 0);
    }

    #[test]
    fn test_enemy_collision_detection() {
        let mut manager = EnemyManager::new();
        let player = create_test_player(vec3(0.0, 0.0, 10.0));

        // Add enemy at player position
        manager.enemies.push(Enemy::new(
            vec3(0.0, 0.0, 10.0),
            EnemyType::Drone,
        ));

        assert!(manager.check_collision(&player));
    }

    #[test]
    fn test_no_collision_when_far() {
        let mut manager = EnemyManager::new();
        let player = create_test_player(vec3(0.0, 0.0, 10.0));

        // Add enemy far from player
        manager.enemies.push(Enemy::new(
            vec3(20.0, 20.0, 50.0),
            EnemyType::Drone,
        ));

        assert!(!manager.check_collision(&player));
    }

    #[test]
    fn test_enemy_takes_damage_from_projectile() {
        let mut manager = EnemyManager::new();
        let mut player = create_test_player(vec3(0.0, 0.0, 0.0));
        player.pickup_weapon(Weapon::Laser);

        // Add enemy ahead
        manager.enemies.push(Enemy::new(
            vec3(0.0, 0.0, 5.0),
            EnemyType::Drone,
        ));

        // Shoot at enemy
        player.shoot();
        player.projectiles[0].position = vec3(0.0, 0.0, 5.0);

        let initial_health = manager.enemies[0].health;
        manager.update(0.1, &player);

        // Enemy should take damage
        assert!(manager.enemies[0].health < initial_health);
    }

    #[test]
    fn test_dead_enemy_removal() {
        let mut manager = EnemyManager::new();
        let mut player = create_test_player(vec3(0.0, 0.0, 0.0));
        player.pickup_weapon(Weapon::Laser);

        // Add enemy
        manager.enemies.push(Enemy::new(
            vec3(0.0, 0.0, 5.0),
            EnemyType::Drone,
        ));

        // Kill enemy by setting health to 0
        manager.enemies[0].health = 0.0;

        manager.update(0.1, &player);

        // Dead enemy should be removed
        assert_eq!(manager.enemies.len(), 0);
    }

    #[test]
    fn test_drone_movement() {
        let mut enemy = Enemy::new(vec3(0.0, 0.0, 10.0), EnemyType::Drone);
        let player_pos = vec3(0.0, 0.0, 0.0);
        let initial_z = enemy.position.z;

        enemy.update(0.1, player_pos);

        // Drone should move backward (toward player)
        assert!(enemy.position.z < initial_z);
    }

    #[test]
    fn test_seeker_follows_player() {
        let mut enemy = Enemy::new(vec3(10.0, 0.0, 10.0), EnemyType::Seeker);
        let player_pos = vec3(0.0, 0.0, 0.0);
        let initial_distance = (enemy.position - player_pos).length();

        enemy.update(0.5, player_pos);

        // Seeker should move closer to player
        let new_distance = (enemy.position - player_pos).length();
        assert!(new_distance < initial_distance);
    }

    #[test]
    fn test_turret_stationary() {
        let mut enemy = Enemy::new(vec3(0.0, 0.0, 10.0), EnemyType::Turret);
        let player_pos = vec3(0.0, 0.0, 0.0);
        let initial_x = enemy.position.x;
        let initial_y = enemy.position.y;

        enemy.update(0.1, player_pos);

        // Turret should not move laterally (only z-axis for scrolling)
        assert_eq!(enemy.position.x, initial_x);
        assert_eq!(enemy.position.y, initial_y);
    }

    #[test]
    fn test_clear_around_position() {
        let mut manager = EnemyManager::new();

        // Add enemies at various positions
        let center = vec3(0.0, 0.0, 100.0);
        manager.enemies.push(Enemy::new(
            vec3(5.0, 0.0, 100.0), // Close
            EnemyType::Drone,
        ));
        manager.enemies.push(Enemy::new(
            vec3(30.0, 0.0, 100.0), // Far
            EnemyType::Seeker,
        ));

        manager.clear_around_position(center, 15.0);

        // Should only keep far enemy
        assert_eq!(manager.enemies.len(), 1);
        assert!(manager.enemies[0].position.x > 20.0);
    }
}
