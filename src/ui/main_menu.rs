use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuAction {
    None,
    Start,
    Options,
}

pub struct MainMenu {
    time: f32,
    selected_index: usize,
    menu_items: Vec<String>,
}

impl MainMenu {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            selected_index: 0,
            menu_items: vec![
                "START GAME".to_string(),
                "OPTIONS".to_string(),
            ],
        }
    }

    pub fn update(&mut self, dt: f32) -> MenuAction {
        self.time += dt;

        // Handle input
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            if self.selected_index > 0 {
                self.selected_index -= 1;
            }
        }

        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            if self.selected_index < self.menu_items.len() - 1 {
                self.selected_index += 1;
            }
        }

        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            return match self.selected_index {
                0 => MenuAction::Start,
                1 => MenuAction::Options,
                _ => MenuAction::None,
            };
        }

        MenuAction::None
    }

    pub fn draw(&self) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Animated background
        self.draw_background();

        // Title
        let title = "GLIDE WARS";
        let title_size = 60.0;
        let pulse = (self.time * 2.0).sin() * 0.2 + 0.8;
        let text_width = measure_text(title, None, title_size as u16, 1.0).width;

        // Title glow
        draw_text(
            title,
            screen_w / 2.0 - text_width / 2.0,
            screen_h / 3.0,
            title_size,
            Color::from_rgba(0, 255, 255, (pulse * 255.0) as u8),
        );

        // Menu items
        let menu_start_y = screen_h / 2.0;
        let item_spacing = 60.0;

        for (i, item) in self.menu_items.iter().enumerate() {
            let y = menu_start_y + i as f32 * item_spacing;
            let is_selected = i == self.selected_index;

            // Selection indicator
            if is_selected {
                let indicator_pulse = (self.time * 8.0).sin() * 0.5 + 0.5;
                let indicator_x = screen_w / 2.0 - 200.0;

                draw_text(
                    ">",
                    indicator_x,
                    y,
                    30.0,
                    Color::from_rgba(0, 255, 255, (indicator_pulse * 255.0) as u8),
                );
            }

            // Menu item text
            let item_size = if is_selected { 35.0 } else { 30.0 };
            let item_color = if is_selected {
                Color::from_rgba(0, 255, 255, 255)
            } else {
                Color::from_rgba(150, 150, 150, 200)
            };

            let item_width = measure_text(item, None, item_size as u16, 1.0).width;
            draw_text(
                item,
                screen_w / 2.0 - item_width / 2.0,
                y,
                item_size,
                item_color,
            );

            // Selection box
            if is_selected {
                let box_pulse = (self.time * 6.0).sin() * 0.3 + 0.7;
                draw_rectangle_lines(
                    screen_w / 2.0 - item_width / 2.0 - 20.0,
                    y - 35.0,
                    item_width + 40.0,
                    45.0,
                    2.0,
                    Color::from_rgba(0, 255, 255, (box_pulse * 150.0) as u8),
                );
            }
        }

        // Controls hint
        let hint = "↑↓ MOVE | ENTER SELECT";
        let hint_size = 16.0;
        let hint_width = measure_text(hint, None, hint_size as u16, 1.0).width;
        let hint_alpha = ((self.time * 2.0).sin() * 0.3 + 0.7 * 180.0) as u8;

        draw_text(
            hint,
            screen_w / 2.0 - hint_width / 2.0,
            screen_h - 60.0,
            hint_size,
            Color::from_rgba(200, 200, 200, hint_alpha),
        );
    }

    fn draw_background(&self) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Dark gradient background
        clear_background(Color::from_rgba(0, 0, 20, 255));

        // Animated grid
        let grid_offset = (self.time * 20.0) % 50.0;

        // Vertical lines
        for i in 0..((screen_w / 50.0) as i32 + 1) {
            let x = i as f32 * 50.0;
            let alpha = ((self.time + i as f32 * 0.5).sin() * 30.0 + 40.0) as u8;
            draw_line(
                x,
                0.0,
                x,
                screen_h,
                1.0,
                Color::from_rgba(0, 100, 150, alpha),
            );
        }

        // Horizontal lines (scrolling)
        for i in 0..((screen_h / 50.0) as i32 + 2) {
            let y = i as f32 * 50.0 - grid_offset;
            let alpha = ((self.time + i as f32 * 0.3).cos() * 30.0 + 40.0) as u8;
            draw_line(
                0.0,
                y,
                screen_w,
                y,
                1.0,
                Color::from_rgba(0, 100, 150, alpha),
            );
        }

        // Floating glider silhouettes
        for i in 0..3 {
            let x = (screen_w * 0.2 + i as f32 * screen_w * 0.3) + (self.time * 30.0 + i as f32 * 2.0).sin() * 50.0;
            let y = screen_h * 0.3 + (self.time * 2.0 + i as f32).cos() * 30.0;
            let size = 15.0;
            let alpha = ((self.time * 3.0 + i as f32).sin() * 0.3 + 0.5 * 100.0) as u8;

            // Draw triangle for glider
            draw_triangle(
                vec2(x, y - size),
                vec2(x - size, y + size),
                vec2(x + size, y + size),
                Color::from_rgba(0, 200, 255, alpha),
            );
        }
    }
}
