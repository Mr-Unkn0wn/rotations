use crate::court::{self, Court};
use macroquad::prelude::*;

const CORRECT_POSITION_PER_ROTATION: [[[Vec2; 3]; 2]; 6] = [
    [[Vec2::new(0.5, 0.5), Vec2::new(1.0, 3.0), court::ONE], [court::FIVE, court::SIX, Vec2::new(8.0, 7.0)]],
    [
        [Vec2::new(0.9, 2.3), Vec2::new(1.4, 5.8), Vec2::new(5.9, 1.0)],
        [Vec2::new(3.7, 7.5), Vec2::new(4.5, 6.0), Vec2::new(8.0, 6.0)],
    ],
    [
        [Vec2::new(1.3, 6.0), Vec2::new(6.8, 0.9), Vec2::new(8.1, 2.6)],
        [Vec2::new(4.7, 6.2), Vec2::new(6.1, 8.0), Vec2::new(8.0, 6.0)],
    ],
    [
        [Vec2::new(0.8, 1.2), Vec2::new(1.5, 2.6), Vec2::new(8.0, 5.9)],
        [Vec2::new(1.0, 6.0), Vec2::new(4.5, 6.0), Vec2::new(8.0, 7.8)],
    ],
    [
        [Vec2::new(0.5, 2.3), Vec2::new(1.4, 5.8), Vec2::new(8.6, 2.7)],
        [Vec2::new(3.8, 3.1), Vec2::new(4.5, 6.0), Vec2::new(8.0, 6.0)],
    ],
    [
        [Vec2::new(1.8, 6.0), Vec2::new(7.5, 1.1), Vec2::new(7.8, 2.9)],
        [Vec2::new(4.9, 6.2), Vec2::new(5.8, 2.2), Vec2::new(8.0, 6.0)],
    ],
];

pub struct Solutions {
    pub show_solution: bool,
}

impl Solutions {
    pub fn draw_solution(&self, court: &Court) {
        if !self.show_solution {
            return;
        }

        let correct_pos = CORRECT_POSITION_PER_ROTATION[(court.get_rotation() - 1) as usize];

        for y in 0..2 {
            for x in 0..3 {
                let role_color = court.get_players()[y][x].role.get_color();
                let dark_color = court.get_players()[y][x].role.get_dark_color();
                let player_target = court.get_players()[y][x].pos;
                let correct_target = court.on_court_meters_to_pixel(&correct_pos[y][x]);

                draw_line(player_target.x, player_target.y, correct_target.x, correct_target.y, 9.0, dark_color);
                draw_line(player_target.x, player_target.y, correct_target.x, correct_target.y, 5.0, role_color);
                draw_circle(correct_target.x, correct_target.y, 10.0, dark_color);
                draw_circle(correct_target.x, correct_target.y, 8.0, role_color);
            }
        }
    }
}
