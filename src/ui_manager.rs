use crate::constants::*;
use macroquad::prelude::*;
use macroquad::ui::root_ui;

pub fn create_ui_manager() -> UIManager {
    return UIManager {
        clear_points: false,
    };
}

pub struct UIManager {
    clear_points: bool,
}

impl UIManager {
    pub fn update_ui(&mut self) {
        self.clear_points = root_ui().button(
            Vec2::new(screen_width() - BUTTON_X_OFFSET, BUTTON_Y_OFFSET),
            "Clear",
        );
    }

    pub fn clear_points_clicked(&self) -> bool {
        self.clear_points
    }
}
