use macroquad::prelude::*;

use crate::court::Court;

const RADIUS: f32 = 50.0;
const MOVE_SPEED: f32 = 0.1; // per tick in meters

#[derive(Clone, Copy, Debug)]
pub enum Roles {
    Outside,
    Middle,
    Diagonal,
    Setter,
}

impl Roles {
    pub fn get_color(&self) -> Color {
        match self {
            Roles::Outside => color_u8!(0, 72, 110, 255),
            Roles::Middle => color_u8!(214, 40, 40, 255),
            Roles::Diagonal => color_u8!(247, 127, 0, 255),
            Roles::Setter => color_u8!(252, 191, 73, 255),
        }
    }

    pub fn get_dark_color(&self) -> Color {
        let mut dark = Color::from_vec(self.get_color().to_vec() * 0.5);
        dark.a = 1.0;
        dark
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Player {
    pub role: Roles,
    pub pos: Vec2,
    pub target: Vec2,
}

impl Player {
    pub fn new(role: Roles, pos: Vec2) -> Player {
        Player { role, pos, target: pos }
    }

    pub fn move_player(&mut self, size: f32) {
        let speed_in_px = size * (MOVE_SPEED / 9.0);
        let mut direction = self.target - self.pos;

        if direction.length() > speed_in_px {
            direction = direction.normalize() * speed_in_px;
        }

        if direction.is_nan() {
            return;
        }

        self.pos += direction;
    }

    pub fn draw_player(&self, font: &Font) {
        draw_circle(self.pos.x, self.pos.y, RADIUS, self.role.get_dark_color());
        draw_circle(self.pos.x, self.pos.y, RADIUS - 4.0, self.role.get_color());

        let text = match self.role {
            Roles::Outside => "Out",
            Roles::Middle => "Middle",
            Roles::Diagonal => "Dia",
            Roles::Setter => "Setter",
        };

        //TEXT
        let text_paras = TextParams {
            font: *font,
            font_size: 30,
            color: WHITE,
            ..Default::default()
        };
        let dim = measure_text(text, Some(text_paras.font), text_paras.font_size, text_paras.font_scale);
        let text_x = self.pos.x - dim.width / 2.0;
        let text_y = self.pos.y + dim.height / 2.0;
        draw_text_ex(text, text_x, text_y, text_paras);
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
