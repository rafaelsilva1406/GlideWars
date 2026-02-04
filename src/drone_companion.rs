use macroquad::prelude::*;
use crate::player::Player;

pub struct DroneProjectile {
    pub position: Vec3,
    pub velocity: Vec3,
    pub lifetime: f32,
}

pub struct DroneCompanion {
    position: Vec3,
    velocity: Vec3,
    active: bool,
    duration: f32,
    remaining_time: f32,
    shoot_timer: f32,
    shoot_cooldown: f32,
    behavior_timer: f32,
    behavior: DroneBehavior,
    projectiles: Vec<DroneProjectile>,
}

#[derive(Clone, Copy, PartialEq)]
enum DroneBehavior {
    FollowPlayer,
    ClearAhead,
}

impl DroneCompanion {
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            active: false,
            duration: 30.0, // Active for 30 seconds
            remaining_time: 0.0,
            shoot_timer: 0.0,
            shoot_cooldown: 0.5, // Shoots every 0.5 seconds
            behavior_timer: 0.0,
            behavior: DroneBehavior::FollowPlayer,
            projectiles: Vec::new(),
        }
    }

    pub fn activate(&mut self, player_pos: Vec3) {
        self.active = true;
        self.remaining_time = self.duration;
        self.position = player_pos + vec3(-3.0, 1.0, -2.0); // Start beside player
        self.velocity = Vec3::ZERO;
        self.behavior = DroneBehavior::FollowPlayer;
        self.behavior_timer = 0.0;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn remaining_time(&self) -> f32 {
        self.remaining_time
    }

    pub fn update(&mut self, dt: f32, player: &Player) {
        if !self.active {
            return;
        }

        self.remaining_time -= dt;
        if self.remaining_time <= 0.0 {
            self.active = false;
            self.projectiles.clear();
            return;
        }

        let player_pos = player.position();
        let player_vel = player.velocity();

        // Update behavior timer
        self.behavior_timer += dt;
        if self.behavior_timer >= 5.0 {
            // Switch behavior every 5 seconds
            self.behavior = match self.behavior {
                DroneBehavior::FollowPlayer => DroneBehavior::ClearAhead,
                DroneBehavior::ClearAhead => DroneBehavior::FollowPlayer,
            };
            self.behavior_timer = 0.0;
        }

        // Move based on behavior
        match self.behavior {
            DroneBehavior::FollowPlayer => {
                // Follow beside player with slight offset
                let offset = vec3(-3.0 + (self.behavior_timer * 0.5).sin() * 0.5, 1.0, -2.0);
                let target_pos = player_pos + offset;
                let direction = (target_pos - self.position).normalize_or_zero();

                // Match player's forward speed
                self.velocity.z = player_vel.z;
                // Move toward target X/Y position
                self.velocity.x = direction.x * 8.0;
                self.velocity.y = direction.y * 8.0;
            }
            DroneBehavior::ClearAhead => {
                // Fly ahead of player to clear path
                let ahead_distance = 15.0;
                let target_z = player_pos.z + ahead_distance;
                let target_x = player_pos.x + (self.behavior_timer * 0.8).sin() * 4.0;
                let target_y = player_pos.y + 1.0;

                let target_pos = vec3(target_x, target_y, target_z);
                let direction = (target_pos - self.position).normalize_or_zero();

                // Move faster when clearing ahead
                self.velocity = direction * 12.0;
            }
        }

        self.position += self.velocity * dt;

        // Keep drone in bounds
        self.position.x = self.position.x.clamp(-7.0, 7.0);
        self.position.y = self.position.y.clamp(-0.5, 5.0);

        // Shooting logic
        self.shoot_timer += dt;
        if self.shoot_timer >= self.shoot_cooldown {
            self.shoot();
            self.shoot_timer = 0.0;
        }

        // Update projectiles
        self.projectiles.retain_mut(|proj| {
            proj.position += proj.velocity * dt;
            proj.lifetime -= dt;
            proj.lifetime > 0.0
        });
    }

    fn shoot(&mut self) {
        // Shoot forward
        self.projectiles.push(DroneProjectile {
            position: self.position + vec3(0.0, 0.0, 1.0),
            velocity: vec3(0.0, 0.0, 25.0),
            lifetime: 2.0,
        });
    }

    pub fn draw(&self) {
        if !self.active {
            return;
        }

        // Draw drone body (small, friendly green)
        draw_cube(
            self.position,
            vec3(0.4, 0.3, 0.4),
            None,
            Color::from_rgba(0, 255, 100, 255),
        );

        // Draw propellers/wings (smaller)
        let wing_offset = 0.3;
        draw_cube(
            self.position + vec3(-wing_offset, 0.0, 0.0),
            vec3(0.4, 0.05, 0.2),
            None,
            Color::from_rgba(0, 200, 100, 255),
        );
        draw_cube(
            self.position + vec3(wing_offset, 0.0, 0.0),
            vec3(0.4, 0.05, 0.2),
            None,
            Color::from_rgba(0, 200, 100, 255),
        );

        // Draw status indicator (glowing sphere on top)
        let health_percent = self.remaining_time / self.duration;
        let indicator_color = if health_percent > 0.5 {
            Color::from_rgba(0, 255, 0, 200)
        } else if health_percent > 0.25 {
            Color::from_rgba(255, 255, 0, 200)
        } else {
            Color::from_rgba(255, 100, 0, 200)
        };
        draw_sphere(self.position + vec3(0.0, 0.3, 0.0), 0.15, None, indicator_color);

        // Draw projectiles (green)
        for proj in &self.projectiles {
            draw_sphere(
                proj.position,
                0.15,
                None,
                Color::from_rgba(0, 255, 0, 255),
            );
        }
    }

    pub fn get_projectiles(&self) -> &Vec<DroneProjectile> {
        &self.projectiles
    }

    pub fn clear_projectile(&mut self, index: usize) {
        if index < self.projectiles.len() {
            self.projectiles.remove(index);
        }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.projectiles.clear();
    }
}
