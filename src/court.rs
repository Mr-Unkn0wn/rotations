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
    img: Texture2D,
    pub pos: Vec2,
    pub size: Vec2,
    pub rotation: u8,
    players: [[Player; 3]; 2],
    clicked_player: Option<(i32, i32)>,
}

impl Court {
    pub fn new(pos: Vec2, size: f32) -> Self {
        let bytes = include_bytes!("court.png");

        let img = Image::from_file_with_format(bytes, Some(ImageFormat::Png));

        let img = img.sub_image(Rect {
            x: 0.0,
            y: (img.height / 2) as f32,
            w: img.width as f32,
            h: (img.height / 2) as f32,
        });

        let img = Texture2D::from_image(&img);

        let players = [
            [
                Player {
                    role: Roles::Diagonal,
                    pos: FOUR,
                },
                Player {
                    role: Roles::Middle,
                    pos: THREE,
                },
                Player {
                    role: Roles::Outside,
                    pos: TWO,
                },
            ],
            [
                Player {
                    role: Roles::Outside,
                    pos: FIVE,
                },
                Player {
                    role: Roles::Middle,
                    pos: SIX,
                },
                Player {
                    role: Roles::Setter,
                    pos: ONE,
                },
            ],
        ];

        Court {
            img,
            pos,
            size: Vec2 { x: size, y: size },
            rotation: 1,
            players,
            clicked_player: None,
        }
    }

    pub fn draw_court(&self) {
        // draw_texture(self.img, 0.0, 0.0, WHITE);
        self.draw_court_manually();
    }

    fn draw_court_manually(&self) {
        let line_color = Color::from_rgba(230, 230, 240, 255);
        let thickness = 10.0;

        let field_color = Color::from_rgba(255, 145, 92, 255);

        draw_rectangle(
            self.pos.x,
            self.pos.y,
            self.size.x,
            self.size.y,
            field_color,
        );

        draw_rectangle_lines(
            self.pos.x,
            self.pos.y,
            self.size.x,
            self.size.y,
            thickness,
            common_colors::OFF_WHITE,
        );

        draw_line(
            self.pos.x,
            self.pos.y + self.size.y / 3.0,
            self.pos.x + self.size.x,
            self.pos.y + self.size.y / 3.0,
            thickness / 2.0,
            line_color,
        );
    }

    pub fn draw_players(&self) {
        for line in &self.players {
            for player in line {
                player.draw_player(&self.pos, &self.size);
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
                    let x = Player::pixel_to_court_x(&self.pos, &self.size, mouse_pos.0);
                    let y = Player::pixel_to_court_y(&self.pos, &self.size, mouse_pos.1);

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
                    let player = &mut self.players[p.0 as usize][p.1 as usize];

                    for p in surrounding {
                        match p {
                            Some(l) => draw_line(
                                Player::court_to_pixel_x(l.x, &self.pos, &self.size),
                                Player::court_to_pixel_y(l.y, &self.pos, &self.size),
                                Player::court_to_pixel_x(player.pos.x, &self.pos, &self.size),
                                Player::court_to_pixel_y(player.pos.y, &self.pos, &self.size),
                                5.0,
                                BLACK,
                            ),
                            None => (),
                        }
                    }

                    if Player::is_pos_legal(x, y, left, right, front, behind) {
                        player.pos.x = x;
                        player.pos.y = y;
                    }
                }
                None => {
                    for (y, line) in self.players.iter().enumerate() {
                        for (x, player) in line.iter().enumerate() {
                            if player.is_mouse_on_player(mouse_pos, &self.pos, &self.size) {
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
