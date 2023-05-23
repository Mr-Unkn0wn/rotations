use macroquad::prelude::*;

use crate::{common_colors::*, court::Court};

const RADIUS: f32 = 50.0;

pub enum Roles {
    Outside,
    Middle,
    Diagonal,
    Setter,
}

pub struct Player {
    pub role: Roles,
    pub pos: Vec2,
}

impl Player {
    pub fn draw_player(&self) {
        draw_circle(self.pos.x, self.pos.y, RADIUS, BLACK);
        draw_circle(self.pos.x, self.pos.y, RADIUS - 4.0, OFF_WHITE);

        let text = match self.role {
            Roles::Outside => "Außen",
            Roles::Middle => "Mitte",
            Roles::Diagonal => "Dia",
            Roles::Setter => "Setter",
        };

        let dim = measure_text(text, None, 20, 1.0);
        let text_x = self.pos.x - dim.width / 2.0;
        let text_y = self.pos.y - dim.height / 2.0;
        draw_text(text, text_x, text_y, 20.0, BLACK);
    }

    pub fn is_mouse_on_player(&self, mouse_pos: (f32, f32)) -> bool {
        let mouse_pos_vec = Vec2::new(mouse_pos.0, mouse_pos.1);

        mouse_pos_vec.distance(self.pos) <= RADIUS
    }

    pub fn is_pos_legal(mouse_pos: (f32, f32), surrounding: [Option<Vec2>; 4], court: &Court) -> bool {
        let x = mouse_pos.0;
        let y = mouse_pos.1;

        if x < court.get_pos().x || x > court.get_pos().x + court.get_size() {
            return false;
        }
        if y < court.get_pos().y || y > court.get_pos().y + court.get_size() {
            return false;
        }
        if let Some(left) = surrounding[0] {
            if left.x >= x {
                return false;
            }
        }
        if let Some(right) = surrounding[1] {
            if right.x <= x {
                return false;
            }
        }
        if let Some(front) = surrounding[2] {
            if front.y >= y {
                return false;
            }
        }
        if let Some(behind) = surrounding[3] {
            if behind.y <= y {
                return false;
            }
        }

        true
    }
}
