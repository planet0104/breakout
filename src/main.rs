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

thread_local! {
    pub static UPDATE_COUNT: RefCell<u64> = RefCell::new(0);
    pub static DRAW_COUNT: RefCell<u64> = RefCell::new(0);
    pub static NEXT_TIME: RefCell<u64> = RefCell::new(0);
    pub static UPDATE_FPS: RefCell<u64> = RefCell::new(0);
    pub static DRAW_FPS: RefCell<u64> = RefCell::new(0);
}

impl State for Game {
    fn new(image_loader: &mut ImageLoader) -> Self {
        NEXT_TIME.with(|time| {
            *time.borrow_mut() = current_timestamp() as u64 + 1000;
        });
        Game::new(image_loader)
    }

    fn update(&mut self) {
        UPDATE_COUNT.with(|count| {
            *count.borrow_mut() += 1;
        });
        self.update();
    }

    fn event(&mut self, event: Event) {
        // println!("event:{:?}", event);
        match event {
            Event::MouseMove(x, y) => self.on_mouse_move(x as f64, y as f64),
            Event::Click(x, y) => self.on_click(x as f64, y as f64),
            _ => (),
        };
    }

    fn draw(&mut self, g: &mut Graphics) -> Result<(), String> {
        DRAW_COUNT.with(|count| {
            *count.borrow_mut() += 1;
        });
        NEXT_TIME.with(|time| {
            if current_timestamp() as u64 >= *time.borrow() {
                //显示帧率
                UPDATE_FPS.with(|count| {
                    *count.borrow_mut() = UPDATE_COUNT.with(|count| *count.borrow_mut());
                });
                DRAW_FPS.with(|count| {
                    *count.borrow_mut() = DRAW_COUNT.with(|count| *count.borrow_mut());
                });

                *time.borrow_mut() = current_timestamp() as u64 + 1000;
                UPDATE_COUNT.with(|count| {
                    *count.borrow_mut() = 0;
                });
                DRAW_COUNT.with(|count| {
                    *count.borrow_mut() = 0;
                });
            }
        });
        self.draw(g)?;

        g.draw_text(
            &format!(
                "FPS:{}/{}",
                UPDATE_FPS.with(|count| { *count.borrow() }),
                DRAW_FPS.with(|count| { *count.borrow() })
            ),
            20.,
            30.,
            &[0, 0, 0, 200],
            10,
        )
    }
}

fn main() {
    mengine::run::<Game>(
        "打砖块",
        320.0,
        416.0,
        Settings {
            icon_path: Some("favicon.ico"),
            font_file: Some("wqy-micro-hei.ttf"),
            ups: UPDATE_RATE,
            background_color: Some([216, 216, 216, 255]),
            auto_scale: true,
            ..Default::default()
        },
    );
}
