use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::player::Player;

pub struct Cloud {
    position: Vec3,
    size: f32,
    opacity: f32,
    drift_speed: f32,
}

pub struct CloudManager {
    clouds: Vec<Cloud>,
    spawn_timer: f32,
    spawn_interval: f32,
}

impl CloudManager {
    pub fn new() -> Self {
        Self {
            clouds: Vec::new(),
            spawn_timer: 0.0,
            spawn_interval: 5.0, // Spawn cloud every 5 seconds (reduced density)
        }
    }

    pub fn update(&mut self, dt: f32, player: &Player) {
        self.spawn_timer += dt;

        // Spawn new clouds ahead of player
        if self.spawn_timer >= self.spawn_interval {
            self.spawn_cloud(player.position().z);
            self.spawn_timer = 0.0;
        }

        // Update existing clouds (drift slightly)
        for cloud in &mut self.clouds {
            cloud.position.x += cloud.drift_speed * dt;
        }

        // Remove clouds that are too far behind player
        let player_z = player.position().z;
        self.clouds.retain(|cloud| cloud.position.z > player_z - 50.0);
    }

    fn spawn_cloud(&mut self, player_z: f32) {
        // Spawn multiple clouds in a cluster (reduced)
        let num_clouds = gen_range(1, 3);

        for _ in 0..num_clouds {
            let spawn_x = gen_range(-15.0, 15.0); // Wider area for clouds
            let spawn_y = gen_range(2.0, 8.0); // Higher in the sky
            let spawn_z = player_z + gen_range(50.0, 120.0); // Far ahead

            self.clouds.push(Cloud {
                position: vec3(spawn_x, spawn_y, spawn_z),
                size: gen_range(1.5, 4.0), // Half size
                opacity: gen_range(0.05, 0.15), // 10% opacity
                drift_speed: gen_range(-0.3, 0.3),
            });
        }
    }

    pub fn draw(&self) {
        for cloud in &self.clouds {
            // Draw cloud as multiple overlapping spheres for fluffy effect
            let num_puffs = 5;
            let base_color = Color::from_rgba(255, 255, 255, (cloud.opacity * 255.0) as u8);

            for i in 0..num_puffs {
                let offset_x = (i as f32 - 2.0) * cloud.size * 0.3;
                let offset_y = ((i % 2) as f32 - 0.5) * cloud.size * 0.2;

                let puff_pos = vec3(
                    cloud.position.x + offset_x,
                    cloud.position.y + offset_y,
                    cloud.position.z,
                );

                let puff_size = cloud.size * gen_range(0.8, 1.2);

                // Draw semi-transparent sphere
                draw_sphere(puff_pos, puff_size, None, base_color);
            }
        }
    }

    pub fn clear(&mut self) {
        self.clouds.clear();
    }
}
