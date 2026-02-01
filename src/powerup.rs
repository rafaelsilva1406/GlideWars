use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::player::{Player, Weapon};

#[derive(Clone, Copy)]
pub enum PowerupType {
    HealthSmall,
    HealthLarge,
    WeaponLaser,
    WeaponMissile,
    WeaponSpread,
    AmmoRefill,
}

pub struct Powerup {
    position: Vec3,
    powerup_type: PowerupType,
    rotation: f32,
    time_alive: f32,
}

impl Powerup {
    fn new(position: Vec3, powerup_type: PowerupType) -> Self {
        Self {
            position,
            powerup_type,
            rotation: 0.0,
            time_alive: 0.0,
        }
    }

    fn update(&mut self, dt: f32) {
        self.rotation += dt * 2.0;
        self.time_alive += dt;

        // Floating animation
        self.position.y += (self.time_alive * 3.0).sin() * 0.01;
    }

    fn draw(&self) {
        let (color, size) = match self.powerup_type {
            PowerupType::HealthSmall => (Color::from_rgba(0, 255, 0, 255), 0.3),
            PowerupType::HealthLarge => (Color::from_rgba(0, 255, 0, 255), 0.5),
            PowerupType::WeaponLaser => (Color::from_rgba(255, 0, 0, 255), 0.4),
            PowerupType::WeaponMissile => (Color::from_rgba(255, 255, 0, 255), 0.4),
            PowerupType::WeaponSpread => (Color::from_rgba(255, 0, 255, 255), 0.4),
            PowerupType::AmmoRefill => (Color::from_rgba(255, 165, 0, 255), 0.35),
        };

        // Draw rotating cube for retro look
        draw_cube(self.position, vec3(size, size, size), None, color);

        // Wireframe overlay
        draw_cube_wires(self.position, vec3(size, size, size), WHITE);

        // Glowing effect
        let glow_size = size + (self.time_alive * 5.0).sin() * 0.1;
        draw_cube_wires(
            self.position,
            vec3(glow_size, glow_size, glow_size),
            Color::from_rgba(color.r as u8, color.g as u8, color.b as u8, 100)
        );
    }

    fn collect(&self, player: &mut Player, score: &mut u32) {
        match self.powerup_type {
            PowerupType::HealthSmall => {
                player.heal(25.0);
                *score += 50;
            }
            PowerupType::HealthLarge => {
                player.heal(50.0);
                *score += 100;
            }
            PowerupType::WeaponLaser => {
                player.set_weapon(Weapon::Laser, 50);
                *score += 200;
            }
            PowerupType::WeaponMissile => {
                player.set_weapon(Weapon::Missile, 20);
                *score += 250;
            }
            PowerupType::WeaponSpread => {
                player.set_weapon(Weapon::Spread, 30);
                *score += 200;
            }
            PowerupType::AmmoRefill => {
                player.add_ammo(25);
                *score += 75;
            }
        }
    }
}

pub struct PowerupManager {
    powerups: Vec<Powerup>,
    spawn_timer: f32,
    spawn_interval: f32,
}

impl PowerupManager {
    pub fn new() -> Self {
        Self {
            powerups: Vec::new(),
            spawn_timer: 0.0,
            spawn_interval: 5.0,
        }
    }

    pub fn update(&mut self, dt: f32, player: &Player, _score: &mut u32) {
        let player_pos = player.position();

        // Update spawn timer
        self.spawn_timer += dt;
        if self.spawn_timer >= self.spawn_interval {
            self.spawn_powerup(player_pos);
            self.spawn_timer = 0.0;
        }

        // Update all powerups
        for powerup in &mut self.powerups {
            powerup.update(dt);
        }

        // Remove powerups that are behind the player
        self.powerups.retain(|powerup| powerup.position.z > player_pos.z - 20.0);
    }

    fn spawn_powerup(&mut self, player_pos: Vec3) {
        // Random spawn position
        let spawn_x = gen_range(-6.0, 6.0);
        let spawn_y = gen_range(1.0, 4.0);
        let spawn_z = player_pos.z + gen_range(30.0, 50.0);

        // Random powerup type with weighted distribution
        let powerup_type = match gen_range(0, 100) {
            0..=30 => PowerupType::HealthSmall,
            31..=40 => PowerupType::HealthLarge,
            41..=55 => PowerupType::WeaponLaser,
            56..=70 => PowerupType::WeaponSpread,
            71..=80 => PowerupType::WeaponMissile,
            _ => PowerupType::AmmoRefill,
        };

        self.powerups.push(Powerup::new(
            vec3(spawn_x, spawn_y, spawn_z),
            powerup_type,
        ));
    }

    pub fn draw(&self) {
        for powerup in &self.powerups {
            powerup.draw();
        }
    }

    pub fn check_collection(&mut self, player: &mut Player, score: &mut u32) {
        let player_pos = player.position();
        let collection_distance = 1.5;

        self.powerups.retain(|powerup| {
            let distance = (powerup.position - player_pos).length();
            if distance < collection_distance {
                powerup.collect(player, score);
                false // Remove collected powerup
            } else {
                true
            }
        });
    }
}
