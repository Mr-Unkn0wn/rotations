mod common_colors;
mod court;
mod player;

use court::Court;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rotation Visualisation".to_string(),
        window_resizable: true,
        fullscreen: false,
        window_width: 1000,
        window_height: 1000,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let mut court = Court::new(Vec2 { x: 50.0, y: 50.0 }, 900.0);

    loop {
        clear_background(Color::from_rgba(8, 115, 165, 255));

        court.draw_court();
        court.handle_input();
        court.draw_players();

        next_frame().await
    }
}
