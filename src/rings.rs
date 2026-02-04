use macroquad::prelude::*;
use crate::player::Player;

pub struct Ring {
    pub position: Vec3,
    pub radius: f32,
    pub collected: bool,
    pub rotation: f32,
}

pub struct RingManager {
    rings: Vec<Ring>,
    spawn_timer: f32,
    spawn_interval: f32,
    last_spawn_z: f32,
}

impl RingManager {
    pub fn new() -> Self {
        Self {
            rings: Vec::new(),
            spawn_timer: 0.0,
            spawn_interval: 5.0, // Spawn ring every 5 seconds
            last_spawn_z: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, player: &Player) {
        self.spawn_timer += dt;

        // Spawn new ring ahead of player
        if self.spawn_timer >= self.spawn_interval {
            self.spawn_ring(player.position().z);
            self.spawn_timer = 0.0;
        }

        // Update existing rings
        for ring in &mut self.rings {
            ring.rotation += dt * 0.5; // Slow rotation for visual effect
        }

        // Remove rings that are too far behind player
        self.rings.retain(|ring| {
            ring.position.z > player.position().z - 30.0
        });
    }

    fn spawn_ring(&mut self, player_z: f32) {
        // Spawn ring 80-120 units ahead of player
        let spawn_z = player_z + 80.0 + rand::gen_range(0.0, 40.0);

        // Avoid spawning too close to previous ring
        if spawn_z - self.last_spawn_z < 50.0 {
            return;
        }

        // Random position within flight area
        let x = rand::gen_range(-6.0, 6.0);
        let y = rand::gen_range(0.0, 4.0);

        self.rings.push(Ring {
            position: vec3(x, y, spawn_z),
            radius: 4.0,
            collected: false,
            rotation: 0.0,
        });

        self.last_spawn_z = spawn_z;
    }

    pub fn check_collection(&mut self, player: &Player, score: &mut u32) -> bool {
        let mut collected_any = false;

        for ring in &mut self.rings {
            if ring.collected {
                continue;
            }

            // Check if player flew through the ring
            let player_pos = player.position();
            let distance = vec3(
                player_pos.x - ring.position.x,
                player_pos.y - ring.position.y,
                0.0, // Only check X/Y distance
            ).length();

            // Check if player is at the ring's Z position (within tolerance)
            let z_distance = (player_pos.z - ring.position.z).abs();

            // Player counts as passing through if they're anywhere within the ring bounds
            // Add extra tolerance on the outer edge to make it more forgiving
            let collection_radius = ring.radius + 1.0; // Extra 1 unit tolerance

            if distance < collection_radius && z_distance < 3.0 {
                ring.collected = true;
                *score += 100; // Bonus points for flying through ring
                collected_any = true;
            }
        }

        collected_any
    }

    pub fn draw(&self) {
        for ring in &self.rings {
            if ring.collected {
                continue; // Don't draw collected rings
            }

            // Draw ring using multiple segments
            let segments = 32;
            let ring_color = Color::from_rgba(0, 255, 255, 200); // Cyan semi-transparent
            let ring_inner_color = Color::from_rgba(0, 200, 200, 100); // Darker cyan

            for i in 0..segments {
                let angle1 = (i as f32 / segments as f32) * std::f32::consts::PI * 2.0 + ring.rotation;
                let angle2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::PI * 2.0 + ring.rotation;

                // Outer ring
                let x1 = ring.position.x + angle1.cos() * ring.radius;
                let y1 = ring.position.y + angle1.sin() * ring.radius;
                let x2 = ring.position.x + angle2.cos() * ring.radius;
                let y2 = ring.position.y + angle2.sin() * ring.radius;

                let p1 = vec3(x1, y1, ring.position.z);
                let p2 = vec3(x2, y2, ring.position.z);

                // Draw outer edge
                draw_line_3d(p1, p2, ring_color);

                // Inner ring (slightly smaller)
                let inner_radius = ring.radius * 0.7;
                let ix1 = ring.position.x + angle1.cos() * inner_radius;
                let iy1 = ring.position.y + angle1.sin() * inner_radius;
                let ix2 = ring.position.x + angle2.cos() * inner_radius;
                let iy2 = ring.position.y + angle2.sin() * inner_radius;

                let ip1 = vec3(ix1, iy1, ring.position.z);
                let ip2 = vec3(ix2, iy2, ring.position.z);

                draw_line_3d(ip1, ip2, ring_inner_color);
            }

            // Draw center indicator (small sphere)
            draw_sphere(ring.position, 0.3, None, Color::from_rgba(255, 255, 0, 255));

            // Draw connecting lines for depth perception
            let num_spokes = 8;
            for i in 0..num_spokes {
                let angle = (i as f32 / num_spokes as f32) * std::f32::consts::PI * 2.0 + ring.rotation;
                let x = ring.position.x + angle.cos() * ring.radius * 0.7;
                let y = ring.position.y + angle.sin() * ring.radius * 0.7;
                let outer_x = ring.position.x + angle.cos() * ring.radius;
                let outer_y = ring.position.y + angle.sin() * ring.radius;

                draw_line_3d(
                    vec3(x, y, ring.position.z),
                    vec3(outer_x, outer_y, ring.position.z),
                    Color::from_rgba(0, 150, 150, 150),
                );
            }
        }
    }

    pub fn clear(&mut self) {
        self.rings.clear();
        self.spawn_timer = 0.0;
        self.last_spawn_z = 0.0;
    }

    pub fn reset(&mut self) {
        self.clear();
    }
}
