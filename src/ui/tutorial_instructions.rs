use macroquad::prelude::*;

pub struct TutorialInstructions {
    time: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TutorialAction {
    None,
    Start,
    Back,
}

impl TutorialInstructions {
    pub fn new() -> Self {
        Self {
            time: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) -> TutorialAction {
        self.time += dt;

        // Check for input
        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
            TutorialAction::Start
        } else if is_key_pressed(KeyCode::Escape) {
            TutorialAction::Back
        } else {
            TutorialAction::None
        }
    }

    pub fn draw(&self) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Background
        clear_background(Color::from_rgba(0, 5, 15, 255));

        // Title
        let title = "TUTORIAL - HOW TO PLAY";
        let title_size = 40;
        let title_width = measure_text(title, None, title_size, 1.0).width;
        draw_text(
            title,
            screen_w / 2.0 - title_width / 2.0,
            80.0,
            title_size as f32,
            Color::from_rgba(0, 255, 255, 255),
        );

        // Instruction panels
        let panel_x = screen_w / 2.0 - 400.0;
        let panel_width = 800.0;
        let mut y = 140.0;

        // === CONTROLS ===
        draw_instruction_panel(
            panel_x,
            y,
            panel_width,
            "CONTROLS",
            &[
                "↑ ↓ ← → or WASD - Move your glider",
                "SHIFT or TAB - Speed boost (consumes boost energy)",
                "SPACE - Fire weapon",
                "ESC - Pause / Return to menu",
            ],
            Color::from_rgba(0, 255, 255, 255),
        );
        y += 160.0;

        // === OBJECTIVES ===
        draw_instruction_panel(
            panel_x,
            y,
            panel_width,
            "OBJECTIVES",
            &[
                "Survive for 4 minutes to complete the tutorial",
                "Fly through cyan rings for bonus points (+100)",
                "Collect powerups (health, weapons, ammo)",
                "Reach 7 checkpoints before the boss",
                "Defeat the Training Drone boss at 3:30",
            ],
            Color::from_rgba(255, 215, 0, 255),
        );
        y += 180.0;

        // === HAZARDS ===
        draw_instruction_panel(
            panel_x,
            y,
            panel_width,
            "AVOID",
            &[
                "Obstacles (mountains, boulders, turbines)",
                "Enemy drones (red/orange cubes)",
                "Running out of health",
            ],
            Color::from_rgba(255, 100, 100, 255),
        );
        y += 130.0;

        // === TIPS ===
        draw_instruction_panel(
            panel_x,
            y,
            panel_width,
            "TIPS",
            &[
                "Powerups gravitate toward you - get close!",
                "Checkpoints save your progress",
                "Respawning clears nearby hazards",
                "Watch your health and boost bars (top left)",
                "Use boost strategically in tight situations",
            ],
            Color::from_rgba(0, 255, 0, 255),
        );

        // === START PROMPT ===
        let prompt = "PRESS SPACE TO START TUTORIAL";
        let prompt_size = 28;
        let prompt_width = measure_text(prompt, None, prompt_size, 1.0).width;

        // Pulsing effect
        let pulse = ((self.time * 3.0).sin() * 0.5 + 0.5) * 100.0 + 155.0;
        let pulse_color = Color::from_rgba(pulse as u8, 255, pulse as u8, 255);

        draw_text(
            prompt,
            screen_w / 2.0 - prompt_width / 2.0,
            screen_h - 80.0,
            prompt_size as f32,
            pulse_color,
        );

        // Back hint
        let back_hint = "ESC - Back to Level Select";
        let back_size = 18;
        let back_width = measure_text(back_hint, None, back_size, 1.0).width;
        draw_text(
            back_hint,
            screen_w / 2.0 - back_width / 2.0,
            screen_h - 40.0,
            back_size as f32,
            Color::from_rgba(150, 150, 150, 255),
        );
    }

    pub fn reset(&mut self) {
        self.time = 0.0;
    }
}

fn draw_instruction_panel(x: f32, y: f32, width: f32, title: &str, items: &[&str], color: Color) {
    let padding = 15.0;
    let line_height = 25.0;
    let height = padding * 2.0 + 30.0 + (items.len() as f32 * line_height);

    // Panel background
    draw_rectangle(x, y, width, height, Color::from_rgba(10, 20, 30, 220));
    draw_rectangle_lines(x, y, width, height, 2.0, color);

    // Title
    let title_size = 22;
    draw_text(
        title,
        x + padding,
        y + padding + 20.0,
        title_size as f32,
        color,
    );

    // Items
    let item_size = 18;
    for (i, item) in items.iter().enumerate() {
        draw_text(
            &format!("• {}", item),
            x + padding + 10.0,
            y + padding + 50.0 + (i as f32 * line_height),
            item_size as f32,
            WHITE,
        );
    }
}

impl Default for TutorialInstructions {
    fn default() -> Self {
        Self::new()
    }
}
