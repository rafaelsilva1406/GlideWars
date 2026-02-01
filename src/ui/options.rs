use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptionsAction {
    None,
    Back,
}

pub struct OptionsMenu {
    time: f32,
    selected_index: usize,
    sound_volume: f32,
    music_volume: f32,
    difficulty: usize, // 0=Easy, 1=Normal, 2=Hard
}

impl OptionsMenu {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            selected_index: 0,
            sound_volume: 100.0,
            music_volume: 100.0,
            difficulty: 1, // Default to Normal
        }
    }

    pub fn update(&mut self, dt: f32) -> OptionsAction {
        self.time += dt;

        // Navigation
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            if self.selected_index > 0 {
                self.selected_index -= 1;
            }
        }

        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            if self.selected_index < 3 { // 4 options (0-3)
                self.selected_index += 1;
            }
        }

        // Value adjustment
        match self.selected_index {
            0 => { // Sound Volume
                if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                    self.sound_volume = (self.sound_volume - 100.0 * dt).max(0.0);
                }
                if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                    self.sound_volume = (self.sound_volume + 100.0 * dt).min(100.0);
                }
            }
            1 => { // Music Volume
                if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                    self.music_volume = (self.music_volume - 100.0 * dt).max(0.0);
                }
                if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                    self.music_volume = (self.music_volume + 100.0 * dt).min(100.0);
                }
            }
            2 => { // Difficulty
                if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
                    if self.difficulty > 0 {
                        self.difficulty -= 1;
                    }
                }
                if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
                    if self.difficulty < 2 {
                        self.difficulty += 1;
                    }
                }
            }
            _ => {}
        }

        // Back button
        if self.selected_index == 3 && (is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space)) {
            return OptionsAction::Back;
        }

        if is_key_pressed(KeyCode::Escape) {
            return OptionsAction::Back;
        }

        OptionsAction::None
    }

    pub fn draw(&self) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Background
        clear_background(Color::from_rgba(0, 0, 20, 255));

        // Title
        let title = "OPTIONS";
        let title_size = 50.0;
        let text_width = measure_text(title, None, title_size as u16, 1.0).width;

        draw_text(
            title,
            screen_w / 2.0 - text_width / 2.0,
            100.0,
            title_size,
            Color::from_rgba(0, 255, 255, 255),
        );

        // Options
        let start_y = 200.0;
        let spacing = 80.0;

        // Sound Volume
        self.draw_option(
            0,
            "SOUND VOLUME",
            &format!("{:.0}%", self.sound_volume),
            start_y,
            self.sound_volume / 100.0,
        );

        // Music Volume
        self.draw_option(
            1,
            "MUSIC VOLUME",
            &format!("{:.0}%", self.music_volume),
            start_y + spacing,
            self.music_volume / 100.0,
        );

        // Difficulty
        let difficulty_text = match self.difficulty {
            0 => "EASY",
            1 => "NORMAL",
            2 => "HARD",
            _ => "NORMAL",
        };
        self.draw_option(
            2,
            "DIFFICULTY",
            difficulty_text,
            start_y + spacing * 2.0,
            0.0,
        );

        // Back button
        let back_y = start_y + spacing * 3.0;
        let is_selected = self.selected_index == 3;

        if is_selected {
            let pulse = (self.time * 8.0).sin() * 0.5 + 0.5;
            draw_text(
                ">",
                screen_w / 2.0 - 150.0,
                back_y,
                30.0,
                Color::from_rgba(0, 255, 255, (pulse * 255.0) as u8),
            );
        }

        let back_size = if is_selected { 35.0 } else { 30.0 };
        let back_color = if is_selected {
            Color::from_rgba(0, 255, 255, 255)
        } else {
            Color::from_rgba(150, 150, 150, 200)
        };

        let back_text = "BACK";
        let back_width = measure_text(back_text, None, back_size as u16, 1.0).width;
        draw_text(
            back_text,
            screen_w / 2.0 - back_width / 2.0,
            back_y,
            back_size,
            back_color,
        );

        // Controls hint
        let hint = "↑↓ NAVIGATE | ←→ ADJUST | ESC BACK";
        let hint_size = 16.0;
        let hint_width = measure_text(hint, None, hint_size as u16, 1.0).width;

        draw_text(
            hint,
            screen_w / 2.0 - hint_width / 2.0,
            screen_h - 60.0,
            hint_size,
            Color::from_rgba(200, 200, 200, 180),
        );
    }

    fn draw_option(&self, index: usize, label: &str, value: &str, y: f32, bar_fill: f32) {
        let screen_w = screen_width();
        let is_selected = self.selected_index == index;

        // Selection indicator
        if is_selected {
            let pulse = (self.time * 8.0).sin() * 0.5 + 0.5;
            draw_text(
                ">",
                screen_w / 2.0 - 250.0,
                y,
                30.0,
                Color::from_rgba(0, 255, 255, (pulse * 255.0) as u8),
            );
        }

        // Label
        let label_size = if is_selected { 28.0 } else { 24.0 };
        let label_color = if is_selected {
            Color::from_rgba(0, 255, 255, 255)
        } else {
            Color::from_rgba(200, 200, 200, 200)
        };

        draw_text(
            label,
            screen_w / 2.0 - 200.0,
            y,
            label_size,
            label_color,
        );

        // Value or bar
        if bar_fill > 0.0 {
            // Draw progress bar for volume
            let bar_width = 200.0;
            let bar_height = 20.0;
            let bar_x = screen_w / 2.0 + 50.0;
            let bar_y = y - 20.0;

            // Background
            draw_rectangle(
                bar_x,
                bar_y,
                bar_width,
                bar_height,
                Color::from_rgba(40, 40, 60, 255),
            );

            // Fill
            let fill_color = if is_selected {
                Color::from_rgba(0, 255, 255, 200)
            } else {
                Color::from_rgba(0, 200, 200, 150)
            };

            draw_rectangle(
                bar_x,
                bar_y,
                bar_width * bar_fill,
                bar_height,
                fill_color,
            );

            // Border
            draw_rectangle_lines(
                bar_x,
                bar_y,
                bar_width,
                bar_height,
                2.0,
                Color::from_rgba(0, 255, 255, 255),
            );
        }

        // Value text
        let value_size = if is_selected { 28.0 } else { 24.0 };
        draw_text(
            value,
            screen_w / 2.0 + 270.0,
            y,
            value_size,
            Color::from_rgba(255, 255, 255, 255),
        );
    }

    pub fn get_sound_volume(&self) -> f32 {
        self.sound_volume
    }

    pub fn get_music_volume(&self) -> f32 {
        self.music_volume
    }

    pub fn get_difficulty(&self) -> usize {
        self.difficulty
    }
}
