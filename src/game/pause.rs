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

    session.viewport.object.physic_object.scale.x = session.eng.render.resolution_x as f32;
    session.viewport.object.physic_object.scale.y = session.eng.render.resolution_y as f32;
    session.viewport.object.mesh.ubo[48] = session.frame_count as f32;
    session.viewport.object.mesh.ubo[49] = 0.0;
    session.viewport.object.mesh.ubo[50] = 0.2;
    session.viewport.object.mesh.ubo[51] = 0.18;
    session.viewport.object.draw = true;
    session.viewport.exec(&mut session.eng);

    // Pause-menu rendering can be added here.
}

pub fn handle_input(session: &mut GameSession) {
    let start_pressed = super::control::start_pressed(session);
    if session.action_cooldown <= 0.0 && start_pressed {
        session.game_state = super::GameState::InGame;
        session.action_cooldown = 0.25;
    }
}