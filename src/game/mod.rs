#![allow(dead_code)]
#![allow(unused_variables)]

pub mod control;
pub mod game_over;
pub mod demo_end;
pub mod initialization;
pub mod pause;
pub mod mainmenu;
pub mod rest;
pub mod settings;
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

fn copy_text_layout(source: &[UItext], target: &mut [UItext]) {
    for (source, target) in source.iter().zip(target) {
        target.pos = source.pos;
        target.size = source.size;
        target.rotation = source.rotation;
    }
}

pub(super) fn sync_writing_system_layout(
    session: &mut GameSession,
    previous_writing_system: usize,
    next_writing_system: usize,
) {
    if previous_writing_system == next_writing_system {
        return;
    }

    if previous_writing_system < next_writing_system {
        let (previous, next) = session.text.split_at_mut(next_writing_system);
        copy_text_layout(&previous[previous_writing_system], &mut next[0]);
        let (previous, next) = session.menu_text.split_at_mut(next_writing_system);
        copy_text_layout(&previous[previous_writing_system], &mut next[0]);
    } else {
        let (next, previous) = session.text.split_at_mut(previous_writing_system);
        copy_text_layout(&previous[0], &mut next[next_writing_system]);
        let (next, previous) = session.menu_text.split_at_mut(previous_writing_system);
        copy_text_layout(&previous[0], &mut next[next_writing_system]);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WritingSystem{
    Latin = 0,
    Cyrillic = 1,
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

#[derive(PartialEq, Clone, Copy)]
pub enum SubMenu {
    Main,
    Settings,
    Game,
    Audio,
    Graphics,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameState{
    InGame,
    Paused,
    Gameover,
    MainMenu,
    DemoEnd,
}

pub struct TextContent{
    pub language_name: String,
    pub writing_system: WritingSystem,
    pub tbt_ui: Vec<String>,
    pub pause: Vec<String>,
    pub main_menu: Vec<String>,
    pub settings: Vec<String>,
    pub game_set: Vec<String>,
    pub audio: Vec<String>,
    pub graphics: Vec<String>,
    pub game_over: Vec<String>,
    pub demo_end: Vec<String>,
}

impl TextContent {
    fn parse_writing_system(node: &JsonF, language_name: &str) -> Result<WritingSystem, String> {
        if node.node_type != NodeType::String {
            return Err(format!(
                "writing_system is not a string for language {language_name}"
            ));
        }

        match node.strval.as_str() {
            "Latin" => Ok(WritingSystem::Latin),
            "Cyrillic" => Ok(WritingSystem::Cyrillic),
            writing_system => Err(format!(
                "unsupported writing system {writing_system} for language {language_name}"
            )),
        }
    }

    fn parse_string_list(node: &JsonF, field_name: &str, language_name: &str) -> Result<Vec<String>, String> {
        if node.node_type != NodeType::Other {
            return Err(format!(
                "{field_name} is not a string list for language {}",
                language_name
            ));
        }

        let mut values = Vec::with_capacity(node.other_nodes.len());
        for item in &node.other_nodes {
            if !item.indexed || item.node_type != NodeType::String {
                return Err(format!(
                    "{field_name} contains an invalid list value for language {}",
                    language_name
                ));
            }
            values.push(item.strval.clone());
        }
        Ok(values)
    }

    pub fn from_json(json: &JsonF, language_index: usize) -> Result<Self, String> {
        let language = json
            .other_nodes
            .get(language_index)
            .ok_or_else(|| format!("language index {language_index} is missing from text.json"))?;
        let writing_system = language
            .other_nodes
            .iter()
            .find(|node| node.name == "writing_system")
            .ok_or_else(|| {
                format!(
                    "writing_system is missing for language {}",
                    language.name
                )
            })
            .and_then(|node| Self::parse_writing_system(node, &language.name))?;
        let parse_list = |field_name: &str| {
            let node = language
                .other_nodes
                .iter()
                .find(|node| node.name == field_name)
                .ok_or_else(|| {
                    format!(
                        "{field_name} string list is missing for language {}",
                        language.name
                    )
                })?;
            Self::parse_string_list(node, field_name, &language.name)
        };

        Ok(Self {
            language_name: language
                .other_nodes
                .iter()
                .find(|node| node.name == "language_name")
                .filter(|node| node.node_type == NodeType::String)
                .ok_or_else(|| {
                    format!("language_name is missing for language {}", language.name)
                })?
                .strval
                .clone(),
            writing_system,
            tbt_ui: parse_list("tbt_ui")?,
            pause: parse_list("pause")?,
            main_menu: parse_list("main_menu")?,
            settings: parse_list("settings")?,
            game_set: parse_list("game_set")?,
            audio: parse_list("audio")?,
            graphics: parse_list("graphics")?,
            game_over: parse_list("game_over")?,
            demo_end: parse_list("demo_end")?,
        })
    }

    pub fn all_from_json(json: &JsonF) -> Result<Vec<Self>, String> {
        if json.other_nodes.is_empty() {
            return Err("text.json does not contain any languages".to_string());
        }

        json.other_nodes
            .iter()
            .enumerate()
            .map(|(language_index, _)| Self::from_json(json, language_index))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{TextContent, WritingSystem};
    use crate::engine::loader::jsonparser::JsonF;

    #[test]
    fn loads_languages_in_json_order() {
        let json = JsonF::from_text(
            r#"{
                "en": {
                    "language_name": "English",
                    "writing_system": "Latin",
                    "tbt_ui": ["Step"],
                    "pause": ["Continue"],
                    "main_menu": ["Start"],
                    "settings": ["Settings"],
                    "game_set": ["Language"],
                    "audio": ["Volume"],
                    "graphics": ["Lighting"],
                    "game_over": ["Retry"],
                    "demo_end": ["Demo ended", "Start"]
                },
                "ru": {
                    "language_name": "Russian",
                    "writing_system": "Cyrillic",
                    "tbt_ui": ["Step"],
                    "pause": ["Continue"],
                    "main_menu": ["Start"],
                    "settings": ["Settings"],
                    "game_set": ["Language"],
                    "audio": ["Volume"],
                    "graphics": ["Lighting"],
                    "game_over": ["Retry"],
                    "demo_end": ["Demo ended", "Start"]
                }
            }"#,
        );

        let languages = TextContent::all_from_json(&json).unwrap();
        assert_eq!(languages.len(), 2);
        assert_eq!(languages[0].language_name, "English");
        assert_eq!(languages[0].writing_system, WritingSystem::Latin);
        assert_eq!(languages[0].demo_end, ["Demo ended", "Start"]);
        assert_eq!(languages[1].language_name, "Russian");
        assert_eq!(languages[1].writing_system, WritingSystem::Cyrillic);
        assert_eq!(languages[1].demo_end, ["Demo ended", "Start"]);
    }

    #[test]
    fn loads_every_language_from_text_asset() {
        let json = JsonF::load_from_file("assets/text.json");
        let languages = TextContent::all_from_json(&json).unwrap();

        assert_eq!(languages.len(), 2);
        assert_eq!(languages[0].language_name, "English");
        assert_eq!(languages[0].demo_end, [" Demo ended! ", " New game "]);
        assert!(!languages[1].language_name.is_empty());
    }
}

pub struct GameSession {
    pub show_dbg_stat: bool,
    pub eng: Engine,
    pub scn: Scene,
    pub viewport: UIplane,
    pub ltviewport: UIplane,
    pub ltsviewport: UIplane,
    pub fpscnt: UItext,
    pub text: Vec<Vec<UItext>>,
    pub menu_text: Vec<Vec<UItext>>,
    pub keyimg: Vec<UItext>,
    pub speech_bubble_background: Vec<UIplane>,
    pub white_background: Vec<UIplane>,
    pub menu_background: Vec<UIplane>,
    pub black_background: Vec<UIplane>,
    pub awarness_signs: Vec<UIplane>,
    pub logo: UIplane,
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
    pub writing_system: WritingSystem,
    pub text_content: Vec<TextContent>,
    pub game_state: GameState,
    pub last_menu_hover: Option<(GameState, SubMenu, usize)>,
    pub last_menu_hover_frame: Option<u32>,
    pub resize_cnt: u32,
    pub tredr: f32,
    pub exit: bool,
    pub gamepadsel: usize,
    pub submenu: SubMenu,
    pub use_rt_lighting: bool,
    pub current_time: f32,
    pub tmcoef: f32,
    pub bwgcoef: f32,
    pub x_max_lv: f32,
    pub lgtext: UItext,
}

pub fn do_paper_sound(session: &mut GameSession) {
    session.speakers[1].move_sound_cursor(0.0);
    session.speakers[1].play = true;
}

pub fn do_paper2_sound(session: &mut GameSession) {
    session.speakers[7].move_sound_cursor(0.0);
    session.speakers[7].play = true;
}

pub fn do_paper_writing_sound(session: &mut GameSession) {
    session.speakers[8].move_sound_cursor(0.0);
    session.speakers[8].play = true;
}

pub fn do_paper_writing_sound_on_hover(
    session: &mut GameSession,
    hovered: bool,
    active: bool,
    item_index: usize,
) {
    if !hovered || !active || session.eng.control.mousebtn[2] {
        return;
    }

    let hover = (session.game_state, session.submenu, item_index);
    let entered_hover = session.last_menu_hover != Some(hover)
        || session.last_menu_hover_frame.map(|frame| frame.wrapping_add(1))
            != Some(session.frame_count);
    session.last_menu_hover = Some(hover);
    session.last_menu_hover_frame = Some(session.frame_count);

    if entered_hover {
        do_paper_writing_sound(session);
    }
}

pub fn do_paper_sliding_sound(session: &mut GameSession) {
    session.speakers[9].move_sound_cursor(0.0);
    session.speakers[9].play = true;
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