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
}

impl EnemyManager {
    pub fn new() -> Self {
        Self {
            enemies: Vec::new(),
            spawn_timer: 0.0,
            spawn_interval: 2.0,
        }
    }

    pub fn update(&mut self, dt: f32, player: &Player) {
        let player_pos = player.position();

        // Update spawn timer
        self.spawn_timer += dt;
        if self.spawn_timer >= self.spawn_interval {
            self.spawn_enemy(player_pos);
            self.spawn_timer = 0.0;
            // Gradually increase spawn rate (but not too fast)
            self.spawn_interval = (self.spawn_interval * 0.98).max(0.8);
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
}
