use crate::{engine::{math::vec3::Vec3, physics::physics::PhysicsObject}, game::{InputType, do_paper_sound, smooth_value_towards}};

use super::{Awarness, GameSession};

pub fn handle(session: &mut GameSession) {
    if session.awareness != Awarness::Compromised(session.max_awareness) {
        return;
    }
    session.game_state = super::GameState::Gameover;
    //session.awareness = Awarness::Unseen(0);
    //session.pivot.tbtmode = false;
    //session.making_step = false;
    //session.scn.objects[session.pivot.id].physic_object.pos = session.pivot.start_pos;
    //for enemy in &mut session.enemies {
    //    enemy.alive = true;
    //    enemy.plate_id = enemy.initial_tile_id;
    //    session.scn.objects[enemy.id].physic_object.pos = enemy.start_pos;
    //    session.scn.objects[enemy.id].draw = true;
    //}
    //for object_id in &session.physics_objects {
    //    let object = &mut session.scn.objects[object_id.0].physic_object;
    //    object.pos = object_id.1;
    //}
}

pub fn handle_gom(session: &mut GameSession){
    session.speakers[3].volume = 0.0;
    for speaker in &mut session.speakers {
        speaker.exec(&mut session.eng);
    }

    if session.show_dbg_stat{
        let fps_text = format!(
            "CPU_cps:{}|GPU_fps:{}",
            session.eng.cpu_fps, session.eng.gpu_fps
        );
        session.fpscnt.size.x = 10.0 * session.ui_scale;
        session.fpscnt.size.y = 20.0 * session.ui_scale;
        session.fpscnt.pos.x = session.eng.render.resolution_x as f32 - 200.0;
        session.fpscnt.pos.y = 0.0;
        session.fpscnt.pos.z = 0.1;
        session.fpscnt.draw = true;
        session.fpscnt.exec(&mut session.eng, &fps_text);
    }else{
        session.fpscnt.draw = false;
        session.fpscnt.exec(&mut session.eng, " ");
    }

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
        sign.object.physic_object.pos.x = -1000.0;
        sign.exec(&mut session.eng);
    }

    session.menu_background[0].object.physic_object.scale.x = session.eng.render.resolution_x as f32;
    session.menu_background[0].object.physic_object.scale.y = session.eng.render.resolution_y as f32;
    session.menu_background[0].object.physic_object.pos.x = session.viewport.object.physic_object.pos.x + session.viewport.object.physic_object.scale.x + 30.0;
    session.menu_background[0].object.physic_object.pos.y = session.viewport.object.physic_object.pos.y;
    session.menu_background[0].object.physic_object.pos.z = 0.99;
    session.menu_background[0].object.draw = true;
    session.menu_background[0].object.mesh.ubo[52] = session.current_time as f32;
    session.menu_background[0].object.mesh.ubo[53] = 2000.0;
    session.menu_background[0].object.mesh.ubo[54] = 2000.0;
    session.menu_background[0].object.mesh.ubo[55] = session.frame_count as f32;
    session.menu_background[0].exec(&mut session.eng);

    session.menu_text[0].size.x = 40.0 * session.ui_scale;
    session.menu_text[0].size.y = 80.0 * session.ui_scale;
    session.menu_text[0].pos.x = session.menu_background[0].object.physic_object.pos.x + 30.0;
    session.menu_text[0].pos.y = session.eng.render.resolution_y as f32 * 0.2;
    session.menu_text[0].pos.z = 0.7;
    session.menu_text[0].draw = true;
    session.menu_text[0].signal = false;
    session.menu_text[0].per_symbol = false;
    session.menu_text[0].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
    session.menu_text[0].exec(&mut session.eng, &session.text_content.game_over[0]);

    session.black_background[0].object.physic_object.scale.x = session.menu_text[0].size.x*(session.text_content.game_over[0].len() as f32)+15.0;
    session.black_background[0].object.physic_object.scale.y = session.menu_text[0].size.y+15.0;
    session.black_background[0].object.physic_object.pos.x = session.menu_text[0].pos.x-7.5;
    session.black_background[0].object.physic_object.pos.y = session.menu_text[0].pos.y-7.5;
    session.black_background[0].object.physic_object.pos.z = 0.71;
    session.black_background[0].object.draw = true;
    session.black_background[0].object.physic_object.rot = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
    session.black_background[0].exec(&mut session.eng);

    session.menu_text[1].size.x = 20.0 * session.ui_scale;
    session.menu_text[1].size.y = 40.0 * session.ui_scale;
    session.menu_text[1].pos.x = session.menu_background[0].object.physic_object.pos.x + 30.0;
    session.menu_text[1].pos.y = session.eng.render.resolution_y as f32 * 0.4 + session.menu_text[1].size.y + 30.0;
    session.menu_text[1].pos.z = 0.7;
    session.menu_text[1].draw = true;
    session.menu_text[1].signal = true;
    session.menu_text[1].per_symbol = false;
    session.menu_text[1].implicit_on = false;
    session.menu_text[1].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
    if session.menu_text[1].exec(&mut session.eng, &session.text_content.game_over[1]) && session.eng.control.mousebtn[2] && session.action_cooldown <= 0.0{
        session.input_type = InputType::Touchscreen;
        session.action_cooldown = 0.25;
        //session.game_state = super::GameState::InGame;
        do_paper_sound(session);
        session.returning_to_game = true;
    }

    session.black_background[1].object.physic_object.scale.x = session.menu_text[1].size.x*(session.text_content.game_over[1].len() as f32)+15.0;
    session.black_background[1].object.physic_object.scale.y = session.menu_text[1].size.y+15.0;
    session.black_background[1].object.physic_object.pos.x = session.menu_text[1].pos.x-7.5;
    session.black_background[1].object.physic_object.pos.y = session.menu_text[1].pos.y-7.5;
    session.black_background[1].object.physic_object.pos.z = 0.71;
    session.black_background[1].object.draw = true;
    session.black_background[1].object.physic_object.rot = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
    session.black_background[1].exec(&mut session.eng);

    if session.eng.control.gamepad_connected {
        if session.eng.control.get_gamepad_button_state(session.controller_mappings.abxy[0] as u8) && session.action_cooldown <= 0.0{
            session.input_type = InputType::Gamepad;
            session.action_cooldown = 0.25;
            do_paper_sound(session);
            session.returning_to_game = true;
        }
    }

    if session.returning_to_game {
        session.viewport.object.mesh.ubo[52] = smooth_value_towards(
            session.viewport.object.mesh.ubo[52],
            1.0,
            &session.eng,
            2.0,
        );
        session.viewport.object.mesh.ubo[53] = smooth_value_towards(
            session.viewport.object.mesh.ubo[53],
            1.0,
            &session.eng,
            2.0,
        );

        let viewport = &mut session.viewport.object.physic_object;
        viewport.pos.x = smooth_value_towards(viewport.pos.x, 0.0, &session.eng, 2.0);
        viewport.pos.y = smooth_value_towards(viewport.pos.y, 0.0, &session.eng, 2.0);
        viewport.scale.x = session.eng.render.resolution_x as f32;
        viewport.scale.y = session.eng.render.resolution_y as f32;

        if viewport_is_normal(
            viewport,
            session.eng.render.resolution_x as f32,
            session.eng.render.resolution_y as f32,
        ) {
            viewport.scale.x = session.eng.render.resolution_x as f32;
            viewport.scale.y = session.eng.render.resolution_y as f32;
            viewport.pos.x = 0.0;
            viewport.pos.y = 0.0;

            session.awareness = Awarness::Unseen(0);
            session.pivot.tbtmode = false;
            session.making_step = false;
            session.scn.objects[session.pivot.id].physic_object.pos = session.pivot.start_pos;
            for enemy in &mut session.enemies {
                enemy.alive = true;
                enemy.plate_id = enemy.initial_tile_id;
                session.scn.objects[enemy.id].physic_object.pos = enemy.start_pos;
                session.scn.objects[enemy.id].draw = true;
            }
            for object_id in &session.physics_objects {
                let object = &mut session.scn.objects[object_id.0].physic_object;
                object.pos = object_id.1;
            }

            session.speakers[6].move_sound_cursor(0.0);
            session.speakers[6].play = true;

            session.returning_to_game = false;
            session.game_state = super::GameState::InGame;
        }
    } else {
        session.viewport.object.mesh.ubo[52] = smooth_value_towards(
            session.viewport.object.mesh.ubo[52],
            1.0,
            &session.eng,
            2.0,
        );
        session.viewport.object.mesh.ubo[53] = 0.0;

        let viewport = &mut session.viewport.object.physic_object;
        viewport.scale.x = session.eng.render.resolution_x as f32;
        viewport.scale.y = session.eng.render.resolution_y as f32;
        viewport.pos.x = smooth_value_towards(
            viewport.pos.x,
            session.eng.render.resolution_x as f32 * -0.75,
            &session.eng,
            2.0,
        );
        viewport.pos.y = 0.0;
    }
    session.viewport.object.physic_object.pos.z = 0.6;
    session.viewport.object.mesh.ubo[48] = session.frame_count as f32;
    session.viewport.object.mesh.ubo[49] = 0.0;
    session.viewport.object.mesh.ubo[50] = 0.2;
    session.viewport.object.mesh.ubo[51] = 0.18;
    session.viewport.object.draw = true;
    session.viewport.exec(&mut session.eng);
}

fn viewport_is_normal(viewport: &PhysicsObject, width: f32, height: f32) -> bool {
    viewport.pos.x.abs() <= 2.0
        && viewport.pos.y.abs() <= 2.0
        && (viewport.scale.x - width).abs() <= 2.0
        && (viewport.scale.y - height).abs() <= 2.0
}