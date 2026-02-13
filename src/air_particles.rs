use macroquad::prelude::*;
use macroquad::rand::gen_range;

pub struct AirParticle {
    position: Vec3,
    velocity: Vec3,
    lifetime: f32,
    max_lifetime: f32,
}

pub struct AirParticleSystem {
    particles: Vec<AirParticle>,
    spawn_timer: f32,
}

impl AirParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            spawn_timer: 0.0,
        }
    }

    pub fn emit(&mut self, position: Vec3, vertical_velocity: f32) {
        self.spawn_timer += 1.0;

        // Only emit particles when moving significantly up or down
        if vertical_velocity.abs() > 1.0 && self.spawn_timer >= 0.05 {
            // Emit 2-3 particles
            for _ in 0..2 {
                let spread = 0.3;
                self.particles.push(AirParticle {
                    position: position + vec3(
                        gen_range(-spread, spread),
                        gen_range(-spread, spread),
                        gen_range(-0.5, 0.0),
                    ),
                    velocity: vec3(
                        gen_range(-0.5, 0.5),
                        -vertical_velocity * 0.5, // Trail opposite to movement
                        -2.0, // Drift backward
                    ),
                    lifetime: 0.0,
                    max_lifetime: 0.5,
                });
            }
            self.spawn_timer = 0.0;
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Update all particles
        for particle in &mut self.particles {
            particle.lifetime += dt;
            particle.position += particle.velocity * dt;
        }

        // Remove dead particles
        self.particles.retain(|p| p.lifetime < p.max_lifetime);
    }

    pub fn draw(&self) {
        for particle in &self.particles {
            let alpha = ((1.0 - (particle.lifetime / particle.max_lifetime)) * 150.0) as u8;
            let color = Color::from_rgba(200, 220, 255, alpha); // Light blue-white

            // Draw as line (air trail)
            let line_start = particle.position;
            let line_end = particle.position - particle.velocity.normalize_or_zero() * 0.5;
            draw_line_3d(line_start, line_end, color);
        }
    }

    pub fn clear(&mut self) {
        self.particles.clear();
    }
}
