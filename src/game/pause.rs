use crate::engine::{math::vec3::Vec3, physics::physics::PhysicsObject};
use crate::game::smooth_value_towards;

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

    if session.returning_to_game {
        let viewport = &mut session.viewport.object.physic_object;
        viewport.scale.x = smooth_value_towards(
            viewport.scale.x,
            session.eng.render.resolution_x as f32,
            &session.eng,
            2.0,
        );
        viewport.scale.y = smooth_value_towards(
            viewport.scale.y,
            session.eng.render.resolution_y as f32,
            &session.eng,
            2.0,
        );
        viewport.pos.x = smooth_value_towards(viewport.pos.x, 0.0, &session.eng, 2.0);
        viewport.pos.y = smooth_value_towards(viewport.pos.y, 0.0, &session.eng, 2.0);

        //let mut rotation = viewport.rot.to_euler();
        //rotation.x = smooth_value_towards(rotation.x, 0.0, &session.eng, 2.0);
        //rotation.y = smooth_value_towards(rotation.y, 0.0, &session.eng, 2.0);
        //rotation.z = smooth_value_towards(rotation.z, 0.0, &session.eng, 2.0);
        //viewport.rot = rotation.to_quat();

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
        viewport.scale.x = smooth_value_towards(
            viewport.scale.x,
            session.eng.render.resolution_x as f32 * 0.8,
            &session.eng,
            2.0,
        );
        viewport.scale.y = smooth_value_towards(
            viewport.scale.y,
            session.eng.render.resolution_y as f32 * 0.8,
            &session.eng,
            2.0,
        );
        viewport.pos.x = smooth_value_towards(
            viewport.pos.x,
            session.eng.render.resolution_x as f32 * 0.5,
            &session.eng,
            2.0,
        );
        viewport.pos.y = smooth_value_towards(
            viewport.pos.y,
            session.eng.render.resolution_y as f32 * 0.1,
            &session.eng,
            2.0,
        );
        //let mut rotation = viewport.rot.to_euler();
        //rotation.z = smooth_value_towards(rotation.z, -0.05, &session.eng, 2.0);
        //viewport.rot = rotation.to_quat();
    }
    session.viewport.object.mesh.ubo[48] = session.frame_count as f32;
    session.viewport.object.mesh.ubo[49] = 0.0;
    session.viewport.object.mesh.ubo[50] = 0.2;
    session.viewport.object.mesh.ubo[51] = 0.18;
    session.viewport.object.draw = true;
    session.viewport.exec(&mut session.eng);

    // Pause-menu rendering can be added here.
}

fn viewport_is_normal(viewport: &PhysicsObject, width: f32, height: f32) -> bool {
    viewport.pos.x.abs() <= 0.1
        && viewport.pos.y.abs() <= 0.1
        && (viewport.scale.x - width).abs() <= 0.1
        && (viewport.scale.y - height).abs() <= 0.1
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
}