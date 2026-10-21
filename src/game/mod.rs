#![allow(dead_code)]
#![allow(unused_variables)]

pub mod control;
pub mod game_over;
pub mod initialization;
pub mod pause;
pub mod rest;
pub mod tbt;

use crate::engine::{
    engine::Engine, loader::jsonparser::{JsonF, NodeType}, math::vec3::Vec3, scene::Scene, speaker::Speaker, ui::{UIplane, UItext},
};

pub fn smooth_value_towards(
    value: f32,
    final_value: f32,
    engine: &Engine,
    speed_modifier: f32,
) -> f32 {
    let difference = final_value - value;
    if difference.abs() <= 0.01 {
        return final_value;
    }

    let step_factor = (engine.logic_frametime * speed_modifier).clamp(0.0, 1.0);
    value + difference * step_factor
}

#[derive(Clone, Copy)]
pub struct Pivot {
    pub id: usize,
    pub tbtmode: bool,
    pub start_pos: Vec3,
}

const DIRECTIONS: [Vec3; 4] = [
    Vec3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    },
    Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    },
    Vec3 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    },
    Vec3 {
        x: -1.0,
        y: 0.0,
        z: 0.0,
    },
];

#[derive(PartialEq, Clone, Eq, Hash, Ord, PartialOrd, Debug)]
pub enum Awarness {
    Unseen(u8),
    Suspected(i8),
    Detected(i8),
    Compromised(i8),
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

#[derive(Clone, Copy)]
pub enum Action{
    Step((Direction, usize)),
    StepWithElemination((Direction, usize)),
    Skip,
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

pub struct Key{
    pub keycode: u8,
    pub key: String,
}

pub struct KeyboardBindings{
    pub arrows: [Key; 4],
    pub abxy: [Key; 4],
    pub start: Key,
    pub select: Key,
}

pub struct ControllerMappings{
    pub arrows: [usize; 4],
    pub arrows_key: [String; 4],
    pub abxy: [usize; 4],
    pub abxy_key: [String; 4],
    pub axis: [usize; 4],
    pub bumpers: [usize; 2],
    pub start: usize,
    pub start_key: String,
    pub select: usize,
    pub select_key: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputType {
    Keyboard,
    Gamepad,
    Touchscreen,
}

pub enum GameState{
    InGame,
    Paused,
    Gameover,
    MainMenu,
}

pub struct TextContent{
    pub actions: Vec<String>,
}

impl TextContent {
    pub fn from_json(json: &JsonF, language_index: usize) -> Result<Self, String> {
        let language = json
            .other_nodes
            .get(language_index)
            .ok_or_else(|| format!("language index {language_index} is missing from text.json"))?;
        let tbt_ui = language
            .other_nodes
            .iter()
            .find(|node| node.name == "tbt_ui")
            .ok_or_else(|| format!("tbt_ui dictionary is missing for language {}", language.name))?;
        let mut actions = Vec::with_capacity(tbt_ui.other_nodes.len());
        for item in &tbt_ui.other_nodes {
            if item.node_type != NodeType::String {
                return Err(format!(
                    "tbt_ui contains a non-string value for language {}",
                    language.name
                ));
            }
            actions.push(item.strval.clone());
        }
        Ok(Self { actions })
    }
}

pub struct GameSession {
    pub eng: Engine,
    pub scn: Scene,
    pub viewport: UIplane,
    pub ltviewport: UIplane,
    pub fpscnt: UItext,
    pub text: Vec<UItext>,
    pub keyimg: Vec<UItext>,
    pub speech_bubble_background: Vec<UIplane>,
    pub white_background: Vec<UIplane>,
    pub awarness_signs: Vec<UIplane>,
    pub ui_scale: f32,
    pub pivot: Pivot,
    pub tbtplates: Vec<TBTplate>,
    pub enemies: Vec<Enemy>,
    pub physics_objects: Vec<(usize, Vec3)>,
    pub actual_plate_id: usize,
    pub awareness: Awarness,
    pub max_awareness: i8,
    pub making_step: bool,
    pub object_holding: Option<(Option<usize>, Vec3)>,
    pub current_dir: Vec3,
    pub action_cooldown: f32,
    pub returning_to_game: bool,
    pub frame_count: u32,
    pub speakers: Vec<Speaker>,
    pub keyboard_bindings: KeyboardBindings,
    pub controller_mappings: ControllerMappings,
    pub input_type: InputType,
    pub action_list: [Option<Action>; 5],
    pub active_action: Option<Action>,
    pub chosen_language: usize,
    pub text_content: TextContent,
    pub game_state: GameState,
    pub resize_cnt: u32,
    pub tredr: f32,
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