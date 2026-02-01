use macroquad::prelude::*;

pub struct SplashScreen {
    time: f32,
    completed: bool,
}

impl SplashScreen {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            completed: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;

        // Auto-complete after 2.5 seconds
        if self.time >= 2.5 {
            self.completed = true;
        }
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn draw(&self) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Animated background with scanlines
        let scanline_intensity = ((self.time * 10.0).sin() * 0.1 + 0.9) * 255.0;
        clear_background(Color::from_rgba(0, 0, 20, 255));

        // Draw retro scanlines
        for i in (0..(screen_h as i32)).step_by(4) {
            draw_line(
                0.0,
                i as f32,
                screen_w,
                i as f32,
                1.0,
                Color::from_rgba(0, 20, 40, (scanline_intensity * 0.3) as u8),
            );
        }

        // Main title with glow effect
        let title = "GLIDE WARS";
        let title_size = 80.0;

        // Calculate fade-in based on time
        let alpha = (self.time * 2.0).min(1.0);

        // Pulsing glow effect
        let pulse = (self.time * 3.0).sin() * 0.3 + 0.7;

        // Draw multiple layers for glow
        for offset in &[6.0, 4.0, 2.0] {
            let glow_alpha = (alpha * pulse * 0.5 * (6.0 - offset) / 6.0 * 255.0) as u8;
            let text_width = measure_text(title, None, title_size as u16, 1.0).width;

            draw_text(
                title,
                screen_w / 2.0 - text_width / 2.0,
                screen_h / 2.0 - *offset,
                title_size,
                Color::from_rgba(0, 255, 255, glow_alpha),
            );
            draw_text(
                title,
                screen_w / 2.0 - text_width / 2.0,
                screen_h / 2.0 + *offset,
                title_size,
                Color::from_rgba(0, 255, 255, glow_alpha),
            );
        }

        // Main title
        let text_width = measure_text(title, None, title_size as u16, 1.0).width;
        draw_text(
            title,
            screen_w / 2.0 - text_width / 2.0,
            screen_h / 2.0,
            title_size,
            Color::from_rgba(0, 255, 255, (alpha * 255.0) as u8),
        );

        // Subtitle with fade-in delay
        if self.time > 0.5 {
            let subtitle = "A RETRO FLIGHT SURVIVAL GAME";
            let subtitle_alpha = ((self.time - 0.5) * 2.0).min(1.0);
            let subtitle_size = 20.0;
            let subtitle_width = measure_text(subtitle, None, subtitle_size as u16, 1.0).width;

            draw_text(
                subtitle,
                screen_w / 2.0 - subtitle_width / 2.0,
                screen_h / 2.0 + 60.0,
                subtitle_size,
                Color::from_rgba(255, 255, 255, (subtitle_alpha * 200.0) as u8),
            );
        }

        // Loading animation
        if self.time > 1.0 {
            let loading_alpha = ((self.time - 1.0) * 2.0).min(1.0);

            // Rotating glider icon
            let icon_x = screen_w / 2.0;
            let icon_y = screen_h / 2.0 + 120.0;
            let rotation = self.time * 2.0;

            // Draw rotating triangle (glider silhouette)
            let size = 20.0;
            for i in 0..3 {
                let angle = rotation + i as f32 * std::f32::consts::PI * 2.0 / 3.0;
                let x1 = icon_x + angle.cos() * size;
                let y1 = icon_y + angle.sin() * size;
                let next_angle = rotation + ((i + 1) % 3) as f32 * std::f32::consts::PI * 2.0 / 3.0;
                let x2 = icon_x + next_angle.cos() * size;
                let y2 = icon_y + next_angle.sin() * size;

                draw_line(
                    x1, y1, x2, y2,
                    2.0,
                    Color::from_rgba(0, 255, 255, (loading_alpha * 255.0) as u8),
                );
            }

            // Loading text
            let loading_text = "LOADING";
            let loading_size = 18.0;
            let loading_width = measure_text(loading_text, None, loading_size as u16, 1.0).width;

            draw_text(
                loading_text,
                screen_w / 2.0 - loading_width / 2.0,
                screen_h / 2.0 + 160.0,
                loading_size,
                Color::from_rgba(255, 255, 255, (loading_alpha * 200.0) as u8),
            );
        }

        // Copyright/credit
        if self.time > 1.5 {
            let credit_alpha = ((self.time - 1.5).min(1.0) * 150.0) as u8;
            let credit = "BUILT WITH RUST & MACROQUAD";
            let credit_size = 14.0;
            let credit_width = measure_text(credit, None, credit_size as u16, 1.0).width;

            draw_text(
                credit,
                screen_w / 2.0 - credit_width / 2.0,
                screen_h - 40.0,
                credit_size,
                Color::from_rgba(150, 150, 150, credit_alpha),
            );
        }
    }

    pub fn skip(&mut self) {
        self.completed = true;
    }
}
