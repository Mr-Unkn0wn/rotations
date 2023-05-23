use macroquad::prelude::*;

use crate::{
    common_colors,
    player::{Player, Roles},
};

const FOUR: Vec2 = Vec2::new(1.0, 2.0);
const THREE: Vec2 = Vec2::new(4.5, 2.0);
const TWO: Vec2 = Vec2::new(8.0, 2.0);
const FIVE: Vec2 = Vec2::new(1.0, 6.0);
const SIX: Vec2 = Vec2::new(4.5, 6.0);
const ONE: Vec2 = Vec2::new(8.0, 6.0);

const NUMBERS_ON_COURT: [Vec2; 6] = [ONE, TWO, THREE, FOUR, FIVE, SIX];

pub struct Court {
    pos: Vec2,
    size: f32,
    rotation: u8,
    players: [[Player; 3]; 2],
    clicked_player: Option<(i32, i32)>,
    positions_on_court: [Vec2; 6],
}

// GETTERS AND SETTERS
impl Court {
    pub fn get_pos(&self) -> Vec2 {
        self.pos
    }
    pub fn get_size(&self) -> f32 {
        self.size
    }
    pub fn get_rotation(&self) -> u8 {
        self.rotation
    }
    pub fn set_rotation(&self) {
        //todo
    }
}

impl Court {
    pub fn new(pos: Vec2, size: f32) -> Self {
        let mut positions_on_court: [Vec2; 6] = [Vec2::new(0.0, 0.0); 6];
        for (index, position) in NUMBERS_ON_COURT.iter().enumerate() {
            positions_on_court[index] = pos + (*position / 9.0) * size;
        }

        let players = [
            [
                Player {
                    role: Roles::Diagonal,
                    pos: positions_on_court[3],
                },
                Player {
                    role: Roles::Middle,
                    pos: positions_on_court[2],
                },
                Player {
                    role: Roles::Outside,
                    pos: positions_on_court[1],
                },
            ],
            [
                Player {
                    role: Roles::Outside,
                    pos: positions_on_court[4],
                },
                Player {
                    role: Roles::Middle,
                    pos: positions_on_court[5],
                },
                Player {
                    role: Roles::Setter,
                    pos: positions_on_court[0],
                },
            ],
        ];

        Court {
            pos,
            size,
            rotation: 1,
            players,
            clicked_player: None,
            positions_on_court,
        }
    }

    pub fn draw_court(&self) {
        self.draw_court_manually();
    }

    fn draw_court_manually(&self) {
        let line_color = Color::from_rgba(230, 230, 240, 255);
        let thickness = 10.0;

        let field_color = Color::from_rgba(255, 145, 92, 255);

        draw_rectangle(self.pos.x, self.pos.y, self.size, self.size, field_color);

        draw_rectangle_lines(
            self.pos.x,
            self.pos.y,
            self.size,
            self.size,
            thickness,
            common_colors::OFF_WHITE,
        );

        draw_line(
            self.pos.x,
            self.pos.y + self.size / 3.0,
            self.pos.x + self.size,
            self.pos.y + self.size / 3.0,
            thickness / 2.0,
            line_color,
        );
    }

    pub fn draw_players(&self) {
        for line in &self.players {
            for player in line {
                player.draw_player();
            }
        }
    }
}

impl Court {
    pub fn handle_input(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();

            match self.clicked_player {
                Some(p) => {
                    let mut left = None;
                    let mut right = None;
                    let mut front = None;
                    let mut behind = None;

                    if p.1 - 1 >= 0 {
                        left = Some(self.players[p.0 as usize][(p.1 - 1) as usize].pos);
                    }
                    if p.1 + 1 < 3 {
                        right = Some(self.players[p.0 as usize][(p.1 + 1) as usize].pos);
                    }
                    if p.0 - 1 >= 0 {
                        front = Some(self.players[(p.0 - 1) as usize][p.1 as usize].pos);
                    }
                    if p.0 + 1 < 2 {
                        behind = Some(self.players[(p.0 + 1) as usize][p.1 as usize].pos);
                    }

                    let surrounding = [left, right, front, behind];

                    if Player::is_pos_legal(
                        mouse_pos.0,
                        mouse_pos.1,
                        left,
                        right,
                        front,
                        behind,
                        &self,
                    ) {
                        self.players[p.0 as usize][p.1 as usize].pos.x = mouse_pos.0;
                        self.players[p.0 as usize][p.1 as usize].pos.y = mouse_pos.1;
                    }

                    for teammate in surrounding {
                        match teammate {
                            Some(teammate) => draw_line(
                                teammate.x,
                                teammate.y,
                                self.players[p.0 as usize][p.1 as usize].pos.x,
                                self.players[p.0 as usize][p.1 as usize].pos.y,
                                5.0,
                                BLACK,
                            ),
                            None => (),
                        }
                    }
                }
                None => {
                    for (y, line) in self.players.iter().enumerate() {
                        for (x, player) in line.iter().enumerate() {
                            if player.is_mouse_on_player(mouse_pos) {
                                self.clicked_player = Some((y as i32, x as i32));
                            }
                        }
                    }
                }
            }
        } else {
            self.clicked_player = None;
        }
    }
}
