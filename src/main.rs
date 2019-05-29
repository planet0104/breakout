// #![windows_subsystem = "windows"]

mod ball;
mod brick;
mod game;
mod level;
mod paddle;
mod prelude;

use game::Game;
use mengine::*;
use prelude::*;

fn main() {
    mengine::run::<Game>(
        "打砖块",
        320.0,
        416.0,
        Settings {
            icon_path: Some("favicon.ico"),
            ups: UPDATE_RATE,
            show_ups_fps: true,
            background_color: Some([216, 216, 216, 255]),
            auto_scale: true,
            draw_center: true,
            window_size: Some((100.0, 200.0)),
            ..Default::default()
        },
    );
}
