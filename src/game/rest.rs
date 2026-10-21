use crate::{engine::math::{vec3::Vec3, vec4::Vec4}, game::{Action::{Skip, Step, StepWithElemination}, Awarness}};

use super::{Direction, GameSession, InputType, smooth_value_towards, tile_test};

pub fn update(session: &mut GameSession) {
    for speaker in &mut session.speakers {
        speaker.exec(&mut session.eng);
    }
    session.speakers[3].volume = 0.0;

    let pivot_position = session.scn.objects[session.pivot.id].physic_object.pos;
    session.eng.cameras[0].physic_object.pos.x = pivot_position.x - 5.0;
    session.eng.cameras[0].physic_object.pos.y = pivot_position.y + 7.5;
    session.eng.cameras[0].physic_object.pos.z = pivot_position.z + 10.0;
    session.eng.cameras[0].look_at(
        pivot_position
            + Vec3 {
                x: 0.0,
                y: 0.5,
                z: 0.0,
            },
    );

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

    for plates in &session.tbtplates{
        session.scn.objects[plates.id].mesh.ubo[53] = 0.0;
    }
    for i in 0..5{
        if session.action_list[i].is_some() && !session.making_step && session.action_cooldown <= 0.0{
            let action = session.action_list[i].as_ref().unwrap();
            match action {
                Step(plate) => {
                    session.scn.objects[plate.1].mesh.ubo[53] = 2.0;
                    let mut pos = session.scn.objects[plate.1].physic_object.pos;
                    pos.y = session.scn.objects[plate.1].physic_object.collider.aabb_max.y;
                    let mut cammat = session.eng.cameras[0].get_projection((session.eng.render.resolution_x as f32)/(session.eng.render.resolution_y as f32));
                    cammat.transpose();
                    let screenpos = cammat.vec4mul(Vec4{ x: pos.x, y: pos.y, z: pos.z, w: 1.0});
                    let screenpos = Vec3{ x: ((screenpos.x/screenpos.w+1.0)/2.0)*(session.eng.render.resolution_x as f32), y: (session.eng.render.resolution_y as f32) - ((screenpos.y/screenpos.w+1.0)/2.0)*(session.eng.render.resolution_y as f32), z: 0.0};

                    session.text[i].size.x = 10.0 * session.ui_scale;
                    session.text[i].size.y = 20.0 * session.ui_scale;
                    session.text[i].pos.x = screenpos.x;
                    session.text[i].pos.y = screenpos.y - session.text[i].size.y/2.0;
                    session.text[i].auto_recalculate_clickzone = true;
                    session.text[i].rotate_each_character = false;
                    session.text[i].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.1 }.to_quat();
                    session.text[i].pos.z = 0.1;
                    session.text[i].draw = true;
                    session.text[i].signal = true;
                    session.text[i].per_symbol = false;
                    let text = &session.text_content.actions[0];
                    if session.text[i].exec(&mut session.eng, text) && session.action_cooldown <= 0.0 && session.eng.control.mousebtn[2] {
                        session.input_type = InputType::Touchscreen;
                        session.active_action = Some(Step((match i {
                            0 => Direction::Up,
                            1 => Direction::Down,
                            2 => Direction::Left,
                            3 => Direction::Right,
                            _ => Direction::Up,
                        }, plate.1)));
                    }
                    session.speech_bubble_background[i].object.physic_object.scale.x = (session.text[i].size.x * text.len() as f32 + 12.0) * session.ui_scale;
                    session.speech_bubble_background[i].object.physic_object.scale.y = (session.text[i].size.y + 12.0) * session.ui_scale;
                    session.speech_bubble_background[i].object.physic_object.pos.x = session.text[i].pos.x - 6.0 * session.ui_scale;
                    session.speech_bubble_background[i].object.physic_object.pos.y = session.text[i].pos.y - 6.0 * session.ui_scale;
                    session.speech_bubble_background[i].object.physic_object.pos.z = 0.2;
                    session.speech_bubble_background[i].object.physic_object.rot = Vec3{ x: 0.0, y: 0.0, z: -0.1 }.to_quat();
                    session.speech_bubble_background[i].object.draw = true;
                    session.speech_bubble_background[i].exec(&mut session.eng);

                    session.keyimg[i].size.x = 40.0 * session.ui_scale;
                    session.keyimg[i].size.y = 40.0 * session.ui_scale;
                    session.keyimg[i].pos.z = 0.1;
                    session.keyimg[i].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.1 }.to_quat();
                    match i {
                        0 => {
                            session.keyimg[i].pos.x = session.text[i].pos.x;
                            session.keyimg[i].pos.y = session.text[i].pos.y + session.text[i].size.y;
                        },
                        1 => {
                            session.keyimg[i].pos.x = session.text[i].pos.x;
                            session.keyimg[i].pos.y = session.text[i].pos.y - session.keyimg[i].size.y;
                        },
                        2 => {
                            session.keyimg[i].pos.x = session.text[i].pos.x + (text.len() as f32)*session.text[i].size.x;
                            session.keyimg[i].pos.y = session.text[i].pos.y - session.keyimg[i].size.y/4.0;
                        },
                        3 => {
                            session.keyimg[i].pos.x = session.text[i].pos.x - session.keyimg[i].size.x;
                            session.keyimg[i].pos.y = session.text[i].pos.y - session.keyimg[i].size.y/4.0;
                        },
                        _ => {},
                    }
                    match session.input_type {
                        InputType::Keyboard => {
                            session.keyimg[i].draw = true;
                            session.keyimg[i].exec(&mut session.eng, &session.keyboard_bindings.arrows[i].key);
                        },
                        InputType::Gamepad => {
                            session.keyimg[i].draw = true;
                            session.keyimg[i].exec(&mut session.eng, &session.controller_mappings.arrows_key[i]);
                        },
                        InputType::Touchscreen => {
                            session.keyimg[i].draw = false;
                            session.keyimg[i].exec(&mut session.eng, " ");
                        },
                    }
                },
                StepWithElemination(plate) => {
                    session.scn.objects[plate.1].mesh.ubo[53] = 2.0;
                    let mut pos = session.scn.objects[plate.1].physic_object.pos;
                    pos.y = session.scn.objects[plate.1].physic_object.collider.aabb_max.y;
                    let mut cammat = session.eng.cameras[0].get_projection((session.eng.render.resolution_x as f32)/(session.eng.render.resolution_y as f32));
                    cammat.transpose();
                    let screenpos = cammat.vec4mul(Vec4{ x: pos.x, y: pos.y, z: pos.z, w: 1.0});
                    let screenpos = Vec3{ x: ((screenpos.x/screenpos.w+1.0)/2.0)*(session.eng.render.resolution_x as f32), y: (session.eng.render.resolution_y as f32) - ((screenpos.y/screenpos.w+1.0)/2.0)*(session.eng.render.resolution_y as f32), z: 0.0};

                    session.text[i].size.x = 10.0 * session.ui_scale;
                    session.text[i].size.y = 20.0 * session.ui_scale;
                    session.text[i].pos.x = screenpos.x;
                    session.text[i].pos.y = screenpos.y - session.text[i].size.y/2.0;
                    session.text[i].pos.z = 0.1;
                    session.text[i].draw = true;
                    session.text[i].signal = true;
                    session.text[i].per_symbol = false;
                    session.text[i].auto_recalculate_clickzone = true;
                    session.text[i].rotate_each_character = false;
                    session.text[i].rotation = Vec3{ x: 0.0, y: 0.0, z: 0.1 }.to_quat();
                    let text = &session.text_content.actions[1];

                    session.speech_bubble_background[i].object.physic_object.scale.x = (session.text[i].size.x * text.len() as f32 + 12.0) * session.ui_scale;
                    session.speech_bubble_background[i].object.physic_object.scale.y = (session.text[i].size.y + 12.0) * session.ui_scale;
                    session.speech_bubble_background[i].object.physic_object.pos.x = session.text[i].pos.x - 6.0 * session.ui_scale;
                    session.speech_bubble_background[i].object.physic_object.pos.y = session.text[i].pos.y - 6.0 * session.ui_scale;
                    session.speech_bubble_background[i].object.physic_object.pos.z = 0.2;
                    session.speech_bubble_background[i].object.physic_object.rot = Vec3{ x: 0.0, y: 0.0, z: 0.1 }.to_quat();
                    session.speech_bubble_background[i].object.draw = true;

                    session.keyimg[i].size.x = 50.0 * session.ui_scale;
                    session.keyimg[i].size.y = 50.0 * session.ui_scale;
                    session.keyimg[i].rotation = Vec3{ x: 0.0, y: 0.0, z: 0.1 }.to_quat();
                    session.keyimg[i].pos.z = 0.1;
                    match i {
                        0 => {
                            session.keyimg[i].pos.x = session.text[i].pos.x;
                            session.keyimg[i].pos.y = session.text[i].pos.y + session.text[i].size.y;
                        },
                        1 => {
                            session.keyimg[i].pos.x = session.text[i].pos.x;
                            session.keyimg[i].pos.y = session.text[i].pos.y - session.keyimg[i].size.y;
                        },
                        2 => {
                            session.keyimg[i].pos.x = session.text[i].pos.x + (text.len() as f32)*session.text[i].size.x;
                            session.keyimg[i].pos.y = session.text[i].pos.y - session.keyimg[i].size.y/4.0;
                        },
                        3 => {
                            session.text[i].pos.x -= (text.len() as f32 / 4.0) * session.text[i].size.x;
                            session.speech_bubble_background[i].object.physic_object.pos.x = session.text[i].pos.x - 6.0 * session.ui_scale;
                            session.keyimg[i].pos.x = session.text[i].pos.x - session.keyimg[i].size.x;
                            session.keyimg[i].pos.y = session.text[i].pos.y - session.keyimg[i].size.y/4.0;
                        },
                        _ => {},
                    }
                    match session.input_type {
                        InputType::Keyboard => {
                            session.keyimg[i].draw = true;
                            session.keyimg[i].exec(&mut session.eng, &session.keyboard_bindings.arrows[i].key);
                        },
                        InputType::Gamepad => {
                            session.keyimg[i].draw = true;
                            session.keyimg[i].exec(&mut session.eng, &session.controller_mappings.arrows_key[i]);
                        },
                        InputType::Touchscreen => {
                            session.keyimg[i].draw = false;
                            session.keyimg[i].exec(&mut session.eng, " ");
                        },
                    }
                    session.speech_bubble_background[i].exec(&mut session.eng);
                    if session.text[i].exec(&mut session.eng, text) && session.action_cooldown <= 0.0 && session.eng.control.mousebtn[2] {
                        session.input_type = InputType::Touchscreen;
                        session.active_action = Some(StepWithElemination((match i {
                            0 => Direction::Up,
                            1 => Direction::Down,
                            2 => Direction::Left,
                            3 => Direction::Right,
                            _ => Direction::Up,
                        }, plate.1)));
                    }
                },
                Skip => {
                    let text = &session.text_content.actions[2];
                    session.text[i].size.x = 15.0 * session.ui_scale;
                    session.text[i].size.y = 30.0 * session.ui_scale;
                    session.text[i].pos.x = session.eng.render.resolution_x as f32 / 2.0 - text.len() as f32;
                    session.text[i].pos.y = session.eng.render.resolution_y as f32 - session.text[i].size.y*2.0;
                    session.text[i].pos.z = 0.1;
                    session.text[i].draw = true;
                    session.text[i].signal = true;
                    session.text[i].per_symbol = false;
                    session.text[i].auto_recalculate_clickzone = true;
                    session.text[i].rotate_each_character = false;
                    session.text[i].rotation = Vec3{ x: 0.0, y: 0.0, z: 0.1 }.to_quat();

                    session.speech_bubble_background[i].object.physic_object.scale.x = (session.text[i].size.x * text.len() as f32 + 12.0) * session.ui_scale;
                    session.speech_bubble_background[i].object.physic_object.scale.y = (session.text[i].size.y + 12.0) * session.ui_scale;
                    session.speech_bubble_background[i].object.physic_object.pos.x = session.text[i].pos.x - 6.0 * session.ui_scale;
                    session.speech_bubble_background[i].object.physic_object.pos.y = session.text[i].pos.y - 6.0 * session.ui_scale;
                    session.speech_bubble_background[i].object.physic_object.pos.z = 0.2;
                    session.speech_bubble_background[i].object.physic_object.rot = Vec3{ x: 0.0, y: 0.0, z: 0.1 }.to_quat();
                    session.speech_bubble_background[i].object.draw = true;

                    session.keyimg[i].size.x = 50.0 * session.ui_scale;
                    session.keyimg[i].size.y = 50.0 * session.ui_scale;
                    session.keyimg[i].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.1 }.to_quat();
                    session.keyimg[i].pos.z = 0.1;
                    session.keyimg[i].pos.x = session.speech_bubble_background[i].object.physic_object.pos.x - session.keyimg[i].size.x/1.125;
                    session.keyimg[i].pos.y = session.eng.render.resolution_y as f32 - session.keyimg[i].size.y*1.5;

                    match session.input_type {
                        InputType::Keyboard => {
                            session.keyimg[i].draw = true;
                            session.keyimg[i].exec(&mut session.eng, &session.keyboard_bindings.abxy[0].key);
                        },
                        InputType::Gamepad => {
                            session.keyimg[i].draw = true;
                            session.keyimg[i].exec(&mut session.eng, &session.controller_mappings.abxy_key[0]);
                        },
                        InputType::Touchscreen => {
                            session.keyimg[i].draw = false;
                            session.keyimg[i].exec(&mut session.eng, " ");
                        },
                    }

                    session.speech_bubble_background[i].exec(&mut session.eng);
                    if session.text[i].exec(&mut session.eng, text) && session.action_cooldown <= 0.0 && session.eng.control.mousebtn[2] {
                        session.input_type = InputType::Touchscreen;
                        session.active_action = Some(Skip);
                    }
                },
            }
        }else{
            session.speech_bubble_background[i].object.draw = false;
            session.speech_bubble_background[i].exec(&mut session.eng);
            session.text[i].draw = false;
            session.text[i].exec(&mut session.eng, " ");
            session.keyimg[i].draw = false;
            session.keyimg[i].exec(&mut session.eng, " ");
        }
    }

    session.scn.exec(&mut session.eng);
    session.ltviewport.object.physic_object.scale.x =
        session.eng.render.resolution_x as f32 * session.eng.render.resolution_scale;
    session.ltviewport.object.physic_object.scale.y =
        session.eng.render.resolution_y as f32 * session.eng.render.resolution_scale;
    session.ltviewport.object.physic_object.pos.z = 0.9;
    session.ltviewport.object.mesh.ubo[48] = session.frame_count as f32;
    session.ltviewport.object.mesh.ubo[49] = session.scn.voxel_representation.size[0] as f32;
    session.ltviewport.object.mesh.ubo[50] = session.scn.voxel_representation.size[1] as f32;
    session.ltviewport.object.mesh.ubo[51] = session.scn.voxel_representation.size[2] as f32;
    session.ltviewport.object.mesh.ubo[52] = session.scn.voxel_representation.origin.x as f32;
    session.ltviewport.object.mesh.ubo[53] = session.scn.voxel_representation.origin.y as f32;
    session.ltviewport.object.mesh.ubo[54] = session.scn.voxel_representation.origin.z as f32;
    session.ltviewport.object.mesh.ubo[55] = session.scn.voxel_representation.voxel_size;
    session.ltviewport.object.draw = true;
    session.ltviewport.exec(&mut session.eng);

    session.scn.objects[session.pivot.id].mesh.ubo[52] = 1.0;

    if !session.pivot.tbtmode {
        for i in 0..session.action_list.len() {
            session.action_list[i] = None;
        }
        for (index, plate) in session.tbtplates.iter().enumerate() {
            if tile_test(
                &session.scn.objects[session.pivot.id].physic_object,
                &session.scn.objects[plate.id].physic_object,
            ) && !plate.safe
            {
                session.actual_plate_id = index;
                session.pivot.tbtmode = true;
                break;
            }
        }

        session.scn.objects[session.pivot.id].mesh.ubo[53] = 0.0;
        for enemy in &session.enemies{
            session.scn.objects[enemy.id].mesh.ubo[53] = 0.0;
        }

        for pl in &mut session.awarness_signs{
            pl.object.physic_object.pos.x = smooth_value_towards(
                pl.object.physic_object.pos.x,
                -150.0,
                &session.eng,
                1.0,
            );
            pl.object.physic_object.scale.x = 100.0 * session.ui_scale;
            pl.object.physic_object.scale.y = 100.0 * session.ui_scale;
            pl.object.physic_object.pos.y = 10.0;
            pl.object.draw = true;
            pl.exec(&mut session.eng);
        }
    }else{
        session.scn.objects[session.pivot.id].mesh.ubo[53] = 1.0;
        for enemy in &session.enemies{
            session.scn.objects[enemy.id].mesh.ubo[53] = 1.0;
        }

        match session.awareness {
            Awarness::Unseen(l) => {
                session.awarness_signs[0].object.physic_object.scale.x = 100.0 * session.ui_scale;
                session.awarness_signs[0].object.physic_object.scale.y = 100.0 * session.ui_scale;
                session.awarness_signs[0].object.physic_object.pos.x = smooth_value_towards(
                    session.awarness_signs[0].object.physic_object.pos.x,
                    10.0,
                    &session.eng,
                    10.0,
                );
                session.awarness_signs[0].object.physic_object.pos.y = 10.0;
                session.awarness_signs[0].object.physic_object.pos.z = 0.1;
                session.awarness_signs[0].object.draw = true;
                session.awarness_signs[0].exec(&mut session.eng);

                for i in 1..3{
                    //session.awarness_signs[i].object.draw = false;
                    session.awarness_signs[i].object.physic_object.pos.y = 10.0;
                    session.awarness_signs[i].object.physic_object.pos.x = smooth_value_towards(
                        session.awarness_signs[i].object.physic_object.pos.x,
                        -150.0,
                        &session.eng,
                        10.0,
                    );
                    session.awarness_signs[i].object.draw = true;
                    session.awarness_signs[i].exec(&mut session.eng);
                }
            },
            Awarness::Suspected(l) => {
                session.awarness_signs[0].object.physic_object.scale.x = 100.0 * session.ui_scale;
                session.awarness_signs[0].object.physic_object.scale.y = 100.0 * session.ui_scale;
                session.awarness_signs[0].object.physic_object.pos.x = smooth_value_towards(
                    session.awarness_signs[0].object.physic_object.pos.x,
                    10.0,
                    &session.eng,
                    10.0,
                );
                session.awarness_signs[0].object.physic_object.pos.y = 10.0;
                session.awarness_signs[0].object.physic_object.pos.z = 0.1;
                session.awarness_signs[0].object.draw = true;
                session.awarness_signs[0].exec(&mut session.eng);

                session.awarness_signs[1].object.physic_object.scale.x = 100.0 * session.ui_scale;
                session.awarness_signs[1].object.physic_object.scale.y = 100.0 * session.ui_scale;
                //session.awarness_signs[1].object.physic_object.pos.x = 10.0;
                session.awarness_signs[1].object.physic_object.pos.x = smooth_value_towards(
                    session.awarness_signs[1].object.physic_object.pos.x,
                    20.0 + session.awarness_signs[1].object.physic_object.scale.x,
                    &session.eng,
                    10.0,
                );
                session.awarness_signs[1].object.physic_object.pos.y = 10.0;
                session.awarness_signs[1].object.physic_object.pos.z = 0.2;
                session.awarness_signs[1].object.draw = true;
                session.awarness_signs[1].exec(&mut session.eng);

                for i in 2..3{
                    session.awarness_signs[i].object.physic_object.pos.x = smooth_value_towards(
                        session.awarness_signs[i].object.physic_object.pos.x,
                        -150.0,
                        &session.eng,
                        10.0,
                    );
                    session.awarness_signs[i].object.draw = true;
                    session.awarness_signs[i].exec(&mut session.eng);
                }
            },
            Awarness::Detected(l) => {
                session.awarness_signs[0].object.physic_object.scale.x = 100.0 * session.ui_scale;
                session.awarness_signs[0].object.physic_object.scale.y = 100.0 * session.ui_scale;
                session.awarness_signs[0].object.physic_object.pos.x = smooth_value_towards(
                    session.awarness_signs[0].object.physic_object.pos.x,
                    10.0,
                    &session.eng,
                    10.0,
                );
                session.awarness_signs[0].object.physic_object.pos.y = 10.0;
                session.awarness_signs[0].object.physic_object.pos.z = 0.1;
                session.awarness_signs[0].object.draw = true;
                session.awarness_signs[0].exec(&mut session.eng);

                session.awarness_signs[1].object.physic_object.scale.x = 100.0 * session.ui_scale;
                session.awarness_signs[1].object.physic_object.scale.y = 100.0 * session.ui_scale;
                //session.awarness_signs[1].object.physic_object.pos.x = 10.0;
                session.awarness_signs[1].object.physic_object.pos.x = smooth_value_towards(
                    session.awarness_signs[1].object.physic_object.pos.x,
                    20.0 + session.awarness_signs[1].object.physic_object.scale.x,
                    &session.eng,
                    10.0,
                );
                session.awarness_signs[1].object.physic_object.pos.y = 10.0;
                session.awarness_signs[1].object.physic_object.pos.z = 0.2;
                session.awarness_signs[1].object.draw = true;
                session.awarness_signs[1].exec(&mut session.eng);

                session.awarness_signs[2].object.physic_object.scale.x = 100.0 * session.ui_scale;
                session.awarness_signs[2].object.physic_object.scale.y = 100.0 * session.ui_scale;
                //session.awarness_signs[1].object.physic_object.pos.x = 10.0;
                session.awarness_signs[2].object.physic_object.pos.x = smooth_value_towards(
                    session.awarness_signs[2].object.physic_object.pos.x,
                    (10.0 + session.awarness_signs[2].object.physic_object.scale.x) * 2.0 + 10.0,
                    &session.eng,
                    10.0,
                );
                session.awarness_signs[2].object.physic_object.pos.y = 10.0;
                session.awarness_signs[2].object.physic_object.pos.z = 0.3;
                session.awarness_signs[2].object.draw = true;
                session.awarness_signs[2].exec(&mut session.eng);

                session.awarness_signs[3].object.physic_object.pos.x = smooth_value_towards(
                    session.awarness_signs[3].object.physic_object.pos.x,
                    -150.0,
                    &session.eng,
                    10.0,
                );
                session.awarness_signs[3].object.draw = true;
                session.awarness_signs[3].exec(&mut session.eng);
            },
            Awarness::Compromised(l) => {
                session.awarness_signs[0].object.physic_object.scale.x = 100.0 * session.ui_scale;
                session.awarness_signs[0].object.physic_object.scale.y = 100.0 * session.ui_scale;
                session.awarness_signs[0].object.physic_object.pos.x = smooth_value_towards(
                    session.awarness_signs[0].object.physic_object.pos.x,
                    20.0,
                    &session.eng,
                    10.0,
                );
                session.awarness_signs[0].object.physic_object.pos.y = 10.0;
                session.awarness_signs[0].object.physic_object.pos.z = 0.1;
                session.awarness_signs[0].object.draw = true;
                session.awarness_signs[0].exec(&mut session.eng);

                session.awarness_signs[1].object.physic_object.scale.x = 100.0 * session.ui_scale;
                session.awarness_signs[1].object.physic_object.scale.y = 100.0 * session.ui_scale;
                //session.awarness_signs[1].object.physic_object.pos.x = 10.0;
                session.awarness_signs[1].object.physic_object.pos.x = smooth_value_towards(
                    session.awarness_signs[1].object.physic_object.pos.x,
                    10.0 + session.awarness_signs[1].object.physic_object.scale.x,
                    &session.eng,
                    10.0,
                );
                session.awarness_signs[1].object.physic_object.pos.y = 10.0;
                session.awarness_signs[1].object.physic_object.pos.z = 0.2;
                session.awarness_signs[1].object.draw = true;
                session.awarness_signs[1].exec(&mut session.eng);

                session.awarness_signs[2].object.physic_object.scale.x = 100.0 * session.ui_scale;
                session.awarness_signs[2].object.physic_object.scale.y = 100.0 * session.ui_scale;
                //session.awarness_signs[1].object.physic_object.pos.x = 10.0;
                session.awarness_signs[2].object.physic_object.pos.x = smooth_value_towards(
                    session.awarness_signs[2].object.physic_object.pos.x,
                    (10.0 + session.awarness_signs[2].object.physic_object.scale.x) * 2.0 + 10.0,
                    &session.eng,
                    10.0,
                );
                session.awarness_signs[2].object.physic_object.pos.y = 10.0;
                session.awarness_signs[2].object.physic_object.pos.z = 0.3;
                session.awarness_signs[2].object.draw = true;
                session.awarness_signs[2].exec(&mut session.eng);

                session.awarness_signs[3].object.physic_object.scale.x = 100.0 * session.ui_scale;
                session.awarness_signs[3].object.physic_object.scale.y = 100.0 * session.ui_scale;
                //session.awarness_signs[1].object.physic_object.pos.x = 10.0;
                session.awarness_signs[3].object.physic_object.pos.x = smooth_value_towards(
                    session.awarness_signs[3].object.physic_object.pos.x,
                    (10.0 + session.awarness_signs[3].object.physic_object.scale.x) * 3.0 + 10.0,
                    &session.eng,
                    10.0,
                );
                session.awarness_signs[3].object.physic_object.pos.y = 10.0;
                session.awarness_signs[3].object.physic_object.pos.z = 0.4;
                session.awarness_signs[3].object.draw = true;
                session.awarness_signs[3].exec(&mut session.eng);
            },
        }
    }

    if let Some(direction) = session.object_holding {
        let point = session.scn.objects[session.pivot.id].physic_object.pos + direction.1;
        if direction.0.is_some() {
            let object = &mut session.scn.objects[direction.0.unwrap()].physic_object;
            object.acceleration = (point - object.pos) * 100.0;
        }else{
            for object_id in &session.physics_objects {
                let object = &mut session.scn.objects[object_id.0].physic_object;
                if (object.pos - point).length_squared().sqrt() <= 1.8 {
                    object.acceleration = (point - object.pos) * 100.0;
                }
                session.object_holding = Some((Some(object_id.0), direction.1));
                break;
            }
        }
    }

    for wbg in &mut session.white_background{
        wbg.object.draw = false;
        wbg.exec(&mut session.eng);
    }

    session.viewport.object.physic_object.scale.x = session.eng.render.resolution_x as f32;
    session.viewport.object.physic_object.scale.y = session.eng.render.resolution_y as f32;
    session.viewport.object.mesh.ubo[48] = session.frame_count as f32;
    session.viewport.object.mesh.ubo[49] = 0.0;
    session.viewport.object.mesh.ubo[50] = 0.2;
    session.viewport.object.mesh.ubo[51] = 0.18;
    session.viewport.object.draw = true;
    session.viewport.exec(&mut session.eng);
}
