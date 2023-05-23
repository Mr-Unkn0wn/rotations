use macroquad::prelude::*;

use crate::common_colors::*;

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
    pub fn draw_player(&self, pos: &Vec2, size: &f32) {
        let x = Player::court_to_pixel_x(self.pos.x, pos, size);
        let y = Player::court_to_pixel_y(self.pos.y, pos, size);
        draw_circle(x, y, RADIUS, BLACK);
        draw_circle(x, y, RADIUS - 4.0, OFF_WHITE);

        let text = match self.role {
            Roles::Outside => "Außen",
            Roles::Middle => "Mitte",
            Roles::Diagonal => "Dia",
            Roles::Setter => "Setter",
        };

        let dim = measure_text(text, None, 20, 1.0);
        let text_x = x - dim.width / 2.0;
        let text_y = y - dim.height / 2.0;
        draw_text(text, text_x, text_y, 20.0, BLACK);
    }

    pub fn is_mouse_on_player(&self, mouse_pos: (f32, f32), pos: &Vec2, size: &f32) -> bool {
        let mouse_pos_vec = Vec2::new(mouse_pos.0, mouse_pos.1);
        let player_pos = Vec2::new(
            Player::court_to_pixel_x(self.pos.x, pos, size),
            Player::court_to_pixel_y(self.pos.y, pos, size),
        );

        if mouse_pos_vec.distance(player_pos) <= RADIUS {
            true
        } else {
            false
        }
    }

    pub fn court_to_pixel_x(x: f32, pos: &Vec2, size: &f32) -> f32 {
        pos.x + size * (x / 9.0)
    }

    pub fn court_to_pixel_y(y: f32, pos: &Vec2, size: &f32) -> f32 {
        pos.y + size * (y / 9.0)
    }

    pub fn pixel_to_court_x(pos: &Vec2, size: &f32, x: f32) -> f32 {
        (x - pos.x) / size * 9.0
    }

    pub fn pixel_to_court_y(pos: &Vec2, size: &f32, y: f32) -> f32 {
        (y - pos.y) / size * 9.0
    }

    pub fn is_pos_legal(
        x: f32,
        y: f32,
        left: Option<Vec2>,
        right: Option<Vec2>,
        front: Option<Vec2>,
        behind: Option<Vec2>,
    ) -> bool {
        let min_dist = 0.9;

        if x < 0.0 || x > 9.0 {
            return false;
        }
        if y < 0.0 || y > 9.0 {
            return false;
        }
        match left {
            Some(l) => {
                if l.x >= x - min_dist {
                    return false;
                }
            }
            None => (),
        }
        match right {
            Some(r) => {
                if r.x <= x + min_dist {
                    return false;
                }
            }
            None => (),
        }
        match front {
            Some(f) => {
                if f.y >= y - min_dist {
                    return false;
                }
            }
            None => (),
        }
        match behind {
            Some(f) => {
                if f.y <= y + min_dist {
                    return false;
                }
            }
            None => (),
        }

        true
    }
}
