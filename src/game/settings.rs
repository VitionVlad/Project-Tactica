use crate::engine::loader::{
    jsonparser::{JsonF, NodeType},
    rw::{checkfs, mkdirfs, readfs, writefs},
};

use super::GameSession;

const SETTINGS_PATH: &str = "saves/settings.json";

fn ensure_settings_dir() {
    mkdirfs("saves");
}

pub fn save_settings(session: &GameSession) {
    ensure_settings_dir();

    let json = format!(
        "{{\n  \"ui_scale\": {},\n  \"show_dbg_stat\": {},\n  \"audio_volume\": {},\n  \"use_rt_lighting\": {},\n  \"resolution_scale\": {},\n  \"fullscreen\": {},\n  \"shadowmaps_resolution\": {},\n  \"resolution_x\": {},\n  \"resolution_y\": {}\n}}\n",
        session.ui_scale,
        session.show_dbg_stat,
        session.eng.audio.vol,
        session.use_rt_lighting,
        session.eng.render.resolution_scale,
        session.eng.render.fullscreen,
        session.eng.render.shadow_map_resolution,
        session.eng.render.resolution_x,
        session.eng.render.resolution_y,
    );

    writefs(SETTINGS_PATH, json.into_bytes());
}

pub fn load_settings(session: &mut GameSession) {
    if !checkfs(SETTINGS_PATH) {
        return;
    }

    let text = String::from_utf8(readfs(SETTINGS_PATH)).unwrap_or_default();
    let parsed = JsonF::from_text(&text);

    let mut nr = (session.eng.render.resolution_x, session.eng.render.resolution_y);

    for option in &parsed.other_nodes {
        match option.name.as_str() {
            "ui_scale" if option.node_type == NodeType::Number => {
                session.ui_scale = option.numeral_val as f32;
            }
            "show_dbg_stat" if option.node_type == NodeType::Bolean => {
                session.show_dbg_stat = option.bolean;
            }
            "audio_volume" if option.node_type == NodeType::Number => {
                session.eng.audio.vol = option.numeral_val as f32;
            }
            "use_rt_lighting" if option.node_type == NodeType::Bolean => {
                session.use_rt_lighting = option.bolean;
            }
            "resolution_scale" if option.node_type == NodeType::Number => {
                session.eng.render.resolution_scale = option.numeral_val as f32;
            }
            "fullscreen" if option.node_type == NodeType::Bolean => {
                session.eng.render.fullscreen = option.bolean;
            }
            "shadowmaps_resolution" if option.node_type == NodeType::Number => {
                session.eng.render.shadow_map_resolution = option.numeral_val as u32;
            }
            "resolution_x" if option.node_type == NodeType::Number => {
                nr.0 = option.numeral_val as u32;
            }
            "resolution_y" if option.node_type == NodeType::Number => {
                nr.1 = option.numeral_val as u32;
            }
            _ => {}
        }
    }
    session.eng.render.set_new_resolution(nr.0, nr.1);
}
