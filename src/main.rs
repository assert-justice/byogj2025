use macroquad::prelude::*;
use byogj2025::Game;

// const WIDTH: u32 = 640;
// const HEIGHT: u32 = 360;
// const WIDTH_F: f32 = 640.;
// const HEIGHT_F: f32 = 360.;

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        // window_width: 640,
        // window_height: 360,
        // fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    while game.is_running {
        game.update(get_frame_time());
        game.draw();
        next_frame().await
    }
}