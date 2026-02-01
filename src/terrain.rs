use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::player::Player;

#[derive(Clone)]
pub struct Obstacle {
    pub position: Vec3,
    pub size: Vec3,
    pub obstacle_type: ObstacleType,
}

#[derive(Clone, Copy)]
pub enum ObstacleType {
    Mountain,
    Canyon,
    Boulder,
    WindTurbine,
}

pub struct TerrainManager {
    ground_tiles: Vec<Vec3>,
    obstacles: Vec<Obstacle>,
    difficulty: f32,
    last_spawn_z: f32,
}

impl TerrainManager {
    pub fn new() -> Self {
        let mut manager = Self {
            ground_tiles: Vec::new(),
            obstacles: Vec::new(),
            difficulty: 1.0,
            last_spawn_z: 0.0,
        };

        // Initialize ground tiles
        for i in 0..20 {
            manager.ground_tiles.push(vec3(0.0, -3.0, i as f32 * 10.0));
        }

        manager
    }

    pub fn update(&mut self, _dt: f32, player: &Player) {
        let player_z = player.position().z;

        // Progressive difficulty increase
        self.difficulty = 1.0 + (player_z / 500.0);

        // Update ground tiles (scrolling effect)
        for tile in &mut self.ground_tiles {
            if tile.z < player_z - 20.0 {
                // Move tile forward and randomize terrain height slightly
                tile.z += 200.0;
                tile.y = -3.0 + gen_range(-0.5, 0.5);
            }
        }

        // Spawn new obstacles based on distance
        if player_z > self.last_spawn_z {
            self.spawn_obstacles(player_z);
            self.last_spawn_z = player_z + 20.0;
        }

        // Remove obstacles that are behind the player
        self.obstacles.retain(|obs| obs.position.z > player_z - 30.0);
    }

    fn spawn_obstacles(&mut self, player_z: f32) {
        // Number of obstacles increases with difficulty
        let num_obstacles = gen_range(1, (2 + self.difficulty as i32).min(5) + 1);

        for _ in 0..num_obstacles {
            let spawn_z = player_z + gen_range(30.0, 60.0);
            let spawn_x = gen_range(-7.0, 7.0);
            let spawn_y = gen_range(-1.0, 3.0);

            let obstacle_type = match gen_range(0, 4) {
                0 => ObstacleType::Mountain,
                1 => ObstacleType::Canyon,
                2 => ObstacleType::Boulder,
                _ => ObstacleType::WindTurbine,
            };

            let size = match obstacle_type {
                ObstacleType::Mountain => vec3(3.0, 5.0, 3.0),
                ObstacleType::Canyon => vec3(2.0, 1.0, 4.0),
                ObstacleType::Boulder => vec3(1.5, 1.5, 1.5),
                ObstacleType::WindTurbine => vec3(0.5, 4.0, 0.5),
            };

            self.obstacles.push(Obstacle {
                position: vec3(spawn_x, spawn_y, spawn_z),
                size,
                obstacle_type,
            });
        }
    }

    pub fn draw(&self) {
        // Draw ground plane (grid effect for retro look)
        for tile in &self.ground_tiles {
            let tile_size = 10.0;

            // Ground tile
            draw_cube(
                *tile,
                vec3(20.0, 0.5, tile_size),
                None,
                Color::from_rgba(20, 40, 60, 255)
            );

            // Grid lines (retro style)
            for i in 0..=4 {
                let x_offset = -10.0 + i as f32 * 5.0;
                draw_line_3d(
                    vec3(tile.x + x_offset, tile.y + 0.3, tile.z),
                    vec3(tile.x + x_offset, tile.y + 0.3, tile.z + tile_size),
                    Color::from_rgba(0, 100, 150, 255)
                );
            }

            // Side boundaries
            draw_cube(
                vec3(-10.0, tile.y, tile.z),
                vec3(0.2, 2.0, tile_size),
                None,
                Color::from_rgba(100, 100, 200, 255)
            );
            draw_cube(
                vec3(10.0, tile.y, tile.z),
                vec3(0.2, 2.0, tile_size),
                None,
                Color::from_rgba(100, 100, 200, 255)
            );
        }

        // Draw obstacles with retro styling
        for obstacle in &self.obstacles {
            let color = match obstacle.obstacle_type {
                ObstacleType::Mountain => Color::from_rgba(150, 75, 0, 255),
                ObstacleType::Canyon => Color::from_rgba(100, 50, 50, 255),
                ObstacleType::Boulder => Color::from_rgba(120, 120, 120, 255),
                ObstacleType::WindTurbine => Color::from_rgba(200, 200, 200, 255),
            };

            draw_cube(obstacle.position, obstacle.size, None, color);

            // Draw wireframe outline for retro effect
            draw_cube_wires(obstacle.position, obstacle.size, Color::from_rgba(255, 255, 255, 100));
        }
    }

    pub fn check_collision(&self, player: &Player) -> bool {
        let player_pos = player.position();
        let player_radius = 0.7; // Collision radius for player

        for obstacle in &self.obstacles {
            // Simple box collision
            let dx = (player_pos.x - obstacle.position.x).abs();
            let dy = (player_pos.y - obstacle.position.y).abs();
            let dz = (player_pos.z - obstacle.position.z).abs();

            if dx < obstacle.size.x / 2.0 + player_radius
                && dy < obstacle.size.y / 2.0 + player_radius
                && dz < obstacle.size.z / 2.0 + player_radius
            {
                return true;
            }
        }

        false
    }

    pub fn get_obstacles(&self) -> &Vec<Obstacle> {
        &self.obstacles
    }

    pub fn reset_to_position(&mut self, z_position: f32) {
        // Clear obstacles behind the checkpoint position
        self.obstacles.retain(|obs| obs.position.z > z_position - 20.0);

        // Reset ground tiles to align with checkpoint position
        self.ground_tiles.clear();
        for i in 0..20 {
            let z = z_position - 50.0 + i as f32 * 10.0;
            self.ground_tiles.push(vec3(0.0, -3.0, z));
        }

        // Update last spawn position to be just before checkpoint
        self.last_spawn_z = z_position;

        println!("Terrain reset to Z={:.1}", z_position);
    }

    /// Clear obstacles within a radius around a position (used on checkpoint respawn)
    pub fn clear_around_position(&mut self, position: Vec3, radius: f32) {
        let initial_count = self.obstacles.len();
        self.obstacles.retain(|obs| {
            // Calculate distance from obstacle to position
            let distance = (obs.position - position).length();
            distance > radius
        });
        let cleared = initial_count - self.obstacles.len();
        if cleared > 0 {
            println!("Cleared {} obstacles around checkpoint", cleared);
        }
    }
}
