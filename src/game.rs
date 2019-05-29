use super::prelude::*;
use std::collections::HashMap;

enum Scenes {
    Menu,
    Play,
    GameOver,
    Win,
}

struct Stage {
    img_logo: Image,
    img_background: Image,
    img_button: Image,
    img_button_color: [u8; 4],
    level: Level,
    sound_bounce: Sound,
}

pub struct Game {
    resources: HashMap<String, Assets>,
    lives: i32,
    world: Rc<RefCell<World<f64>>>,
    score: Rc<RefCell<i32>>,
    level_number: usize,
    stage: Option<Stage>,
    scenes: Scenes,
}

pub const ASSETS_LOGO: &str = "logo.png";
pub const ASSETS_BACKGROUND: &str = "bg_prerendered.png";
pub const ASSETS_BUTTON: &str = "button.png";
pub const ASSETS_TILES: &str = "tiles.png";
pub const ASSETS_SOUND_BOUNCE: &str = "sounds/bounce.wav";
pub const ASSETS_SOUND_COUNTDOWN: &str = "sounds/countdownBlip.wav";
pub const ASSETS_SOUND_BRICK_DEATH: &str = "sounds/brickDeath.wav";

impl Game {
    const RESOURCES: &'static [(&'static str, AssetsType); 7] = &[
        (ASSETS_LOGO, AssetsType::Image),
        (ASSETS_BACKGROUND, AssetsType::Image),
        (ASSETS_BUTTON, AssetsType::Image),
        (ASSETS_TILES, AssetsType::Image),
        (ASSETS_SOUND_BOUNCE, AssetsType::Sound),
        (ASSETS_SOUND_COUNTDOWN, AssetsType::Sound),
        (ASSETS_SOUND_BRICK_DEATH, AssetsType::Sound),
    ];
    pub fn new(window: &mut Window) -> Game {
        let world = Rc::new(RefCell::new(World::new()));
        world.borrow_mut().set_timestep(1.0 / UPDATE_RATE as f64);

        window.load_assets(Game::RESOURCES.to_vec());

        let score = Rc::new(RefCell::new(0));

        //创建墙体
        let add_wall =
            |world: &mut World<f64>, x: f64, y: f64, width: f64, height: f64| -> ColliderHandle {
                let top_wall_shape = ShapeHandle::new(Cuboid::new(Vector2::new(width, height)));
                ColliderDesc::new(top_wall_shape)
                    .material(MaterialHandle::new(BasicMaterial::new(1.0, 0.0)))
                    .translation(Vector2::new(x, y))
                    .build(world)
                    .handle()
            };

        add_wall(&mut world.borrow_mut(), 160., 416.0 - 8.0, 160.0, 8.0); //顶部
        add_wall(&mut world.borrow_mut(), 8.0, 176.0 + 48.0, 8.0, 176.0); //左边
        add_wall(
            &mut world.borrow_mut(),
            320.0 - 8.0,
            176.0 + 48.0,
            8.0,
            176.0,
        ); //左边

        add_wall(&mut world.borrow_mut(), 8.0, 16.0, 8.0, 16.0); //左下
        add_wall(&mut world.borrow_mut(), 320.0 - 8.0, 16.0, 8.0, 16.0); //右下

        // add_wall(&mut world.borrow_mut(), 160., 0.0, 160.0, 8.0);//底部
        // add_wall(&mut world.borrow_mut(), -8.0, 40.0, 8.0, 10.0);//左边堵口
        // add_wall(&mut world.borrow_mut(), 320.0+8.0, 40.0, 8.0, 10.0);//右边堵口

        Game {
            resources: HashMap::new(),
            scenes: Scenes::Menu,
            world,
            score,
            lives: 3,
            stage: None,
            level_number: 0,
        }
    }

    pub fn reduce_life(&mut self) {
        let stage = self.stage.as_mut().unwrap();
        self.lives -= 1;
        if self.lives >= 0 {
            stage.level.reset();
        } else {
            self.gameover();
        }
    }

    fn gameover(&mut self) {
        self.scenes = Scenes::GameOver;
        log("游戏结束！");
    }

    pub fn on_click(&mut self, x: f64, y: f64) {
        let stage = self.stage.as_mut().unwrap();
        let img_button = &stage.img_button;
        let (x, y, ix, iy) = (x, y, WINDOW_WIDTH / 2. - img_button.width() / 2., 290.0);
        if x > ix && y > iy && x < ix + img_button.width() && y < iy + img_button.height() {
            stage.img_button_color[3] = 200;
            //初始化第一关
            self.level_number = 0;
            stage.level.init_level(0);
            if let Scenes::GameOver = self.scenes {
                self.lives = 3;
                *self.score.borrow_mut() = 0;
            }
            self.scenes = Scenes::Play;
        }
    }

    //处理鼠标事件
    pub fn on_mouse_move(&mut self, x: f64, y: f64) {
        let stage = self.stage.as_mut().unwrap();
        match self.scenes {
            Scenes::Play => stage.level.on_mouse_move(x, y),
            Scenes::Menu | Scenes::GameOver => {
                //按钮效果
                let img_button = &stage.img_button;
                let (x, y, ix, iy) = (x, y, WINDOW_WIDTH / 2. - img_button.width() / 2., 290.);
                if x > ix && y > iy && x < ix + img_button.width() && y < iy + img_button.height() {
                    if stage.img_button_color[3] != 255 {
                        stage.img_button_color[3] = 255;
                        mengine::play_sound(&stage.sound_bounce);
                    }
                } else {
                    stage.img_button_color[3] = 200;
                }
            }
            _ => {}
        };
    }

    pub fn next_level(&mut self) {
        let stage = self.stage.as_mut().unwrap();
        if !stage.level.init_level(self.level_number + 1) {
            //通关了
            self.scenes = Scenes::Win;
            return;
        }
        self.level_number += 1;
    }
}

impl State for Game {
    fn new(window: &mut Window) -> Self {
        Game::new(window)
    }

    fn on_assets_load(
        &mut self,
        path: &str,
        _: AssetsType,
        assets: std::io::Result<Assets>,
        _window: &mut Window,
    ) {
        match assets {
            Ok(assets) => {
                self.resources.insert(path.to_string(), assets);

                if self.resources.len() == Game::RESOURCES.len() {
                    // log(format!("resources:{:?}", self.resources));
                    let level = Level::new(self.score.clone(), &self.resources, self.world.clone());
                    self.stage = Some(Stage {
                        level,
                        img_logo: self.resources.get(ASSETS_LOGO).unwrap().as_image().unwrap(),
                        img_background: self
                            .resources
                            .get(ASSETS_BACKGROUND)
                            .unwrap()
                            .as_image()
                            .unwrap(),
                        img_button: self
                            .resources
                            .get(ASSETS_BUTTON)
                            .unwrap()
                            .as_image()
                            .unwrap(),
                        img_button_color: [255, 255, 255, 200],
                        sound_bounce: self
                            .resources
                            .get(ASSETS_SOUND_BOUNCE)
                            .unwrap()
                            .as_sound()
                            .unwrap(),
                    });
                }
            }
            Err(err) => alert(
                "温馨提示",
                &format!("资源文件加载失败:{:?} {:?}", path, err).as_str(),
            ),
        }
    }

    fn update(&mut self, _window: &mut Window) {
        if self.stage.is_none() {
            return;
        }
        match self.scenes {
            Scenes::Play => {
                if !self.stage.as_mut().unwrap().level.update() {
                    self.reduce_life();
                }
                if self.stage.as_mut().unwrap().level.bricks.len() == 0 {
                    self.next_level();
                }
                self.world.borrow_mut().step();
            }
            _ => {}
        }
    }

    fn event(&mut self, event: Event, _window: &mut Window) {
        // println!("event:{:?}", event);
        if self.stage.is_none() {
            return;
        }
        match event {
            Event::MouseMove(x, y) => self.on_mouse_move(x as f64, y as f64),
            Event::Click(x, y) => self.on_click(x as f64, y as f64),
            _ => (),
        };
    }

    fn draw(&mut self, g: &mut Graphics, _window: &mut Window) {
        g.fill_rect(&[216, 216, 216, 255], 0., 0., WINDOW_WIDTH, WINDOW_HEIGHT);

        if self.stage.is_none() {
            let progress_bar_height = 30.0;
            let progress_bar_width = WINDOW_WIDTH * 0.8;
            let progress_bar_x = (WINDOW_WIDTH - progress_bar_width) / 2.0;
            let progress_bar_y = WINDOW_HEIGHT / 2.0;
            //进度条背景
            g.fill_rect(
                &[127, 127, 127, 255],
                progress_bar_x,
                progress_bar_y,
                progress_bar_width,
                progress_bar_height,
            );
            //进度条前景
            let progress = self.resources.len() as f64 / Game::RESOURCES.len() as f64;
            g.fill_rect(
                &[10, 10, 128, 255],
                progress_bar_x,
                progress_bar_y,
                progress * progress_bar_width,
                progress_bar_height,
            );
            g.draw_text(
                &format!("Loading {}%", (progress * 100.0) as i32),
                progress_bar_x + 20.0,
                progress_bar_y + 2.0,
                &[255, 255, 255, 255],
                16,
            );
            return;
        }
        let stage = self.stage.as_mut().unwrap();

        g.draw_image(None, &stage.img_background, None, None);

        if let Scenes::Play = self.scenes {
            stage.level.draw(g);
            g.draw_text(
                &format!(
                    "Lives:{} Score:{} Level:{}",
                    self.lives,
                    self.score.borrow(),
                    self.level_number + 1
                ),
                30.,
                WINDOW_HEIGHT - 20.,
                &[0, 0, 0, 255],
                14,
            );
            return;
        } else {
            //画logo
            let logo = &stage.img_logo;
            g.draw_image(
                None,
                logo,
                None,
                Some([
                    WINDOW_WIDTH / 2.0 - logo.width() as f64 / 2.0,
                    WINDOW_HEIGHT / 2.0 - logo.height() as f64 / 1.2,
                    logo.width() as f64,
                    logo.height() as f64,
                ]),
            );
        }

        if let Scenes::Win = self.scenes {
            g.draw_text("恭喜！！", 125., 290., &[0, 0, 0, 255], 13);
            g.draw_text("你赢了！", 125., 320., &[0, 0, 0, 255], 13);
        } else {
            let img_button = &stage.img_button;
            g.draw_image(
                None,
                &img_button,
                None,
                Some([
                    WINDOW_WIDTH / 2.0 - img_button.width() as f64 / 2.0,
                    290.,
                    img_button.width() as f64,
                    img_button.height() as f64,
                ]),
            );
            if let Scenes::GameOver = self.scenes {
                g.draw_text("你输了！", 125., 270., &[0, 0, 0, 255], 13);
                g.draw_text("重新开始", 132., 301., &stage.img_button_color, 12);
            } else {
                g.draw_text("开始游戏", 132., 301., &stage.img_button_color, 12);
            }
        }
    }
}
