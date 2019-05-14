use super::prelude::*;
use retain_mut::RetainMut;

pub struct Level {
    world: Rc<RefCell<World<f64>>>,
    total_score: Rc<RefCell<i32>>,
    countdown: Animation,
    balls: Vec<Ball>,
    paddle: Paddle,
    pub bricks: Vec<Brick>,
    sound_brick_death: AssetsFile,
    sound_bounce: AssetsFile,
    sound_countdown: AssetsFile,
    tiles: Rc<Image>,
    map_levels: Vec<Vec<Vec<char>>>,
}

impl Level {
    pub fn new(
        total_score: Rc<RefCell<i32>>,
        image_loader: &mut ImageLoader,
        world: Rc<RefCell<World<f64>>>,
    ) -> Level {
        let tiles = image_loader.load("tiles.png").unwrap();

        let mut sound_countdown = AssetsFile::new("sounds/countdownBlip.wav");
        sound_countdown.load();

        let mut sound_brick_death = AssetsFile::new("sounds/brickDeath.wav");
        sound_brick_death.load();

        let mut sound_bounce = AssetsFile::new("sounds/bounce.wav");
        sound_bounce.load();

        //倒计时动画
        let mut countdown = Animation::new(
            tiles.clone(),
            vec![
                [0., 96., 32., 48.],
                [32., 96., 32., 48.],
                [64., 96., 32., 48.],
            ],
            1.0,
        );
        countdown.position = Some([
            WINDOW_WIDTH / 2.0 - (countdown.frame_width() / 2.0),
            WINDOW_HEIGHT - 150.0,
            countdown.frame_width(),
            countdown.frame_height(),
        ]);

        //创建paddle
        let paddle = Paddle::new(
            VRect::new(WINDOW_WIDTH / 2.0, 40., 24., 8.),
            VRect::new(
                WINDOW_WIDTH / 2.0,
                WINDOW_HEIGHT / 2.0,
                WINDOW_WIDTH / 2.0,
                WINDOW_HEIGHT / 2.0,
            ),
            SubImage::new(tiles.clone(), [0., 64., 48., 16.]),
            world.clone(),
        );

        let map_levels = vec![
            vec![
                // letsa begin
                vec!['X', 'X', 'g', 'o', 'g', 'X', 'X'],
                vec!['o', 'b', 'g', 'g', 'g', 'b', 'o'],
                vec!['X', 'b', 'b', 'b', 'b', 'b', 'X'],
            ],
            vec![
                // how's it going?
                vec!['X', 'g', 'o', 'g', 'o', 'g', 'X'],
                vec!['X', 'b', 'b', 'b', 'b', 'b', 'X'],
                vec!['g', 'b', 'r', 'b', 'r', 'b', 'g'],
                vec!['g', 'b', 'b', 'b', 'b', 'b', 'g'],
                vec!['g', 'b', 'X', 'X', 'X', 'b', 'g'],
                vec!['X', 'b', 'b', 'b', 'b', 'b', 'X'],
            ],
            vec![
                // tie fighta!
                vec!['X', 'b', 'X', 'g', 'X', 'b', 'X'],
                vec!['b', 'X', 'b', 'o', 'b', 'X', 'b'],
                vec!['b', 'g', 'b', 'o', 'b', 'g', 'b'],
                vec!['b', 'X', 'b', 'o', 'b', 'X', 'b'],
                vec!['X', 'b', 'X', 'X', 'X', 'b', 'X'],
                vec!['r', 'X', 'r', 'X', 'r', 'X', 'r'],
            ],
            vec![
                // swirl
                vec!['r', 'g', 'o', 'b', 'r', 'g', 'o'],
                vec!['b', 'X', 'X', 'X', 'X', 'X', 'X'],
                vec!['o', 'X', 'o', 'b', 'r', 'g', 'o'],
                vec!['g', 'X', 'g', 'X', 'X', 'X', 'b'],
                vec!['r', 'X', 'r', 'X', 'r', 'X', 'r'],
                vec!['b', 'X', 'b', 'o', 'g', 'X', 'g'],
                vec!['o', 'X', 'X', 'X', 'X', 'X', 'o'],
                vec!['g', 'r', 'b', 'o', 'g', 'r', 'b'],
            ],
        ];

        Level {
            tiles,
            countdown,
            world,
            total_score,
            paddle,
            balls: vec![],
            bricks: vec![],
            sound_brick_death,
            sound_bounce,
            sound_countdown,
            map_levels,
        }
    }

    pub fn reset(&mut self) {
        self.countdown.start();
        for ball in &mut self.balls {
            self.world.borrow_mut().remove_bodies(&[*ball.handle()]);
        }
        self.balls.clear();
        self.add_ball(50., 166.);
    }

    pub fn init_level(&mut self, level_number: usize) -> bool {
        if level_number >= self.map_levels.len() {
            return false;
        }
        //创建bricks
        let mut bricks = vec![];

        let level = &self.map_levels[level_number];
        for (i, row) in level.iter().enumerate() {
            for (c, color) in row.iter().enumerate() {
                let y = match color {
                    &'b' => 0 * 16, //'blue'
                    &'r' => 2 * 16, //'red'
                    &'o' => 1 * 16, //'orange'
                    &'g' => 3 * 16, //'green'
                    _ => -1,
                };
                if y >= 0 {
                    let y = y as f64;
                    let anim = Animation::new(
                        self.tiles.clone(),
                        vec![
                            [0., y, 32., 16.],
                            [32., y, 32., 16.],
                            [64., y, 32., 16.],
                            [96., y, 32., 16.],
                            [128., y, 32., 16.],
                        ],
                        15.0,
                    );
                    let (initx, inity) = (54., 331.);
                    let region =
                        VRect::new(initx + 32. * c as f64, inity - 16. * i as f64, 16., 8.);
                    let brick = Brick::new(region, anim, self.world.clone());
                    bricks.push(brick);
                }
            }
        }
        self.bricks = bricks;
        self.reset();
        true
    }

    pub fn add_ball(&mut self, x: f64, y: f64) {
        //创建ball
        let mut anim = Animation::active(
            self.tiles.clone(),
            vec![
                [48., 64., 16., 16.],
                [64., 64., 16., 16.],
                [80., 64., 16., 16.],
                [96., 64., 16., 16.],
                [112., 64., 16., 16.],
            ],
            15.0,
        );
        anim.set_repeat(true);

        let ball = Ball::new(x, y, anim.clone(), self.world.clone());
        self.balls.push(ball);
    }

    /// 更新
    ///
    /// # Arguments
    ///
    /// * `&mut self` Level
    ///
    /// # Return
    ///
    /// * `true` 球都漏掉了下一关
    /// * `false` 还有球
    pub fn update(&mut self) -> bool {
        if self.countdown.is_active() {
            let jump = self.countdown.update();
            if self.countdown.is_end() {
                for ball in &mut self.balls {
                    ball.set_velocity(3.16, -3.16);
                }
            } else {
                if jump {
                    if let Some(sound) = self.sound_countdown.data() {
                        mengine::play_sound(sound, AudioType::WAV);
                    }
                }
            }
        }

        for i in 0..self.balls.len() {
            let _action = self.balls[i].update();
        }

        //删除砖块
        self.bricks.retain_mut(|brick| match brick.update() {
            SpriteAction::Kill => false,
            _ => true,
        });

        let _action = self.paddle.update();

        self.check_sprite_collision()
    }

    fn check_sprite_collision(&mut self) -> bool {
        let mut world = self.world.borrow_mut();
        let mut colliders_to_remove = vec![];
        for contact in world.contact_events() {
            match contact {
                ContactEvent::Started(_, _) => {}
                ContactEvent::Stopped(handle1, handle2) => {
                    let (collider_handle1, collider_handle2) =
                        (handle1 as &ColliderHandle, handle2 as &ColliderHandle);
                    // println!("collider_handle1,collider_handle2 = {:?},{:?}", collider_handle1, collider_handle2);
                    //type ColliderHandle = CollisionObjectHandle;

                    let mut brick_death = false;

                    //检查每个砖块是否被碰到
                    for brick in &mut self.bricks {
                        if brick.dying() {
                            continue;
                        }
                        if collider_handle2 != brick.handle() && collider_handle1 != brick.handle()
                        {
                            continue;
                        }
                        //哪个球撞到了这个砖块?
                        for ball in &mut self.balls {
                            if ball.collider_handle() == collider_handle1
                                || ball.collider_handle() == collider_handle2
                            {
                                colliders_to_remove.push(*brick.handle());
                                brick.kill();
                                *self.total_score.borrow_mut() += 100;
                                if let Some(sound) = self.sound_brick_death.data() {
                                    mengine::play_sound(sound, AudioType::WAV);
                                }
                                brick_death = true;
                                break; //只要一个球撞到，不再检测另外一个球
                            }
                        }
                    }
                    if !brick_death {
                        if let Some(sound) = self.sound_bounce.data() {
                            mengine::play_sound(sound, AudioType::WAV);
                        }
                    }
                }
            };
        }
        world.remove_colliders(colliders_to_remove.as_slice());

        //删除漏掉的球
        self.balls.retain_mut(|ball| {
            let region = ball.get_region();
            if region.t() < 0.0 || region.r() < 0.0 || region.l() > 320.0 {
                world.remove_bodies(&[*ball.handle()]);
                false
            } else {
                true
            }
        });

        //没有球了要减去一命
        self.balls.len() > 0
    }

    //处理鼠标事件
    pub fn on_mouse_move(&mut self, x: f64, _y: f64) {
        let paddle = &mut self.paddle;
        paddle.set_position(x);
    }

    pub fn draw(&mut self, g: &mut Graphics) -> Result<(), String> {
        if self.countdown.is_active() {
            self.countdown.draw(g, self.countdown.position.unwrap())?;
        }
        for i in 0..self.balls.len() {
            self.balls[i].draw(g)?;
        }
        for i in 0..self.bricks.len() {
            self.bricks[i].draw(g)?;
        }
        self.paddle.draw(g)?;
        Ok(())
    }
}
