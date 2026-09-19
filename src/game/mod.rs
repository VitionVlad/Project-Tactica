pub mod control;
pub mod game_over;
pub mod initialization;
pub mod rest;
pub mod tbt;

use crate::engine::{
    engine::Engine,
    math::vec3::Vec3,
    scene::Scene,
    ui::{UIplane, UItext},
};

#[derive(Clone, Copy)]
pub struct Pivot {
    pub id: usize,
    pub tbtmode: bool,
    pub start_pos: Vec3,
}

#[derive(PartialEq, Clone, Eq, Hash, Ord, PartialOrd, Debug)]
pub enum Awarness {
    Unseen(u8),
    Suspected(i8),
    Detected(i8),
    Compromised(i8),
}

pub struct Enemy {
    pub id: usize,
    pub direction: Vec3,
    pub plate_id: Option<usize>,
    pub alive: bool,
    pub start_pos: Vec3,
    pub initial_tile_id: Option<usize>,
}

pub struct TBTplate {
    pub id: usize,
    pub safe: bool,
    pub enemy_id: Option<usize>,
}

pub struct GameSession {
    pub eng: Engine,
    pub scn: Scene,
    pub viewport: UIplane,
    pub ltviewport: UIplane,
    pub fpscnt: UItext,
    pub pivot: Pivot,
    pub tbtplates: Vec<TBTplate>,
    pub enemies: Vec<Enemy>,
    pub physics_objects: Vec<(usize, Vec3)>,
    pub actual_plate_id: usize,
    pub awareness: Awarness,
    pub max_awareness: i8,
    pub making_step: bool,
    pub object_holding: Option<Vec3>,
    pub current_dir: Vec3,
    pub action_cooldown: f32,
    pub frame_count: u32,
}

pub fn tile_test(
    pivot: &crate::engine::physics::physics::PhysicsObject,
    plate: &crate::engine::physics::physics::PhysicsObject,
) -> bool {
    let pivot_collider = &pivot.collider;
    let plate_collider = &plate.collider;
    let overlaps_x = pivot_collider.aabb_min.x <= plate_collider.aabb_max.x
        && plate_collider.aabb_min.x <= pivot_collider.aabb_max.x;
    let overlaps_z = pivot_collider.aabb_min.z <= plate_collider.aabb_max.z
        && plate_collider.aabb_min.z <= pivot_collider.aabb_max.z;
    overlaps_x && overlaps_z
}

pub fn tile_test_point(
    point: &Vec3,
    plate: &crate::engine::physics::physics::PhysicsObject,
) -> bool {
    let plate_collider = &plate.collider;
    let overlaps_x = point.x <= plate_collider.aabb_max.x && plate_collider.aabb_min.x <= point.x;
    let overlaps_z = point.z <= plate_collider.aabb_max.z && plate_collider.aabb_min.z <= point.z;
    overlaps_x && overlaps_z
}
