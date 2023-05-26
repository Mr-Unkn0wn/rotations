mod court;
mod interface;
mod player;
mod solutions;

use court::Court;
use macroquad::prelude::*;
use solutions::Solutions;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rotation Visualisation".to_string(),
        window_resizable: false,
        fullscreen: false,
        window_height: 650,
        window_width: 900,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let font = load_ttf_font_from_bytes(include_bytes!("../Anton-Regular.ttf")).unwrap();
    let court_size = screen_height() * 0.9;
    let offset = (screen_height() - court_size) / 2.0;
    let interface_width = screen_width() - offset * 3.0 - court_size;

    let mut court = Court::new(Vec2 { x: offset, y: offset }, court_size);
    let mut solutions = Solutions {
        show_solution: false,
        go_to_solution: false,
    };

    loop {
        clear_background(Color::from_rgba(8, 115, 165, 255));

        interface::draw_ui(&mut court, &mut solutions, interface_width, offset);
        court.draw_court();
        court.handle_input();
        court.move_players();
        solutions.draw_solution(&mut court);
        court.draw_players(&font);

        /*
        for row in court.get_players() {
            for player in row {
                let pos = court.pixel_to_on_court_meters(player.pos);
                print!("Vec2::new({:.1}, {:.1}), ", pos.x, pos.y);
            }
            println!("");
        }
        println!("");
        */

        next_frame().await
    }
}
