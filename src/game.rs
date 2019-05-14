use super::prelude::*;

enum Scenes {
    Menu,
    Play,
    GameOver,
    Win,
}

pub struct Game {
    lives: i32,
    world: Rc<RefCell<World<f64>>>,
    img_logo: Rc<Image>,
    img_background: Rc<Image>,
    img_button: (Rc<Image>, [u8; 4]),
    score: Rc<RefCell<i32>>,
    level: Level,
    level_number: usize,
    scenes: Scenes,
    sound_bounce: AssetsFile,
}

impl Game {
    pub fn new(image_loader: &mut ImageLoader) -> Game {
        let world = Rc::new(RefCell::new(World::new()));
        world.borrow_mut().set_timestep(1.0 / UPDATE_RATE as f64);

        let img_logo = image_loader.load("logo.png").unwrap();
        let img_background = image_loader.load("bg_prerendered.png").unwrap();
        let img_button = (
            image_loader.load("button.png").unwrap(),
            [255, 255, 255, 200],
        );
        let score = Rc::new(RefCell::new(0));
        let level = Level::new(score.clone(), image_loader, world.clone());

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

        let mut sound_bounce = AssetsFile::new("sounds/bounce.wav");
        sound_bounce.load();

        Game {
            img_button,
            scenes: Scenes::Menu,
            world,
            score,
            lives: 3,
            img_logo,
            img_background,
            level,
            level_number: 0,
            sound_bounce,
        }
    }

    pub fn update(&mut self) {
        match self.scenes {
            Scenes::Play => {
                if !self.level.update() {
                    self.reduce_life();
                }
                if self.level.bricks.len() == 0 {
                    self.next_level();
                }
                self.world.borrow_mut().step();
            }
            _ => {}
        }
    }

    pub fn reduce_life(&mut self) {
        self.lives -= 1;
        if self.lives >= 0 {
            self.level.reset();
        } else {
            self.gameover();
        }
    }

    fn gameover(&mut self) {
        self.scenes = Scenes::GameOver;
        log("游戏结束！");
    }

    pub fn on_click(&mut self, x: f64, y: f64) {
        let img_button = &self.img_button.0;
        let (x, y, ix, iy) = (
            x as u32,
            y as u32,
            WINDOW_WIDTH as u32 / 2 - img_button.width() / 2,
            290,
        );
        if x > ix && y > iy && x < ix + img_button.width() && y < iy + img_button.height() {
            self.img_button.1[3] = 200;
            //初始化第一关
            self.level_number = 0;
            self.level.init_level(0);
            if let Scenes::GameOver = self.scenes {
                self.lives = 3;
                *self.score.borrow_mut() = 0;
            }
            self.scenes = Scenes::Play;
        }
    }

    //处理鼠标事件
    pub fn on_mouse_move(&mut self, x: f64, y: f64) {
        match self.scenes {
            Scenes::Play => self.level.on_mouse_move(x, y),
            Scenes::Menu | Scenes::GameOver => {
                //按钮效果
                let img_button = &self.img_button.0;
                let (x, y, ix, iy) = (
                    x as u32,
                    y as u32,
                    WINDOW_WIDTH as u32 / 2 - img_button.width() / 2,
                    290,
                );
                if x > ix && y > iy && x < ix + img_button.width() && y < iy + img_button.height() {
                    if self.img_button.1[3] != 255 {
                        self.img_button.1[3] = 255;
                        if let Some(sound) = self.sound_bounce.data() {
                            mengine::play_sound(sound, AudioType::WAV);
                        }
                    }
                } else {
                    self.img_button.1[3] = 200;
                }
            }
            _ => {}
        };
    }

    pub fn next_level(&mut self) {
        if !self.level.init_level(self.level_number + 1) {
            //通关了
            self.scenes = Scenes::Win;
            return;
        }
        self.level_number += 1;
    }

    pub fn draw(&mut self, g: &mut Graphics) -> Result<(), String> {
        g.clear_rect(&[255, 255, 255, 255], 0., 0., WINDOW_WIDTH, WINDOW_HEIGHT);
        g.draw_image(self.img_background.as_ref(), None, None)?;

        if let Scenes::Play = self.scenes {
            self.level.draw(g)?;
            g.draw_text(
                &format!(
                    "Lives:{} Score:{} Level:{}",
                    self.lives,
                    self.score.borrow(),
                    self.level_number + 1
                ),
                30.,
                WINDOW_HEIGHT - 10.,
                &[0, 0, 0, 255],
                14,
            )?;
            return Ok(());
        } else {
            //画logo
            let logo = self.img_logo.as_ref();
            g.draw_image(
                logo,
                None,
                Some([
                    WINDOW_WIDTH / 2.0 - logo.width() as f64 / 2.0,
                    WINDOW_HEIGHT / 2.0 - logo.height() as f64 / 1.2,
                    logo.width() as f64,
                    logo.height() as f64,
                ]),
            )?;
        }

        if let Scenes::Win = self.scenes {
            g.draw_text("恭喜！！", 125., 290., &[0, 0, 0, 255], 13)?;
            g.draw_text("你赢了！", 125., 320., &[0, 0, 0, 255], 13)?;
        } else {
            g.draw_image(
                self.img_button.0.as_ref(),
                None,
                Some([
                    WINDOW_WIDTH / 2.0 - self.img_button.0.width() as f64 / 2.0,
                    290.,
                    self.img_button.0.width() as f64,
                    self.img_button.0.height() as f64,
                ]),
            )?;
            if let Scenes::GameOver = self.scenes {
                g.draw_text("你输了！", 125., 270., &[0, 0, 0, 255], 13)?;
                g.draw_text("重新开始", 132., 315., &self.img_button.1, 12)?;
            } else {
                g.draw_text("开始游戏", 132., 315., &self.img_button.1, 12)?;
            }
        }
        Ok(())
    }
}
