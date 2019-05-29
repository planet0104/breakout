use super::prelude::*;

pub struct Ball {
    handle: BodyHandle,
    collider_handle: ColliderHandle,
    world: Rc<RefCell<World<f64>>>,
    region: VRect,
    anim: Animation,
}

impl Ball {
    pub fn new(x: f64, y: f64, anim: Animation, world: Rc<RefCell<World<f64>>>) -> Ball {
        let rect = VRect::new(x, y, 8.0, 8.0);

        //创建碰撞器
        let ball = ShapeHandle::new(NBall::new(rect.x_radius));
        let collider_ball = ColliderDesc::new(ball)
            .material(MaterialHandle::new(BasicMaterial::new(1.0, 0.0)))
            .collision_groups(
                CollisionGroups::new()
                    .with_whitelist(&[0])
                    .with_membership(&[1]),
            ) //球和球之间不碰撞
            .translation(Vector2::new(x, y))
            .density(1.0);

        let ball_handle = RigidBodyDesc::new()
            .velocity(Velocity::linear(0.0, 0.0)) //默认速度0
            .kinematic_rotation(true) //防止旋转减速?
            .mass(1.0)
            .collider(&collider_ball)
            .build(&mut *world.borrow_mut())
            .handle();

        let collider_handle = (&mut *world.borrow_mut())
            .collider_world()
            .body_colliders(ball_handle)
            .next()
            .unwrap()
            .handle();

        Ball {
            collider_handle,
            handle: ball_handle,
            world,
            region: rect,
            anim: anim,
        }
    }

    pub fn get_region(&self) -> &VRect {
        &self.region
    }

    //设置刚体速度
    pub fn set_velocity(&mut self, vx: f64, vy: f64) {
        let mut world = self.world.borrow_mut();
        let timestep = world.timestep();
        let rb = world
            .rigid_body_mut(self.handle)
            .expect("Rigid-body not found.");
        rb.set_velocity(create_velocity(vx, vy, timestep));
    }

    pub fn handle(&self) -> &BodyHandle {
        &self.handle
    }

    pub fn collider_handle(&self) -> &ColliderHandle {
        &self.collider_handle
    }
}

impl Ball {
    pub fn update(&mut self) -> SpriteAction {
        self.anim.update();
        //更新位置
        let world = self.world.borrow();
        let body = world.body(self.handle).unwrap();
        let ball_pos = body.part(0).unwrap().center_of_mass();
        self.region.move_to(ball_pos.x, ball_pos.y);
        SpriteAction::None
    }

    pub fn draw(&mut self, g: &mut Graphics) {
        self.anim.draw(
            None,
            g,
            [
                self.region.l(),
                WINDOW_HEIGHT - self.region.t(),
                self.region.w(),
                self.region.h(),
            ],
        );
    }
}
