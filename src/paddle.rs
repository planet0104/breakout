use super::prelude::*;
pub struct Paddle {
    handle: ColliderHandle,
    world: Rc<RefCell<World<f64>>>,
    region: VRect,
    bounds: VRect,
    image: SubImage,
}

impl Paddle {
    pub fn new(
        region: VRect,
        bounds: VRect,
        image: SubImage,
        world: Rc<RefCell<World<f64>>>,
    ) -> Paddle {
        //用一个圆的上部分作为paddle的碰撞器，以控制球的方向
        let shape = ShapeHandle::new(NBall::new(region.x_radius));

        //Paddle不会受到力的影响，所以只创建对应的碰撞器
        let collider_handle = ColliderDesc::new(shape)
            // .translation(Vector2::new(region.x, region.y))
            .translation(Vector2::new(region.x, region.y - region.y_radius * 2.0))
            .material(MaterialHandle::new(BasicMaterial::new(1.0, 0.0)))
            .density(1.0)
            .build(&mut *world.borrow_mut())
            .handle();

        Paddle {
            handle: collider_handle,
            bounds,
            world,
            region,
            image,
        }
    }

    pub fn set_position(&mut self, x: f64) {
        self.region.move_to(x, self.region.y);
        if self.region.l() < self.bounds.l() {
            self.region
                .move_to(self.bounds.l() + self.region.x_radius, self.region.y);
        }
        if self.region.r() > self.bounds.r() {
            self.region
                .move_to(self.bounds.r() - self.region.x_radius, self.region.y);
        }
        let mut world = self.world.borrow_mut();
        let collider_world = world.collider_world_mut();
        collider_world.set_position(
            self.handle,
            Isometry2::new(
                Vector2::new(self.region.x, self.region.y - self.region.y_radius * 2.0),
                0.0,
            ),
        );
    }
}

impl Paddle {
    pub fn update(&mut self) -> SpriteAction {
        SpriteAction::None
    }

    pub fn draw(&mut self, g: &mut Graphics) {
        self.image.draw(
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
