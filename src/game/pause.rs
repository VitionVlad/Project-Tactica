use crate::engine::{math::vec3::Vec3, physics::physics::PhysicsObject};
use crate::game::SubMenu::{Audio, Game, Graphics, Main, Settings};
use crate::game::{smooth_value_towards, InputType};

use super::GameSession;

pub fn update(session: &mut GameSession) {
    session.speakers[3].volume = 0.0;
    for speaker in &mut session.speakers {
        speaker.exec(&mut session.eng);
    }

    let fps_text = format!(
        "CPU_dt:{}|GPU_fps:{}|detection_level: {:?}",
        session.eng.cpu_fps, session.eng.gpu_fps, session.awareness
    );
    session.fpscnt.size.x = 10.0 * session.ui_scale;
    session.fpscnt.size.y = 20.0 * session.ui_scale;
    session.fpscnt.pos.x = session.eng.render.resolution_x as f32 - 400.0;
    session.fpscnt.pos.y = 0.0;
    session.fpscnt.pos.z = 0.1;
    session.fpscnt.draw = true;
    session.fpscnt.exec(&mut session.eng, &fps_text);

    for text in &mut session.text {
        text.draw = false;
        text.exec(&mut session.eng, " ");
    }
    for keyimg in &mut session.keyimg {
        keyimg.draw = false;
        keyimg.exec(&mut session.eng, " ");
    }
    for background in &mut session.speech_bubble_background {
        background.object.draw = false;
        background.exec(&mut session.eng);
    }
    for sign in &mut session.awarness_signs {
        sign.object.draw = false;
        sign.exec(&mut session.eng);
    }

    let screen_height = session.eng.render.resolution_y as f32;
    let viewport_target_y = match session.submenu {
        Main => 0.0,
        Settings | Game | Audio | Graphics => screen_height * 0.8,
    };

    if session.returning_to_game {
        let viewport = &mut session.viewport.object.physic_object;
        viewport.pos.x = smooth_value_towards(viewport.pos.x, 0.0, &session.eng, 2.0);
        viewport.pos.y = smooth_value_towards(viewport.pos.y, 0.0, &session.eng, 2.0);

        if viewport_is_normal(
            viewport,
            session.eng.render.resolution_x as f32,
            session.eng.render.resolution_y as f32,
        ) {
            viewport.scale.x = session.eng.render.resolution_x as f32;
            viewport.scale.y = session.eng.render.resolution_y as f32;
            viewport.pos.x = 0.0;
            viewport.pos.y = 0.0;
            viewport.rot = Vec3::new().to_quat();
            session.returning_to_game = false;
            session.game_state = super::GameState::InGame;
        }
    } else {
        let viewport = &mut session.viewport.object.physic_object;
        viewport.scale.x = session.eng.render.resolution_x as f32;
        viewport.scale.y = screen_height;
        viewport.pos.x = smooth_value_towards(
            viewport.pos.x,
            session.eng.render.resolution_x as f32 * 0.5,
            &session.eng,
            2.0,
        );
        viewport.pos.y = smooth_value_towards(viewport.pos.y, viewport_target_y, &session.eng, 2.0);
    }

    let viewport = &session.viewport.object.physic_object;
    let viewport_left = viewport.pos.x - viewport.scale.x * 0.5;
    let panel_gap = 30.0;
    for (index, background) in session.menu_background.iter_mut().enumerate() {
        background.object.physic_object.scale.x = viewport.scale.x;
        background.object.physic_object.scale.y = viewport.scale.y;
        background.object.physic_object.pos.x = viewport_left;
        background.object.physic_object.pos.y = if index == 0 {
            viewport.pos.y
        } else {
            viewport.pos.y - viewport.scale.y - panel_gap
        };
        background.object.physic_object.pos.z = 0.99;
        background.object.draw = index < 2;
        background.exec(&mut session.eng);
    }

    let ui_scale = session.ui_scale;
    let margin = 30.0 * ui_scale;
    let menu_items = menu_items(session);
    let menu_item_count = menu_items.len();
    let dense_menu = menu_item_count > 4;
    let row_gap = if dense_menu { 15.0 } else { 30.0 } * ui_scale;
    let button_height = if dense_menu { 40.0 } else { 60.0 } * ui_scale;
    let logo_size = if dense_menu { 120.0 } else { 200.0 } * ui_scale;
    let logo_gap = if dense_menu { 20.0 } else { 30.0 } * ui_scale;
    let panel_top = match session.submenu {
        Main => viewport.pos.y,
        Settings | Game | Audio | Graphics => viewport.pos.y - viewport.scale.y - panel_gap,
    };
    let menu_height =
        button_height * menu_item_count as f32 + row_gap * menu_item_count.saturating_sub(1) as f32;
    let content_height = logo_size + logo_gap + menu_height;
    let content_top = panel_top + (viewport.scale.y - content_height) * 0.5;
    let menu_left = viewport_left + margin;
    let menu_top = content_top + logo_size + logo_gap;

    session.black_background[0].object.physic_object.scale.x =
        session.viewport.object.physic_object.scale.x + 50.0;
    session.black_background[0].object.physic_object.scale.y =
        session.viewport.object.physic_object.scale.y + 50.0;
    session.black_background[0].object.physic_object.pos.x = viewport.pos.x - 25.0;
    session.black_background[0].object.physic_object.pos.y = viewport.pos.y - 25.0;
    session.black_background[0].object.physic_object.pos.z = 0.66;
    session.black_background[0].object.draw = true;
    session.black_background[0].object.physic_object.rot = viewport.rot;
    session.black_background[0].exec(&mut session.eng);

    for i in 0..menu_item_count {
        session.menu_text[i].size.x = 30.0 * ui_scale;
        session.menu_text[i].size.y = button_height;
        session.menu_text[i].pos.x = menu_left;
        session.menu_text[i].pos.y = menu_top + (button_height + row_gap) * i as f32;
        session.menu_text[i].pos.z = 0.7;
        session.menu_text[i].draw = true;
        session.menu_text[i].signal = true;
        session.menu_text[i].per_symbol = false;
        session.menu_text[i].rotation = Vec3 {
            x: 0.0,
            y: 0.0,
            z: -0.05,
        }
        .to_quat();
        if session.menu_text[i].exec(&mut session.eng, &menu_items[i])
            && session.eng.control.mousebtn[2]
            && session.action_cooldown <= 0.0
        {
            activate_menu_item(session, i);
            session.input_type = InputType::Touchscreen;
            session.action_cooldown = 0.25;
        }

        let background = &mut session.black_background[i + 1];
        let padding = 7.5 * ui_scale;
        background.object.physic_object.scale.x =
            session.menu_text[i].size.x * (menu_items[i].len() as f32 + 1.0) + padding * 2.0;
        background.object.physic_object.scale.y = session.menu_text[i].size.y + padding * 2.0;
        background.object.physic_object.pos.x = session.menu_text[i].pos.x - padding;
        background.object.physic_object.pos.y = session.menu_text[i].pos.y - padding;
        background.object.physic_object.pos.z = 0.71;
        background.object.draw = true;
        background.object.physic_object.rot = Vec3 {
            x: 0.0,
            y: 0.0,
            z: -0.05,
        }
        .to_quat();
        background.exec(&mut session.eng);
    }

    session.logo.object.draw = true;
    session.logo.object.physic_object.scale.x = logo_size;
    session.logo.object.physic_object.scale.y = logo_size;
    session.logo.object.physic_object.pos.x = session.menu_text[0].pos.x;
    session.logo.object.physic_object.pos.y = content_top;
    session.logo.object.physic_object.pos.z = 0.7;
    session.logo.object.physic_object.rot = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -0.05,
    }
    .to_quat();
    session.logo.exec(&mut session.eng);

    session.viewport.object.physic_object.pos.z = 0.6;
    session.viewport.object.mesh.ubo[48] = session.frame_count as f32;
    session.viewport.object.mesh.ubo[49] = 0.0;
    session.viewport.object.mesh.ubo[50] = 0.2;
    session.viewport.object.mesh.ubo[51] = 0.18;
    session.viewport.object.draw = true;
    session.viewport.exec(&mut session.eng);

    for i in menu_item_count..session.menu_text.len() {
        session.menu_text[i].draw = false;
        session.menu_text[i].exec(&mut session.eng, " ");
        session.black_background[i + 1].object.draw = false;
        session.black_background[i + 1].exec(&mut session.eng);
    }
}

fn menu_items(session: &GameSession) -> Vec<String> {
    match session.submenu {
        Main => session.text_content.pause.clone(),
        Settings => session.text_content.settings.clone(),
        Game => session.text_content.game_set.clone(),
        Audio => session.text_content.audio.clone(),
        Graphics => session.text_content.graphics.clone(),
    }
}

fn activate_menu_item(session: &mut GameSession, index: usize) {
    match session.submenu {
        Main => match index {
            0 => session.returning_to_game = true,
            1 => session.submenu = Settings,
            2 => session.exit = true,
            _ => {}
        },
        Settings => match index {
            0 => session.submenu = Game,
            1 => session.submenu = Audio,
            2 => session.submenu = Graphics,
            _ => session.submenu = Main,
        },
        Game | Audio | Graphics => {
            if index + 1 == menu_items(session).len() {
                session.submenu = Settings;
            }
        }
    }
    session.gamepadsel = 0;
}

fn viewport_is_normal(viewport: &PhysicsObject, width: f32, height: f32) -> bool {
    viewport.pos.x.abs() <= 2.0
        && viewport.pos.y.abs() <= 2.0
        && (viewport.scale.x - width).abs() <= 2.0
        && (viewport.scale.y - height).abs() <= 2.0
}

pub fn handle_input(session: &mut GameSession) {
    if session.returning_to_game {
        return;
    }

    let start_pressed = super::control::start_pressed(session);
    if session.action_cooldown <= 0.0 && start_pressed {
        session.returning_to_game = true;
        session.action_cooldown = 0.25;
    }

    match session.submenu {
        Main | Settings | Game | Audio | Graphics => {
            for mt in &mut session.menu_text {
                mt.implicit_on = false;
            }
            if session.eng.control.gamepad_connected {
                let menu_len = menu_items(session).len();
                if session
                    .eng
                    .control
                    .get_gamepad_button_state(session.controller_mappings.arrows[1] as u8)
                    && session.gamepadsel > 0
                    && session.action_cooldown <= 0.0
                {
                    session.action_cooldown = 0.25;
                    session.gamepadsel -= 1;
                    session.input_type = InputType::Gamepad;
                } else if session
                    .eng
                    .control
                    .get_gamepad_button_state(session.controller_mappings.arrows[0] as u8)
                    && session.gamepadsel + 1 < menu_len
                    && session.action_cooldown <= 0.0
                {
                    session.action_cooldown = 0.25;
                    session.gamepadsel += 1;
                    session.input_type = InputType::Gamepad;
                } else if session
                    .eng
                    .control
                    .get_gamepad_button_state(session.controller_mappings.abxy[0] as u8)
                    && session.action_cooldown <= 0.0
                {
                    activate_menu_item(session, session.gamepadsel);
                    session.action_cooldown = 0.25;
                    session.input_type = InputType::Gamepad;
                }
                if session.input_type == InputType::Gamepad && menu_len > 0 {
                    session.gamepadsel = session.gamepadsel.min(menu_len - 1);
                    session.menu_text[session.gamepadsel].implicit_on = true;
                }
            }
        }
    }
}
