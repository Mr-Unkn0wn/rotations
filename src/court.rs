use macroquad::prelude::*;

use crate::player::{Player, Roles};

pub const FOUR: Vec2 = Vec2::new(1.0, 2.0);
pub const THREE: Vec2 = Vec2::new(4.5, 2.0);
pub const TWO: Vec2 = Vec2::new(8.0, 2.0);
pub const FIVE: Vec2 = Vec2::new(1.0, 6.0);
pub const SIX: Vec2 = Vec2::new(4.5, 6.0);
pub const ONE: Vec2 = Vec2::new(8.0, 6.0);

const NUMBERS_ON_COURT: [Vec2; 6] = [ONE, TWO, THREE, FOUR, FIVE, SIX];

pub struct Court {
    pos: Vec2,
    size: f32,
    rotation: i32,
    players: [[Player; 3]; 2],
    clicked_player_index: Option<(usize, usize)>,
    positions_on_court: [Vec2; 6],
    pub serve_played: bool,
}

// CONSTRUCTOR, GETTERS AND SETTERS
impl Court {
    pub fn new(pos: Vec2, size: f32) -> Self {
        let mut positions_on_court: [Vec2; 6] = [Vec2::new(0.0, 0.0); 6];
        for (index, position) in NUMBERS_ON_COURT.iter().enumerate() {
            positions_on_court[index] = pos + (*position / 9.0) * size;
        }

        let players = [
            [
                Player::new(Roles::Diagonal, positions_on_court[3]),
                Player::new(Roles::Middle, positions_on_court[2]),
                Player::new(Roles::Outside, positions_on_court[1]),
            ],
            [
                Player::new(Roles::Outside, positions_on_court[4]),
                Player::new(Roles::Middle, positions_on_court[5]),
                Player::new(Roles::Setter, positions_on_court[0]),
            ],
        ];

        Court {
            pos,
            size,
            rotation: 1,
            players,
            clicked_player_index: None,
            positions_on_court,
            serve_played: false,
        }
    }

    pub fn on_court_meters_to_pixel(&self, position: &Vec2) -> Vec2 {
        self.pos + (*position / 9.0) * self.size
    }

    pub fn _pixel_to_on_court_meters(&self, pos: Vec2) -> Vec2 {
        (pos - self.pos) / self.size * 9.0
    }

    pub fn get_players(&mut self) -> &mut [[Player; 3]; 2] {
        &mut self.players
    }

    pub fn get_pos(&self) -> Vec2 {
        self.pos
    }
    pub fn get_size(&self) -> f32 {
        self.size
    }
    pub fn get_rotation(&self) -> i32 {
        self.rotation
    }
    pub fn set_rotation(&mut self, rotation: i32) {
        self.create_player_array(rotation);
        self.rotation = rotation;
        self.serve_played = false;
    }

    fn create_player_array(&mut self, new_rotation: i32) {
        let role_order = [Roles::Setter, Roles::Outside, Roles::Middle, Roles::Diagonal, Roles::Outside, Roles::Middle];
        // 5 - 1 <- 1 - 1 = 2
        // 1 <- 5 = 4
        // (x - 1 + o) % 6 = y - 1
        // y - 1 - x - 1 % 6 = o
        let offset = ((self.rotation - 1) - (new_rotation - 1)).rem_euclid(6);

        let mut players_backup = self.players;

        for (position, _role) in role_order.iter().enumerate() {
            let mut number = new_rotation + position as i32;
            if number > 6 {
                number %= 6;
            }

            let index = Self::position_to_index(number);

            let mut old_number = number + offset;
            if old_number > 6 {
                old_number %= 6;
            }

            let old_index = Self::position_to_index(old_number);
            players_backup[old_index.1][old_index.0].target = self.positions_on_court[(number - 1) as usize];
            players_backup[old_index.1][old_index.0].target_post_serve = None;
            self.players[index.1][index.0] = players_backup[old_index.1][old_index.0];
        }
    }

    fn position_to_index(position: i32) -> (usize, usize) {
        match position {
            1 => (2, 1),
            2 => (2, 0),
            3 => (1, 0),
            4 => (0, 0),
            5 => (0, 1),
            6 => (1, 1),
            _ => panic!("Invalid position"),
        }
    }
}

// DRAW METHODS
impl Court {
    pub fn draw_court(&mut self) {
        self.draw_court_manually();
    }

    fn draw_court_manually(&self) {
        let line_color = Color::from_rgba(230, 230, 240, 255);
        let thickness = 10.0;

        let field_color = Color::from_rgba(255, 145, 92, 255);

        draw_rectangle(self.pos.x, self.pos.y, self.size, self.size, field_color);

        draw_rectangle_lines(self.pos.x, self.pos.y, self.size, self.size, thickness, color_u8!(230, 230, 240, 255));

        draw_line(
            self.pos.x,
            self.pos.y + self.size / 3.0,
            self.pos.x + self.size,
            self.pos.y + self.size / 3.0,
            thickness / 2.0,
            line_color,
        );
    }

    pub fn draw_players(&self, font: &Font) {
        for line in &self.players {
            for player in line {
                player.draw_player(font);
            }
        }
    }
}

// INPUT METHODS
impl Court {
    pub fn handle_input(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            self.left_clicked();
        } else if is_mouse_button_down(MouseButton::Right) {
            // self.right_clicked();
        } else {
            self.clicked_player_index = None;
        }
    }

    fn left_clicked(&mut self) {
        let mouse_pos = mouse_position();

        match self.clicked_player_index {
            Some(clicked_player_index) => {
                let surrounding = self.get_surrounding_players(clicked_player_index);

                if Player::is_pos_legal(mouse_pos, surrounding, self) {
                    self.players[clicked_player_index.0][clicked_player_index.1].pos.x = mouse_pos.0;
                    self.players[clicked_player_index.0][clicked_player_index.1].pos.y = mouse_pos.1;
                    self.players[clicked_player_index.0][clicked_player_index.1].target.x = mouse_pos.0;
                    self.players[clicked_player_index.0][clicked_player_index.1].target.y = mouse_pos.1;
                }

                self.draw_lines_to_surrounding(surrounding, clicked_player_index);
                self.draw_legal_area(surrounding);
            }
            None => {
                for (y, line) in self.players.iter().enumerate() {
                    for (x, player) in line.iter().enumerate() {
                        if player.is_mouse_on_player(mouse_pos) {
                            self.clicked_player_index = Some((y, x));
                        }
                    }
                }
            }
        }
    }

    fn right_clicked(&mut self) {
        let mouse_pos = mouse_position();

        match self.clicked_player_index {
            Some(clicked_player_index) => {
                if Player::is_pos_on_court(mouse_pos, &self) {
                    let player = &mut self.players[clicked_player_index.0][clicked_player_index.1];
                    player.target_post_serve = Some(Vec2::new(mouse_pos.0, mouse_pos.1));
                }
            }
            None => {
                for (y, line) in self.players.iter().enumerate() {
                    for (x, player) in line.iter().enumerate() {
                        if player.is_mouse_on_player(mouse_pos) {
                            self.clicked_player_index = Some((y, x));
                        }
                    }
                }
            }
        }
    }

    fn get_surrounding_players(&self, clicked_player_index: (usize, usize)) -> [Option<Vec2>; 4] {
        let mut left = None;
        let mut right = None;
        let mut front = None;
        let mut behind = None;

        if clicked_player_index.1 > 0 {
            left = Some(self.players[clicked_player_index.0][(clicked_player_index.1 - 1)].pos);
        }
        if clicked_player_index.1 + 1 < 3 {
            right = Some(self.players[clicked_player_index.0][(clicked_player_index.1 + 1)].pos);
        }
        if clicked_player_index.0 > 0 {
            front = Some(self.players[(clicked_player_index.0 - 1)][clicked_player_index.1].pos);
        }
        if clicked_player_index.0 + 1 < 2 {
            behind = Some(self.players[(clicked_player_index.0 + 1)][clicked_player_index.1].pos);
        }

        [left, right, front, behind]
    }

    fn draw_lines_to_surrounding(&self, surrounding: [Option<Vec2>; 4], clicked_player_index: (usize, usize)) {
        for teammate in surrounding.into_iter().flatten() {
            draw_line(
                teammate.x,
                teammate.y,
                self.players[clicked_player_index.0][clicked_player_index.1].pos.x,
                self.players[clicked_player_index.0][clicked_player_index.1].pos.y,
                5.0,
                color_u8!(0, 0, 0, 100),
            )
        }
    }

    fn draw_legal_area(&self, surrounding: [Option<Vec2>; 4]) {
        let mut top_left = self.pos;
        let mut bot_right = self.pos + self.size;

        if let Some(left) = surrounding[0] {
            top_left.x = left.x;
        }
        if let Some(right) = surrounding[1] {
            bot_right.x = right.x;
        }
        if let Some(front) = surrounding[2] {
            top_left.y = front.y;
        }
        if let Some(behind) = surrounding[3] {
            bot_right.y = behind.y;
        }

        bot_right -= top_left;
        draw_rectangle(top_left.x, top_left.y, bot_right.x, bot_right.y, color_u8!(255, 255, 255, 100));
    }
}

impl Court {
    pub fn move_players(&mut self) {
        for line in self.players.iter_mut() {
            for player in line {
                player.move_player(self.size, self.serve_played);
            }
        }
    }
}
