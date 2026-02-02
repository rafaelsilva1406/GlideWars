use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub enum Weapon {
    None,
    Laser,
    Missile,
    Spread,
}

impl Weapon {
    pub fn name(&self) -> &str {
        match self {
            Weapon::None => "NONE",
            Weapon::Laser => "LASER",
            Weapon::Missile => "MISSILE",
            Weapon::Spread => "SPREAD",
        }
    }
}

pub struct Projectile {
    pub position: Vec3,
    pub velocity: Vec3,
    pub lifetime: f32,
}

pub struct Player {
    position: Vec3,
    velocity: Vec3,
    health: f32,
    max_health: f32,
    weapon: Weapon,
    ammo: u32,
    projectiles: Vec<Projectile>,
    shoot_cooldown: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: vec3(0.0, 0.0, 0.0),
            velocity: vec3(0.0, 0.0, 0.0),
            health: 100.0,
            max_health: 100.0,
            weapon: Weapon::None,
            ammo: 0,
            projectiles: Vec::new(),
            shoot_cooldown: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Glider physics - always moving forward
        let forward_speed = 10.0;
        self.velocity.z = forward_speed;

        // Horizontal controls
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.velocity.x = -5.0;
        } else if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.velocity.x = 5.0;
        } else {
            self.velocity.x *= 0.9; // Damping
        }

        // Apply gravity (constant downward force)
        let gravity = -4.0;
        self.velocity.y += gravity * dt;

        // Vertical controls (work against gravity)
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            // Lift force to counter gravity and climb
            self.velocity.y += 8.0 * dt;
        } else if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            // Dive faster
            self.velocity.y -= 5.0 * dt;
        }

        // Cap vertical velocity to prevent excessive speeds
        self.velocity.y = self.velocity.y.clamp(-8.0, 5.0);

        // Update position
        self.position += self.velocity * dt;

        // Keep player in bounds (left-right and up-down)
        if self.position.x < -8.0 {
            self.position.x = -8.0;
            self.velocity.x = 0.0;
        }
        if self.position.x > 8.0 {
            self.position.x = 8.0;
            self.velocity.x = 0.0;
        }

        // Ground level
        if self.position.y < -1.0 {
            self.position.y = -1.0;
            self.velocity.y = self.velocity.y.max(0.0); // Bounce off ground
        }

        // Height ceiling (much lower to prevent flying over everything)
        if self.position.y > 6.0 {
            self.position.y = 6.0;
            self.velocity.y = self.velocity.y.min(0.0); // Can't go higher
        }

        // Shooting
        if self.shoot_cooldown > 0.0 {
            self.shoot_cooldown -= dt;
        }

        if is_key_down(KeyCode::Space) && self.shoot_cooldown <= 0.0 {
            self.shoot();
        }

        // Update projectiles
        self.projectiles.retain_mut(|proj| {
            proj.position += proj.velocity * dt;
            proj.lifetime -= dt;
            proj.lifetime > 0.0 && proj.position.z < self.position.z + 50.0
        });
    }

    fn shoot(&mut self) {
        match self.weapon {
            Weapon::None => {}
            Weapon::Laser => {
                if self.ammo > 0 {
                    self.projectiles.push(Projectile {
                        position: self.position + vec3(0.0, 0.0, 1.0),
                        velocity: vec3(0.0, 0.0, 30.0),
                        lifetime: 2.0,
                    });
                    self.ammo -= 1;
                    self.shoot_cooldown = 0.15;
                }
            }
            Weapon::Missile => {
                if self.ammo > 0 {
                    self.projectiles.push(Projectile {
                        position: self.position + vec3(0.0, 0.0, 1.0),
                        velocity: vec3(0.0, 0.0, 20.0),
                        lifetime: 3.0,
                    });
                    self.ammo -= 1;
                    self.shoot_cooldown = 0.5;
                }
            }
            Weapon::Spread => {
                if self.ammo > 0 {
                    for i in -1..=1 {
                        self.projectiles.push(Projectile {
                            position: self.position + vec3(0.0, 0.0, 1.0),
                            velocity: vec3(i as f32 * 5.0, 0.0, 25.0),
                            lifetime: 2.0,
                        });
                    }
                    self.ammo -= 1;
                    self.shoot_cooldown = 0.3;
                }
            }
        }
    }

    pub fn draw(&self) {
        // Draw glider (low-poly retro style)
        // Body
        draw_cube(self.position, vec3(0.5, 0.3, 1.0), None, Color::from_rgba(0, 200, 255, 255));

        // Wings
        draw_cube(
            self.position + vec3(-1.0, 0.0, 0.0),
            vec3(1.0, 0.1, 0.5),
            None,
            Color::from_rgba(0, 150, 200, 255)
        );
        draw_cube(
            self.position + vec3(1.0, 0.0, 0.0),
            vec3(1.0, 0.1, 0.5),
            None,
            Color::from_rgba(0, 150, 200, 255)
        );

        // Draw projectiles
        for proj in &self.projectiles {
            let color = match self.weapon {
                Weapon::Laser => Color::from_rgba(255, 0, 0, 255),
                Weapon::Missile => Color::from_rgba(255, 255, 0, 255),
                Weapon::Spread => Color::from_rgba(255, 0, 255, 255),
                _ => WHITE,
            };
            draw_sphere(proj.position, 0.2, None, color);
        }
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn take_damage(&mut self, damage: f32) {
        self.health -= damage;
        if self.health < 0.0 {
            self.health = 0.0;
        }
    }

    pub fn heal(&mut self, amount: f32) {
        self.health += amount;
        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }

    pub fn is_dead(&self) -> bool {
        self.health <= 0.0
    }

    pub fn set_weapon(&mut self, weapon: Weapon, ammo: u32) {
        self.weapon = weapon;
        self.ammo = ammo;
    }

    pub fn add_ammo(&mut self, amount: u32) {
        self.ammo += amount;
    }

    pub fn current_weapon(&self) -> Option<&str> {
        match self.weapon {
            Weapon::None => None,
            _ => Some(self.weapon.name()),
        }
    }

    pub fn ammo(&self) -> u32 {
        self.ammo
    }

    pub fn health(&self) -> f32 {
        self.health
    }

    pub fn get_projectiles(&self) -> &Vec<Projectile> {
        &self.projectiles
    }

    pub fn clear_projectile(&mut self, index: usize) {
        if index < self.projectiles.len() {
            self.projectiles.remove(index);
        }
    }

    pub fn restore_from_checkpoint(&mut self, position: Vec3, health: f32, weapon: Weapon, ammo: u32) {
        self.position = position;
        self.health = health;
        self.weapon = weapon;
        self.ammo = ammo;
        self.velocity = Vec3::ZERO;
        self.projectiles.clear();
        self.shoot_cooldown = 0.0;
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_initialization() {
        let player = Player::new();
        assert_eq!(player.health(), 100.0);
        assert_eq!(player.position(), vec3(0.0, 0.0, 0.0));
        assert_eq!(player.velocity, vec3(0.0, 0.0, 0.0));
        assert!(!player.is_dead());
    }

    #[test]
    fn test_player_take_damage() {
        let mut player = Player::new();
        player.take_damage(30.0);
        assert_eq!(player.health(), 70.0);
        assert!(!player.is_dead());
    }

    #[test]
    fn test_player_death() {
        let mut player = Player::new();
        player.take_damage(100.0);
        assert_eq!(player.health(), 0.0);
        assert!(player.is_dead());
    }

    #[test]
    fn test_player_overkill_damage() {
        let mut player = Player::new();
        player.take_damage(150.0);
        assert_eq!(player.health(), 0.0);
        assert!(player.is_dead());
    }

    #[test]
    fn test_player_heal() {
        let mut player = Player::new();
        player.take_damage(50.0);
        assert_eq!(player.health(), 50.0);

        player.heal(30.0);
        assert_eq!(player.health(), 80.0);
    }

    #[test]
    fn test_player_heal_does_not_exceed_max() {
        let mut player = Player::new();
        player.take_damage(20.0);
        player.heal(50.0); // Try to overheal
        assert_eq!(player.health(), 100.0);
    }

    #[test]
    fn test_weapon_switching() {
        let mut player = Player::new();

        // Start with no weapon
        assert_eq!(player.current_weapon(), None);

        // Pickup laser
        player.pickup_weapon(Weapon::Laser);
        assert_eq!(player.current_weapon(), Some("Laser"));
        assert_eq!(player.ammo(), 100);

        // Pickup missiles
        player.pickup_weapon(Weapon::Missile);
        assert_eq!(player.current_weapon(), Some("Missile"));
        assert_eq!(player.ammo(), 50);
    }

    #[test]
    fn test_shooting_without_weapon() {
        let mut player = Player::new();
        assert_eq!(player.projectiles.len(), 0);

        player.shoot();
        // Should not create projectiles without weapon
        assert_eq!(player.projectiles.len(), 0);
    }

    #[test]
    fn test_shooting_with_weapon() {
        let mut player = Player::new();
        player.pickup_weapon(Weapon::Laser);

        player.shoot();
        assert_eq!(player.projectiles.len(), 1);
        assert_eq!(player.ammo(), 99);
    }

    #[test]
    fn test_shooting_without_ammo() {
        let mut player = Player::new();
        player.pickup_weapon(Weapon::Laser);
        player.ammo = 0;

        player.shoot();
        // Should not create projectiles without ammo
        assert_eq!(player.projectiles.len(), 0);
    }

    #[test]
    fn test_checkpoint_restore() {
        let mut player = Player::new();
        player.take_damage(50.0);
        player.position = vec3(10.0, 5.0, 100.0);

        // Restore from checkpoint
        let checkpoint_pos = vec3(5.0, 2.0, 50.0);
        player.restore_from_checkpoint(checkpoint_pos, 80.0, Weapon::Laser, 75);

        assert_eq!(player.health(), 80.0);
        assert_eq!(player.position(), checkpoint_pos);
        assert_eq!(player.current_weapon(), Some("Laser"));
        assert_eq!(player.ammo(), 75);
        assert_eq!(player.velocity, vec3(0.0, 0.0, 0.0));
        assert_eq!(player.projectiles.len(), 0);
    }

    #[test]
    fn test_gravity_applies_downward_force() {
        let mut player = Player::new();
        let initial_y = player.position.y;

        // Update with no input (gravity should pull down)
        player.update(0.1);

        // Y velocity should be negative (downward)
        assert!(player.velocity.y < 0.0);
    }

    #[test]
    fn test_velocity_clamping() {
        let mut player = Player::new();

        // Artificially set extreme velocity
        player.velocity.y = 100.0;
        player.velocity.x = 50.0;

        player.update(0.1);

        // Velocity should be clamped
        assert!(player.velocity.y <= 5.0);
        assert!(player.velocity.x <= 10.0);
        assert!(player.velocity.y >= -8.0);
        assert!(player.velocity.x >= -10.0);
    }

    #[test]
    fn test_height_ceiling() {
        let mut player = Player::new();
        player.position.y = 10.0; // Above ceiling

        player.update(0.1);

        // Should be clamped to ceiling
        assert!(player.position.y <= 6.0);
    }

    #[test]
    fn test_ground_bounce() {
        let mut player = Player::new();
        player.position.y = -2.0; // Below ground
        player.velocity.y = -5.0; // Moving down

        player.update(0.1);

        // Should be at ground level
        assert!(player.position.y >= -1.0);
    }

    #[test]
    fn test_projectile_cleanup() {
        let mut player = Player::new();
        player.pickup_weapon(Weapon::Laser);
        player.shoot();

        // Move projectile far away
        player.projectiles[0].position.z = player.position.z + 100.0;

        player.update(0.1);

        // Projectile should be removed (beyond 80 unit range)
        assert_eq!(player.projectiles.len(), 0);
    }
}
