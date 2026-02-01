use macroquad::prelude::*;
use crate::assets::Continent;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LevelSelectAction {
    None,
    StartLevel(Continent),
    Back,
}

pub struct LevelSelectScreen {
    time: f32,
    rotation: f32,
    selected_continent: usize,
    continents: Vec<ContinentDisplay>,
    unlocked_continents: Vec<bool>,
}

struct ContinentDisplay {
    continent: Continent,
    name: &'static str,
    angle: f32,
    color: Color,
}

impl LevelSelectScreen {
    pub fn new() -> Self {
        let continents = vec![
            ContinentDisplay {
                continent: Continent::Tutorial,
                name: "TUTORIAL",
                angle: 0.0,
                color: Color::from_rgba(150, 150, 200, 255),
            },
            ContinentDisplay {
                continent: Continent::NorthAmerica,
                name: "NORTH AMERICA",
                angle: std::f32::consts::PI * 2.0 / 7.0,
                color: Color::from_rgba(139, 90, 43, 255),
            },
            ContinentDisplay {
                continent: Continent::SouthAmerica,
                name: "SOUTH AMERICA",
                angle: std::f32::consts::PI * 4.0 / 7.0,
                color: Color::from_rgba(50, 150, 50, 255),
            },
            ContinentDisplay {
                continent: Continent::Europe,
                name: "EUROPE",
                angle: std::f32::consts::PI * 6.0 / 7.0,
                color: Color::from_rgba(150, 150, 180, 255),
            },
            ContinentDisplay {
                continent: Continent::Asia,
                name: "ASIA",
                angle: std::f32::consts::PI * 8.0 / 7.0,
                color: Color::from_rgba(255, 200, 50, 255),
            },
            ContinentDisplay {
                continent: Continent::Africa,
                name: "AFRICA",
                angle: std::f32::consts::PI * 10.0 / 7.0,
                color: Color::from_rgba(255, 150, 50, 255),
            },
            ContinentDisplay {
                continent: Continent::Oceania,
                name: "OCEANIA",
                angle: std::f32::consts::PI * 12.0 / 7.0,
                color: Color::from_rgba(50, 150, 255, 255),
            },
        ];

        // Tutorial is always unlocked, others locked initially
        let unlocked_continents = vec![true, false, false, false, false, false, false];

        Self {
            time: 0.0,
            rotation: 0.0,
            selected_continent: 0,
            continents,
            unlocked_continents,
        }
    }

    pub fn update(&mut self, dt: f32) -> LevelSelectAction {
        self.time += dt;

        // Rotate globe slowly
        self.rotation += dt * 0.3;

        // Navigation
        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
            self.selected_continent = if self.selected_continent == 0 {
                self.continents.len() - 1
            } else {
                self.selected_continent - 1
            };
        }

        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
            self.selected_continent = (self.selected_continent + 1) % self.continents.len();
        }

        // Select continent
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            if self.unlocked_continents[self.selected_continent] {
                return LevelSelectAction::StartLevel(self.continents[self.selected_continent].continent);
            }
        }

        // Tutorial shortcut
        if is_key_pressed(KeyCode::T) {
            return LevelSelectAction::StartLevel(Continent::Tutorial);
        }

        // Back
        if is_key_pressed(KeyCode::Escape) {
            return LevelSelectAction::Back;
        }

        LevelSelectAction::None
    }

    pub fn draw(&self) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Background
        clear_background(Color::from_rgba(0, 0, 10, 255));
        self.draw_stars();

        // Title
        let title = "SELECT CONTINENT";
        let title_size = 45.0;
        let text_width = measure_text(title, None, title_size as u16, 1.0).width;

        draw_text(
            title,
            screen_w / 2.0 - text_width / 2.0,
            80.0,
            title_size,
            Color::from_rgba(0, 255, 255, 255),
        );

        // Draw rotating globe/map
        self.draw_globe(screen_w / 2.0, screen_h / 2.0);

        // Draw continent info panel
        self.draw_continent_info();

        // Tutorial prompt
        let tutorial_text = "PRESS T FOR TUTORIAL";
        let tutorial_size = 24.0;
        let tutorial_width = measure_text(tutorial_text, None, tutorial_size as u16, 1.0).width;
        let pulse = (self.time * 4.0).sin() * 0.3 + 0.7;

        draw_text(
            tutorial_text,
            screen_w / 2.0 - tutorial_width / 2.0,
            150.0,
            tutorial_size,
            Color::from_rgba(255, 255, 0, (pulse * 255.0) as u8),
        );

        // Controls
        let hint = "← → SELECT | ENTER START | ESC BACK";
        let hint_size = 16.0;
        let hint_width = measure_text(hint, None, hint_size as u16, 1.0).width;

        draw_text(
            hint,
            screen_w / 2.0 - hint_width / 2.0,
            screen_h - 40.0,
            hint_size,
            Color::from_rgba(200, 200, 200, 180),
        );
    }

    fn draw_stars(&self) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Draw twinkling stars
        for i in 0..100 {
            let x = (i as f32 * 73.7) % screen_w;
            let y = (i as f32 * 57.3) % screen_h;
            let twinkle = ((self.time * 2.0 + i as f32).sin() * 0.5 + 0.5 * 255.0) as u8;

            draw_circle(
                x,
                y,
                1.5,
                Color::from_rgba(255, 255, 255, twinkle),
            );
        }
    }

    fn draw_globe(&self, center_x: f32, center_y: f32) {
        let globe_radius = 180.0;

        // Draw globe outline
        draw_circle_lines(
            center_x,
            center_y,
            globe_radius,
            3.0,
            Color::from_rgba(0, 150, 200, 200),
        );

        // Draw continents around the globe
        for (i, continent) in self.continents.iter().enumerate() {
            let is_selected = i == self.selected_continent;
            let is_unlocked = self.unlocked_continents[i];

            // Calculate position on circle
            let angle = continent.angle + self.rotation;
            let distance = if is_selected { globe_radius + 40.0 } else { globe_radius };

            let x = center_x + angle.cos() * distance;
            let y = center_y + angle.sin() * distance;

            // Draw continent marker
            let size = if is_selected { 25.0 } else { 18.0 };
            let alpha = if is_unlocked { 255 } else { 100 };

            // Continent circle
            let marker_color = if is_unlocked {
                continent.color
            } else {
                Color::from_rgba(80, 80, 80, alpha)
            };

            draw_circle(x, y, size, marker_color);

            // Glow for selected
            if is_selected {
                let glow_pulse = (self.time * 6.0).sin() * 0.3 + 0.7;
                draw_circle_lines(
                    x,
                    y,
                    size + 5.0,
                    3.0,
                    Color::from_rgba(0, 255, 255, (glow_pulse * 255.0) as u8),
                );
            }

            // Lock icon for locked continents
            if !is_unlocked {
                draw_text(
                    "🔒",
                    x - 8.0,
                    y + 8.0,
                    16.0,
                    Color::from_rgba(200, 200, 200, 255),
                );
            }

            // Connect to globe
            let line_alpha = if is_selected { 200 } else { 100 };
            draw_line(
                center_x + angle.cos() * globe_radius,
                center_y + angle.sin() * globe_radius,
                x,
                y,
                1.0,
                Color::from_rgba(0, 200, 255, line_alpha),
            );
        }

        // Draw equator and meridians
        for i in 0..8 {
            let angle = i as f32 * std::f32::consts::PI / 4.0;
            let x1 = center_x + angle.cos() * globe_radius;
            let y1 = center_y + angle.sin() * globe_radius;
            let opposite_angle = angle + std::f32::consts::PI;
            let x2 = center_x + opposite_angle.cos() * globe_radius;
            let y2 = center_y + opposite_angle.sin() * globe_radius;

            draw_line(
                x1, y1, x2, y2,
                1.0,
                Color::from_rgba(0, 100, 150, 80),
            );
        }
    }

    fn draw_continent_info(&self) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        let continent = &self.continents[self.selected_continent];
        let is_unlocked = self.unlocked_continents[self.selected_continent];

        // Info panel
        let panel_y = screen_h - 180.0;
        let panel_width = 600.0;
        let panel_height = 120.0;
        let panel_x = screen_w / 2.0 - panel_width / 2.0;

        // Panel background
        draw_rectangle(
            panel_x,
            panel_y,
            panel_width,
            panel_height,
            Color::from_rgba(0, 20, 40, 200),
        );

        draw_rectangle_lines(
            panel_x,
            panel_y,
            panel_width,
            panel_height,
            2.0,
            Color::from_rgba(0, 255, 255, 255),
        );

        // Continent name
        let name_size = 32.0;
        let name_width = measure_text(continent.name, None, name_size as u16, 1.0).width;
        draw_text(
            continent.name,
            screen_w / 2.0 - name_width / 2.0,
            panel_y + 40.0,
            name_size,
            if is_unlocked {
                Color::from_rgba(0, 255, 255, 255)
            } else {
                Color::from_rgba(150, 150, 150, 255)
            },
        );

        // Status
        let status_text = if is_unlocked {
            "PRESS ENTER TO START"
        } else {
            "LOCKED - COMPLETE PREVIOUS LEVELS"
        };

        let status_size = 18.0;
        let status_width = measure_text(status_text, None, status_size as u16, 1.0).width;
        draw_text(
            status_text,
            screen_w / 2.0 - status_width / 2.0,
            panel_y + 75.0,
            status_size,
            if is_unlocked {
                Color::from_rgba(0, 255, 0, 255)
            } else {
                Color::from_rgba(255, 100, 100, 255)
            },
        );

        // Level details
        if is_unlocked {
            let details = "5 MINUTES | 3 CHECKPOINTS | BOSS BATTLE";
            let details_size = 14.0;
            let details_width = measure_text(details, None, details_size as u16, 1.0).width;
            draw_text(
                details,
                screen_w / 2.0 - details_width / 2.0,
                panel_y + 100.0,
                details_size,
                Color::from_rgba(200, 200, 200, 200),
            );
        }
    }

    pub fn unlock_continent(&mut self, continent: Continent) {
        for (i, display) in self.continents.iter().enumerate() {
            if display.continent == continent {
                self.unlocked_continents[i] = true;
                break;
            }
        }
    }
}
