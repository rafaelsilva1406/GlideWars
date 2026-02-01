use macroquad::prelude::*;
use crate::player::Player;

pub struct GameCamera {
    camera: Camera3D,
    offset: Vec3,
}

impl GameCamera {
    pub fn new() -> Self {
        Self {
            camera: Camera3D {
                position: vec3(0.0, 5.0, -10.0),
                up: vec3(0.0, 1.0, 0.0),
                target: vec3(0.0, 0.0, 0.0),
                ..Default::default()
            },
            offset: vec3(0.0, 5.0, -10.0),
        }
    }

    pub fn update(&mut self, player: &Player) {
        let player_pos = player.position();

        // Camera follows player with slight offset
        // Creates 2.5D effect - 3D perspective but follows in 2D plane
        self.camera.position = player_pos + self.offset;
        self.camera.target = player_pos + vec3(0.0, 0.0, 5.0); // Look ahead

        // Add subtle camera shake based on speed
        let shake = (get_time() as f32 * 20.0).sin() * 0.05;
        self.camera.position.y += shake;
    }

    pub fn get_camera(&self) -> &Camera3D {
        &self.camera
    }
}
