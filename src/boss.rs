use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::assets::Continent;
use crate::player::Player;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BossType {
    TutorialBoss,          // Simple, predictable
    MountainGuardian,      // North America
    JungleBehemoth,        // South America
    StormBringer,          // Europe
    DragonKite,            // Asia
    DesertPhoenix,         // Africa
    TidalWave,             // Oceania
}

impl BossType {
    pub fn from_continent(continent: Continent) -> Self {
        match continent {
            Continent::Tutorial => BossType::TutorialBoss,
            Continent::NorthAmerica => BossType::MountainGuardian,
            Continent::SouthAmerica => BossType::JungleBehemoth,
            Continent::Europe => BossType::StormBringer,
            Continent::Asia => BossType::DragonKite,
            Continent::Africa => BossType::DesertPhoenix,
            Continent::Oceania => BossType::TidalWave,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            BossType::TutorialBoss => "Training Drone",
            BossType::MountainGuardian => "Mountain Guardian",
            BossType::JungleBehemoth => "Jungle Behemoth",
            BossType::StormBringer => "Storm Bringer",
            BossType::DragonKite => "Dragon Kite",
            BossType::DesertPhoenix => "Desert Phoenix",
            BossType::TidalWave => "Tidal Wave",
        }
    }

    pub fn max_health(&self) -> f32 {
        match self {
            BossType::TutorialBoss => 300.0,
            BossType::MountainGuardian => 500.0,
            BossType::JungleBehemoth => 600.0,
            BossType::StormBringer => 700.0,
            BossType::DragonKite => 800.0,
            BossType::DesertPhoenix => 900.0,
            BossType::TidalWave => 1000.0,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            BossType::TutorialBoss => Color::from_rgba(150, 150, 150, 255),
            BossType::MountainGuardian => Color::from_rgba(139, 90, 43, 255),
            BossType::JungleBehemoth => Color::from_rgba(50, 150, 50, 255),
            BossType::StormBringer => Color::from_rgba(150, 150, 180, 255),
            BossType::DragonKite => Color::from_rgba(255, 200, 50, 255),
            BossType::DesertPhoenix => Color::from_rgba(255, 150, 50, 255),
            BossType::TidalWave => Color::from_rgba(50, 150, 255, 255),
        }
    }

    pub fn accent_color(&self) -> Color {
        match self {
            BossType::TutorialBoss => Color::from_rgba(200, 200, 200, 255),
            BossType::MountainGuardian => Color::from_rgba(169, 169, 169, 255),
            BossType::JungleBehemoth => Color::from_rgba(100, 200, 100, 255),
            BossType::StormBringer => Color::from_rgba(200, 200, 255, 255),
            BossType::DragonKite => Color::from_rgba(255, 100, 100, 255),
            BossType::DesertPhoenix => Color::from_rgba(255, 200, 100, 255),
            BossType::TidalWave => Color::from_rgba(100, 200, 255, 255),
        }
    }

    pub fn scale(&self) -> f32 {
        match self {
            BossType::TutorialBoss => 2.0,
            BossType::MountainGuardian => 3.0,
            BossType::JungleBehemoth => 3.5,
            BossType::StormBringer => 3.2,
            BossType::DragonKite => 3.8,
            BossType::DesertPhoenix => 3.5,
            BossType::TidalWave => 4.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AttackPattern {
    ProjectileBarrage,  // Shoots multiple projectiles in patterns
    ChargeAttack,       // Rushes toward player
    AreaDenial,         // Spawns temporary obstacles
    CircularShot,       // Shoots projectiles in a circle
    LaserBeam,          // Continuous beam attack
}

pub struct BossProjectile {
    pub position: Vec3,
    pub velocity: Vec3,
    pub lifetime: f32,
    pub damage: f32,
}

pub struct Boss {
    boss_type: BossType,
    pub position: Vec3,
    velocity: Vec3,
    health: f32,
    max_health: f32,
    phase: u8,              // Different attack patterns per phase
    time_alive: f32,
    attack_timer: f32,
    attack_cooldown: f32,
    current_attack: AttackPattern,
    projectiles: Vec<BossProjectile>,
    defeated: bool,
}

impl Boss {
    pub fn new(boss_type: BossType, spawn_position: Vec3) -> Self {
        let max_health = boss_type.max_health();

        Self {
            boss_type,
            position: spawn_position,
            velocity: Vec3::ZERO,
            health: max_health,
            max_health,
            phase: 1,
            time_alive: 0.0,
            attack_timer: 0.0,
            attack_cooldown: 2.0,
            current_attack: AttackPattern::ProjectileBarrage,
            projectiles: Vec::new(),
            defeated: false,
        }
    }

    pub fn update(&mut self, dt: f32, player_pos: Vec3) {
        if self.defeated {
            return;
        }

        self.time_alive += dt;
        self.attack_timer += dt;

        // Update phase based on health
        let health_percent = self.health / self.max_health;
        self.phase = if health_percent > 0.66 {
            1
        } else if health_percent > 0.33 {
            2
        } else {
            3
        };

        // Boss movement - circular pattern around player
        let angle = self.time_alive * 0.5;
        let radius = 15.0;
        let target_x = player_pos.x + angle.cos() * radius;
        let target_y = player_pos.y + 3.0 + (self.time_alive * 0.3).sin() * 2.0;

        // Move toward target position
        let target_pos = vec3(target_x, target_y, player_pos.z + 20.0);
        let direction = (target_pos - self.position).normalize();
        self.velocity = direction * 3.0;
        self.position += self.velocity * dt;

        // Attack logic
        if self.attack_timer >= self.attack_cooldown {
            self.execute_attack(player_pos);
            self.attack_timer = 0.0;

            // Faster attacks in higher phases
            self.attack_cooldown = match self.phase {
                1 => 2.0,
                2 => 1.5,
                3 => 1.0,
                _ => 2.0,
            };
        }

        // Update projectiles
        self.projectiles.retain_mut(|proj| {
            proj.position += proj.velocity * dt;
            proj.lifetime -= dt;
            proj.lifetime > 0.0
        });
    }

    fn execute_attack(&mut self, player_pos: Vec3) {
        match self.phase {
            1 => {
                // Phase 1: Simple projectile barrage
                self.current_attack = AttackPattern::ProjectileBarrage;
                self.attack_projectile_barrage(player_pos);
            }
            2 => {
                // Phase 2: Circular shots + barrage
                self.current_attack = if gen_range(0, 2) == 0 {
                    AttackPattern::ProjectileBarrage
                } else {
                    AttackPattern::CircularShot
                };

                match self.current_attack {
                    AttackPattern::ProjectileBarrage => self.attack_projectile_barrage(player_pos),
                    AttackPattern::CircularShot => self.attack_circular_shot(),
                    _ => {}
                }
            }
            3 => {
                // Phase 3: All attack patterns
                let pattern = gen_range(0, 3);
                self.current_attack = match pattern {
                    0 => AttackPattern::ProjectileBarrage,
                    1 => AttackPattern::CircularShot,
                    _ => AttackPattern::ChargeAttack,
                };

                match self.current_attack {
                    AttackPattern::ProjectileBarrage => self.attack_projectile_barrage(player_pos),
                    AttackPattern::CircularShot => self.attack_circular_shot(),
                    AttackPattern::ChargeAttack => self.attack_charge(player_pos),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn attack_projectile_barrage(&mut self, player_pos: Vec3) {
        // Shoot 3-5 projectiles toward player with slight spread
        let num_projectiles = match self.phase {
            1 => 3,
            2 => 4,
            _ => 5,
        };

        for i in 0..num_projectiles {
            let spread = (i as f32 - num_projectiles as f32 / 2.0) * 0.3;
            let direction = (player_pos - self.position).normalize();
            let velocity = vec3(
                direction.x + spread,
                direction.y + gen_range(-0.2, 0.2),
                direction.z,
            ).normalize() * 15.0;

            self.projectiles.push(BossProjectile {
                position: self.position,
                velocity,
                lifetime: 5.0,
                damage: 15.0,
            });
        }
    }

    fn attack_circular_shot(&mut self) {
        // Shoot projectiles in all directions
        let num_projectiles = 8 + (self.phase as usize * 2);

        for i in 0..num_projectiles {
            let angle = (i as f32 / num_projectiles as f32) * std::f32::consts::PI * 2.0;
            let velocity = vec3(
                angle.cos() * 10.0,
                angle.sin() * 10.0,
                gen_range(-2.0, 2.0),
            );

            self.projectiles.push(BossProjectile {
                position: self.position,
                velocity,
                lifetime: 6.0,
                damage: 10.0,
            });
        }
    }

    fn attack_charge(&mut self, player_pos: Vec3) {
        // Quick dash toward player
        let direction = (player_pos - self.position).normalize();
        self.velocity = direction * 25.0;
    }

    pub fn take_damage(&mut self, damage: f32) {
        self.health -= damage;
        if self.health <= 0.0 {
            self.health = 0.0;
            self.defeated = true;
        }
    }

    pub fn is_defeated(&self) -> bool {
        self.defeated
    }

    pub fn health(&self) -> f32 {
        self.health
    }

    pub fn max_health(&self) -> f32 {
        self.max_health
    }

    pub fn health_percentage(&self) -> f32 {
        self.health / self.max_health
    }

    pub fn phase(&self) -> u8 {
        self.phase
    }

    pub fn boss_type(&self) -> BossType {
        self.boss_type
    }

    pub fn check_collision_with_player(&self, player_pos: Vec3) -> bool {
        let distance = (self.position - player_pos).length();
        distance < 2.0 * self.boss_type.scale()
    }

    pub fn check_projectile_collision(&mut self, player_pos: Vec3) -> bool {
        let mut hit = false;
        self.projectiles.retain(|proj| {
            let distance = (proj.position - player_pos).length();
            if distance < 1.0 {
                hit = true;
                false // Remove projectile
            } else {
                true
            }
        });
        hit
    }

    pub fn check_hit_by_player_projectile(&mut self, projectile_pos: Vec3) -> bool {
        let distance = (self.position - projectile_pos).length();
        if distance < self.boss_type.scale() {
            self.take_damage(20.0);
            true
        } else {
            false
        }
    }

    pub fn draw(&self) {
        let scale = self.boss_type.scale();
        let color = self.boss_type.color();
        let accent = self.boss_type.accent_color();

        // Draw main body
        draw_cube(
            self.position,
            vec3(scale * 2.0, scale * 1.5, scale * 2.0),
            None,
            color,
        );

        // Draw wireframe for retro look
        draw_cube_wires(
            self.position,
            vec3(scale * 2.0, scale * 1.5, scale * 2.0),
            WHITE,
        );

        // Draw accent parts based on boss type
        for i in 0..3 {
            let offset = vec3(
                (i as f32 - 1.0) * scale * 0.8,
                scale * 0.5,
                0.0,
            );
            draw_cube(
                self.position + offset,
                vec3(scale * 0.4, scale * 0.4, scale * 0.4),
                None,
                accent,
            );
        }

        // Draw projectiles
        for proj in &self.projectiles {
            draw_sphere(proj.position, 0.3, None, accent);
        }

        // Visual effect based on phase
        if self.phase >= 2 {
            // Pulsing glow effect
            let pulse = (self.time_alive * 3.0).sin() * 0.5 + 0.5;
            let glow_color = Color::new(color.r, color.g, color.b, pulse * 0.3);

            draw_cube(
                self.position,
                vec3(scale * 2.2, scale * 1.7, scale * 2.2),
                None,
                glow_color,
            );
        }
    }

    pub fn projectiles(&self) -> &Vec<BossProjectile> {
        &self.projectiles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boss_type_from_continent() {
        assert_eq!(
            BossType::from_continent(Continent::Tutorial),
            BossType::TutorialBoss
        );
        assert_eq!(
            BossType::from_continent(Continent::NorthAmerica),
            BossType::MountainGuardian
        );
    }

    #[test]
    fn test_boss_creation() {
        let boss = Boss::new(BossType::TutorialBoss, vec3(0.0, 5.0, 50.0));
        assert_eq!(boss.health(), BossType::TutorialBoss.max_health());
        assert_eq!(boss.phase(), 1);
        assert!(!boss.is_defeated());
    }

    #[test]
    fn test_boss_damage() {
        let mut boss = Boss::new(BossType::TutorialBoss, vec3(0.0, 5.0, 50.0));
        let initial_health = boss.health();

        boss.take_damage(50.0);
        assert_eq!(boss.health(), initial_health - 50.0);
        assert!(!boss.is_defeated());

        boss.take_damage(1000.0);
        assert!(boss.is_defeated());
        assert_eq!(boss.health(), 0.0);
    }

    #[test]
    fn test_boss_phases() {
        let mut boss = Boss::new(BossType::TutorialBoss, vec3(0.0, 5.0, 50.0));

        // Phase 1
        assert_eq!(boss.phase(), 1);

        // Phase 2 (below 66% health)
        boss.take_damage(boss.max_health() * 0.4);
        boss.update(0.016, vec3(0.0, 0.0, 0.0));
        assert_eq!(boss.phase(), 2);

        // Phase 3 (below 33% health)
        boss.take_damage(boss.max_health() * 0.4);
        boss.update(0.016, vec3(0.0, 0.0, 0.0));
        assert_eq!(boss.phase(), 3);
    }

    #[test]
    fn test_boss_health_percentage() {
        let mut boss = Boss::new(BossType::TutorialBoss, vec3(0.0, 5.0, 50.0));

        assert_eq!(boss.health_percentage(), 1.0);

        boss.take_damage(boss.max_health() * 0.5);
        assert!((boss.health_percentage() - 0.5).abs() < 0.01);
    }
}
