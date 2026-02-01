use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct InputState {
    pub move_x: f32,  // -1.0 to 1.0
    pub move_y: f32,  // -1.0 to 1.0
    pub shoot: bool,
    pub confirm: bool,
    pub back: bool,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            move_x: 0.0,
            move_y: 0.0,
            shoot: false,
            confirm: false,
            back: false,
        }
    }
}

pub struct VirtualJoystick {
    position: Vec2,
    radius: f32,
    knob_radius: f32,
    active: bool,
    touch_id: Option<u64>,
    current_offset: Vec2,
}

impl VirtualJoystick {
    pub fn new(position: Vec2, radius: f32) -> Self {
        Self {
            position,
            radius,
            knob_radius: radius * 0.4,
            active: false,
            touch_id: None,
            current_offset: Vec2::ZERO,
        }
    }

    pub fn update(&mut self, touches: &[Touch]) -> Vec2 {
        // Check if we have an active touch
        if let Some(id) = self.touch_id {
            // Find the touch with our ID
            if let Some(touch) = touches.iter().find(|t| t.id == id) {
                if touch.phase == TouchPhase::Ended || touch.phase == TouchPhase::Cancelled {
                    // Touch ended, reset
                    self.active = false;
                    self.touch_id = None;
                    self.current_offset = Vec2::ZERO;
                } else {
                    // Update joystick position
                    let touch_pos = vec2(touch.position.x, touch.position.y);
                    let offset = touch_pos - self.position;
                    let distance = offset.length();

                    if distance > self.radius {
                        self.current_offset = offset.normalize() * self.radius;
                    } else {
                        self.current_offset = offset;
                    }
                }
            } else {
                // Touch ID not found, reset
                self.active = false;
                self.touch_id = None;
                self.current_offset = Vec2::ZERO;
            }
        } else {
            // No active touch, check for new touches in joystick area
            for touch in touches {
                if touch.phase == TouchPhase::Started {
                    let touch_pos = vec2(touch.position.x, touch.position.y);
                    let distance = (touch_pos - self.position).length();

                    if distance < self.radius * 2.0 {
                        // Touch started in joystick area
                        self.active = true;
                        self.touch_id = Some(touch.id);
                        break;
                    }
                }
            }
        }

        // Return normalized direction
        if self.current_offset.length() > 0.1 {
            self.current_offset / self.radius
        } else {
            Vec2::ZERO
        }
    }

    pub fn draw(&self) {
        if !is_mobile() {
            return;
        }

        let alpha = if self.active { 180 } else { 100 };

        // Draw base circle
        draw_circle(
            self.position.x,
            self.position.y,
            self.radius,
            Color::from_rgba(100, 100, 100, alpha),
        );

        // Draw outer ring
        draw_circle_lines(
            self.position.x,
            self.position.y,
            self.radius,
            3.0,
            Color::from_rgba(200, 200, 200, alpha),
        );

        // Draw knob
        let knob_pos = self.position + self.current_offset;
        draw_circle(
            knob_pos.x,
            knob_pos.y,
            self.knob_radius,
            Color::from_rgba(150, 150, 150, alpha + 50),
        );
        draw_circle_lines(
            knob_pos.x,
            knob_pos.y,
            self.knob_radius,
            2.0,
            Color::from_rgba(255, 255, 255, alpha + 75),
        );
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

pub struct VirtualButton {
    position: Vec2,
    radius: f32,
    pressed: bool,
    touch_id: Option<u64>,
    label: String,
}

impl VirtualButton {
    pub fn new(position: Vec2, radius: f32, label: &str) -> Self {
        Self {
            position,
            radius,
            pressed: false,
            touch_id: None,
            label: label.to_string(),
        }
    }

    pub fn update(&mut self, touches: &[Touch]) -> bool {
        let was_pressed = self.pressed;
        self.pressed = false;

        // Check if we have an active touch
        if let Some(id) = self.touch_id {
            // Find the touch with our ID
            if let Some(touch) = touches.iter().find(|t| t.id == id) {
                if touch.phase == TouchPhase::Ended || touch.phase == TouchPhase::Cancelled {
                    // Touch ended
                    self.touch_id = None;
                } else {
                    self.pressed = true;
                }
            } else {
                // Touch ID not found
                self.touch_id = None;
            }
        } else {
            // No active touch, check for new touches in button area
            for touch in touches {
                if touch.phase == TouchPhase::Started {
                    let touch_pos = vec2(touch.position.x, touch.position.y);
                    let distance = (touch_pos - self.position).length();

                    if distance < self.radius {
                        // Touch started in button area
                        self.touch_id = Some(touch.id);
                        self.pressed = true;
                        break;
                    }
                }
            }
        }

        !was_pressed && self.pressed // Return true on button press (rising edge)
    }

    pub fn is_held(&self) -> bool {
        self.pressed
    }

    pub fn draw(&self) {
        if !is_mobile() {
            return;
        }

        let alpha = if self.pressed { 200 } else { 120 };

        // Draw button circle
        draw_circle(
            self.position.x,
            self.position.y,
            self.radius,
            Color::from_rgba(255, 100, 100, alpha),
        );

        // Draw button ring
        draw_circle_lines(
            self.position.x,
            self.position.y,
            self.radius,
            3.0,
            Color::from_rgba(255, 150, 150, alpha + 60),
        );

        // Draw label
        let font_size = 20.0;
        let text_size = measure_text(&self.label, None, font_size as u16, 1.0);
        draw_text(
            &self.label,
            self.position.x - text_size.width / 2.0,
            self.position.y + text_size.height / 2.0,
            font_size,
            Color::from_rgba(255, 255, 255, alpha + 80),
        );
    }
}

pub struct InputManager {
    joystick: VirtualJoystick,
    shoot_button: VirtualButton,
    mobile_mode: bool,
}

impl InputManager {
    pub fn new() -> Self {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Joystick in bottom-left
        let joystick_pos = vec2(120.0, screen_h - 120.0);
        let joystick_radius = 80.0;

        // Shoot button in bottom-right
        let button_pos = vec2(screen_w - 100.0, screen_h - 100.0);
        let button_radius = 50.0;

        Self {
            joystick: VirtualJoystick::new(joystick_pos, joystick_radius),
            shoot_button: VirtualButton::new(button_pos, button_radius, "FIRE"),
            mobile_mode: is_mobile(),
        }
    }

    pub fn update(&mut self) -> InputState {
        let mut input = InputState::default();

        // Update mobile mode detection
        self.mobile_mode = is_mobile();

        if self.mobile_mode {
            // Mobile input via touch
            let touches = touches();

            // Update virtual controls
            let joystick_dir = self.joystick.update(&touches);
            input.move_x = joystick_dir.x;
            input.move_y = -joystick_dir.y; // Invert Y for game coordinates

            input.shoot = self.shoot_button.update(&touches) || self.shoot_button.is_held();

            // Touch-based menu controls
            input.confirm = self.shoot_button.update(&touches);
            input.back = false; // Could add a back button if needed
        } else {
            // Desktop keyboard input
            // Horizontal movement
            if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                input.move_x = -1.0;
            } else if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                input.move_x = 1.0;
            }

            // Vertical movement
            if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                input.move_y = 1.0;
            } else if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                input.move_y = -1.0;
            }

            // Actions
            input.shoot = is_key_down(KeyCode::Space);
            input.confirm = is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space);
            input.back = is_key_pressed(KeyCode::Escape);
        }

        input
    }

    pub fn draw(&self) {
        if self.mobile_mode {
            self.joystick.draw();
            self.shoot_button.draw();
        }
    }

    pub fn is_mobile(&self) -> bool {
        self.mobile_mode
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        // Update positions when screen size changes
        self.joystick.position = vec2(120.0, height - 120.0);
        self.shoot_button.position = vec2(width - 100.0, height - 100.0);
    }
}

fn is_mobile() -> bool {
    // Detect mobile based on screen size and touch capability
    // In WASM, this would check the user agent or screen size
    let screen_w = screen_width();
    let screen_h = screen_height();

    // Consider it mobile if screen is narrow or touches are available
    (screen_w < 768.0 || screen_h < 768.0) && !touches().is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_state_default() {
        let input = InputState::default();
        assert_eq!(input.move_x, 0.0);
        assert_eq!(input.move_y, 0.0);
        assert_eq!(input.shoot, false);
        assert_eq!(input.confirm, false);
        assert_eq!(input.back, false);
    }

    #[test]
    fn test_virtual_joystick_creation() {
        let joystick = VirtualJoystick::new(vec2(100.0, 100.0), 50.0);
        assert!(!joystick.is_active());
    }

    #[test]
    fn test_virtual_button_creation() {
        let button = VirtualButton::new(vec2(100.0, 100.0), 50.0, "TEST");
        assert!(!button.is_held());
    }
}
