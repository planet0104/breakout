pub static WINDOW_WIDTH: f64 = 320.0;
pub static WINDOW_HEIGHT: f64 = 416.0;
pub static UPDATE_RATE: u64 = 60;
pub use super::ball::Ball;
pub use super::brick::Brick;
pub use super::game::*;
pub use super::level::Level;
pub use super::paddle::Paddle;
pub use mengine::*;
pub use nalgebra::{Isometry2, Point2, Vector2, Vector3};
pub use ncollide2d::events::ContactEvent;
pub use ncollide2d::shape::{Ball as NBall, Capsule, ConvexPolygon, Cuboid, Shape, ShapeHandle};
pub use ncollide2d::world::CollisionGroups;
pub use nphysics2d::joint::{ConstraintHandle, MouseConstraint};
pub use nphysics2d::material::{BasicMaterial, MaterialHandle};
pub use nphysics2d::math::{Inertia, Velocity};
pub use nphysics2d::object::{
    Body, BodyHandle, BodyPart, BodyPartHandle, BodyStatus, ColliderAnchor, ColliderDesc,
    ColliderHandle, RigidBodyDesc,
};
pub use nphysics2d::world::{ColliderWorld, World};
pub use std::cell::RefCell;
pub use std::collections::HashMap;
pub use std::f64::consts::PI;
pub use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum SpriteAction {
    None,
    Kill,
    _AddSprite,
}

#[derive(Clone, Debug)]
pub struct VRect {
    pub x: f64,
    pub y: f64,
    pub x_radius: f64,
    pub y_radius: f64,
}
impl VRect {
    pub fn new(x: f64, y: f64, x_radius: f64, y_radius: f64) -> VRect {
        VRect {
            x,
            y,
            x_radius,
            y_radius,
        }
    }
    pub fn move_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
    pub fn l(&self) -> f64 {
        self.x - self.x_radius
    }
    pub fn t(&self) -> f64 {
        self.y + self.y_radius
    }
    pub fn r(&self) -> f64 {
        self.x + self.x_radius
    }
    pub fn _b(&self) -> f64 {
        self.y - self.y_radius
    }
    pub fn w(&self) -> f64 {
        self.x_radius * 2.0
    }
    pub fn h(&self) -> f64 {
        self.y_radius * 2.0
    }
}

pub fn create_velocity(x: f64, y: f64, timestep: f64) -> Velocity<f64> {
    Velocity::between_positions(
        &Isometry2::new(Vector2::new(0.0, 0.0), std::f64::consts::PI),
        &Isometry2::new(Vector2::new(x, y), std::f64::consts::PI),
        timestep,
    )
}
