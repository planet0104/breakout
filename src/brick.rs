use super::prelude::*;

pub struct Brick {
    handle: ColliderHandle,
    region: VRect,
    anim: Animation,
}

impl Brick {
    pub fn new(region: VRect, anim: Animation, world: Rc<RefCell<World<f64>>>) -> Brick {
        let cuboid = ShapeHandle::new(Cuboid::new(Vector2::new(region.x_radius, region.y_radius)));

        let collider_handle = ColliderDesc::new(cuboid)
            .translation(Vector2::new(region.x, region.y))
            .material(MaterialHandle::new(BasicMaterial::new(1.0, 0.0)))
            .collision_groups(CollisionGroups::new().with_membership(&[0]))
            .density(1.0)
            .build(&mut *world.borrow_mut())
            .handle();

        // let rigid_cuboid_handle = RigidBodyDesc::new()
        //     .status(BodyStatus::Static)
        //     .collider(&collider_cuboid)
        //     .build(&mut *world.borrow_mut()).handle();

        Brick {
            handle: collider_handle,
            region: region,
            anim: anim,
        }
    }

    pub fn dying(&self) -> bool {
        self.anim.is_active()
    }

    pub fn handle(&self) -> &ColliderHandle {
        &self.handle
    }
}

impl Brick {
    pub fn update(&mut self) -> SpriteAction {
        self.anim.update();
        if self.anim.is_end() {
            SpriteAction::Kill
        } else {
            SpriteAction::None
        }
    }

    pub fn draw(&mut self, g: &mut Graphics) -> Result<(), String> {
        self.anim.draw(
            g,
            [
                self.region.l(),
                WINDOW_HEIGHT - self.region.t(),
                self.region.w(),
                self.region.h(),
            ],
        )
    }

    pub fn kill(&mut self) {
        self.anim.start();
    }
}
