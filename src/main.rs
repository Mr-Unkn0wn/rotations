mod common_colors;
mod court;
mod interface;
mod player;

use court::Court;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rotation Visualisation".to_string(),
        window_resizable: false,
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let court_size = screen_height() * 0.9;
    let offset = (screen_height() - court_size) / 2.0;
    // let offset_x = screen_width() / 2.0 - court_height / 2.0;

    let mut court = Court::new(Vec2 { x: offset, y: offset }, court_size);

    loop {
        clear_background(Color::from_rgba(8, 115, 165, 255));

        interface::draw_ui(&mut court, offset, court_size);
        court.draw_court();
        court.handle_input();
        court.draw_players();

        next_frame().await
    }
}
