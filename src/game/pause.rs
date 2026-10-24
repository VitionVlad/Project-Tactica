use crate::engine::{math::vec3::Vec3, physics::physics::PhysicsObject};
use crate::game::SubMenu::{self, Audio, Game, Graphics, Main, Settings};
use crate::game::{InputType, smooth_value_towards};

use super::GameSession;

pub fn updatesubemnumenu(session: &mut GameSession){
    for i in 0..session.text_content.pause.len(){
        session.menu_text[i].size.x = 30.0 * session.ui_scale;
        session.menu_text[i].size.y = 60.0 * session.ui_scale;
        session.menu_text[i].pos.x = session.black_background[0].object.physic_object.pos.x - (session.eng.render.resolution_x as f32 / 2.0) + session.menu_text[i].size.x*2.0;
        session.menu_text[i].pos.y = session.eng.render.resolution_y as f32 / 2.0 + (session.menu_text[i].size.y + 30.0)*(i as f32) + session.viewport.object.physic_object.pos.y;
        session.menu_text[i].pos.z = 0.7;
        session.menu_text[i].draw = true;
        session.menu_text[i].signal = true;
        session.menu_text[i].per_symbol = false;
        session.menu_text[i].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
        if session.menu_text[i].exec(&mut session.eng, &session.text_content.pause[i]) && session.eng.control.mousebtn[2] && session.action_cooldown <= 0.0{
            match i {
                0 => {
                    session.returning_to_game = true;
                }
                1 => {
                    session.submenu = SubMenu::Settings;
                }
                2 => {
                    session.exit = true;
                }
                _ => {}
            }
            session.input_type = InputType::Touchscreen;
            session.action_cooldown = 0.25;
        }

        session.black_background[i+1].object.physic_object.scale.x = session.menu_text[i].size.x*(session.text_content.pause[i].len() as f32)+15.0;
        session.black_background[i+1].object.physic_object.scale.y = session.menu_text[i].size.y+15.0;
        session.black_background[i+1].object.physic_object.pos.x = session.menu_text[i].pos.x-7.5;
        session.black_background[i+1].object.physic_object.pos.y = session.menu_text[i].pos.y-7.5;
        session.black_background[i+1].object.physic_object.pos.z = 0.71;
        session.black_background[i+1].object.draw = true;
        session.black_background[i+1].object.physic_object.rot = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
        session.black_background[i+1].exec(&mut session.eng);
    }
}

pub fn updatesubemnusettings(session: &mut GameSession){
    session.menu_text[3].size.x = 40.0 * session.ui_scale;
    session.menu_text[3].size.y = 80.0 * session.ui_scale;
    if session.submenu != Settings && session.submenu != Main{
        session.menu_text[3].pos.x = smooth_value_towards(session.menu_text[3].pos.x, session.menu_text[0].pos.x + session.eng.render.resolution_x as f32*0.9, &session.eng, 2.0);
    }else{
        session.menu_text[3].pos.x = smooth_value_towards(session.menu_text[3].pos.x, session.menu_text[0].pos.x, &session.eng, 2.0);
    }
    session.menu_text[3].pos.y = session.viewport.object.physic_object.pos.y -(session.eng.render.resolution_y as f32 * 0.6) - (session.menu_text[3].size.y + 30.0);
    session.menu_text[3].pos.z = 0.7;
    session.menu_text[3].draw = true;
    session.menu_text[3].signal = false;
    session.menu_text[3].per_symbol = false;
    session.menu_text[3].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
    session.menu_text[3].exec(&mut session.eng, &session.text_content.pause[1]);

    session.black_background[4].object.physic_object.scale.x = session.menu_text[3].size.x*(session.text_content.pause[1].len() as f32)+15.0;
    session.black_background[4].object.physic_object.scale.y = session.menu_text[3].size.y+15.0;
    session.black_background[4].object.physic_object.pos.x = session.menu_text[3].pos.x-7.5;
    session.black_background[4].object.physic_object.pos.y = session.menu_text[3].pos.y-7.5;
    session.black_background[4].object.physic_object.pos.z = 0.71;
    session.black_background[4].object.draw = true;
    session.black_background[4].object.physic_object.rot = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
    session.black_background[4].exec(&mut session.eng);

    for i in 4..(session.text_content.settings.len()+4){
        session.menu_text[i].size.x = 30.0 * session.ui_scale;
        session.menu_text[i].size.y = 60.0 * session.ui_scale;
        if session.submenu != Settings && session.submenu != Main{
            session.menu_text[i].pos.x = smooth_value_towards(session.menu_text[i].pos.x, session.menu_text[0].pos.x + session.eng.render.resolution_x as f32*0.9, &session.eng, 2.0);
        }else{
            session.menu_text[i].pos.x = smooth_value_towards(session.menu_text[i].pos.x, session.menu_text[0].pos.x, &session.eng, 2.0);
        }
        session.menu_text[i].pos.y = session.viewport.object.physic_object.pos.y -(session.eng.render.resolution_y as f32 * 0.6) + (session.menu_text[i].size.y + 30.0)*(i as f32 - 4.0);
        session.menu_text[i].pos.z = 0.7;
        session.menu_text[i].draw = true;
        session.menu_text[i].signal = true;
        session.menu_text[i].per_symbol = false;
        session.menu_text[i].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
        if session.menu_text[i].exec(&mut session.eng, &session.text_content.settings[i-4]) && session.eng.control.mousebtn[2] && session.action_cooldown <= 0.0{
            match i {
                4 => {
                    session.submenu = SubMenu::Game;
                }
                5 => {
                    session.submenu = SubMenu::Audio;
                }
                6 => {
                    session.submenu = SubMenu::Graphics;
                }
                7 => {
                    session.submenu = SubMenu::Main;
                }
                _ => {}
            }
            session.input_type = InputType::Touchscreen;
            session.action_cooldown = 0.25;
        }

        session.black_background[i+1].object.physic_object.scale.x = session.menu_text[i].size.x*(session.text_content.settings[i-4].len() as f32)+15.0;
        session.black_background[i+1].object.physic_object.scale.y = session.menu_text[i].size.y+15.0;
        session.black_background[i+1].object.physic_object.pos.x = session.menu_text[i].pos.x-7.5;
        session.black_background[i+1].object.physic_object.pos.y = session.menu_text[i].pos.y-7.5;
        session.black_background[i+1].object.physic_object.pos.z = 0.71;
        session.black_background[i+1].object.draw = true;
        session.black_background[i+1].object.physic_object.rot = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
        session.black_background[i+1].exec(&mut session.eng);
    }
}

pub fn updatesubemnugame(session: &mut GameSession){
    session.menu_text[8].size.x = 40.0 * session.ui_scale;
    session.menu_text[8].size.y = 80.0 * session.ui_scale;
    if session.submenu == Game{
        session.menu_text[8].pos.x = smooth_value_towards(session.menu_text[8].pos.x, session.menu_text[0].pos.x, &session.eng, 2.0);
    }else{
        session.menu_text[8].pos.x = smooth_value_towards(session.menu_text[8].pos.x, session.menu_text[0].pos.x - session.eng.render.resolution_x as f32, &session.eng, 2.0);
    }
    session.menu_text[8].pos.y = session.viewport.object.physic_object.pos.y -(session.eng.render.resolution_y as f32 * 0.6) - (session.menu_text[3].size.y + 30.0);
    session.menu_text[8].pos.z = 0.7;
    session.menu_text[8].draw = true;
    session.menu_text[8].signal = false;
    session.menu_text[8].per_symbol = false;
    session.menu_text[8].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
    session.menu_text[8].exec(&mut session.eng, &session.text_content.settings[0]);

    session.black_background[9].object.physic_object.scale.x = session.menu_text[8].size.x*(session.text_content.settings[0].len() as f32)+15.0;
    session.black_background[9].object.physic_object.scale.y = session.menu_text[8].size.y+15.0;
    session.black_background[9].object.physic_object.pos.x = session.menu_text[8].pos.x-7.5;
    session.black_background[9].object.physic_object.pos.y = session.menu_text[8].pos.y-7.5;
    session.black_background[9].object.physic_object.pos.z = 0.71;
    session.black_background[9].object.draw = true;
    session.black_background[9].object.physic_object.rot = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
    session.black_background[9].exec(&mut session.eng);

    for i in 9..(session.text_content.game_set.len()+9){
        session.menu_text[i].size.x = 30.0 * session.ui_scale;
        session.menu_text[i].size.y = 60.0 * session.ui_scale;
        if session.submenu == Game{
            session.menu_text[i].pos.x = smooth_value_towards(session.menu_text[i].pos.x, session.menu_text[0].pos.x, &session.eng, 2.0);
        }else{
            session.menu_text[i].pos.x = smooth_value_towards(session.menu_text[i].pos.x, session.menu_text[0].pos.x - session.eng.render.resolution_x as f32, &session.eng, 2.0);
        }
        session.menu_text[i].pos.y = session.viewport.object.physic_object.pos.y -(session.eng.render.resolution_y as f32 * 0.6) + (session.menu_text[i].size.y + 30.0)*(i as f32 - 9.0);
        session.menu_text[i].pos.z = 0.7;
        session.menu_text[i].draw = true;
        session.menu_text[i].signal = true;
        session.menu_text[i].per_symbol = false;
        session.menu_text[i].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
        if session.menu_text[i].exec(&mut session.eng, &session.text_content.game_set[i-9]) && session.eng.control.mousebtn[2] && session.action_cooldown <= 0.0{
            match i {
                9 => {
                    //session.returning_to_game = true;
                },
                10 => {
                    //session.returning_to_game = true;
                },
                11 => {
                    //session.returning_to_game = true;
                }
                12 => {
                    session.submenu = SubMenu::Settings;
                }
                _ => {}
            }
            session.input_type = InputType::Touchscreen;
            session.action_cooldown = 0.25;
        }

        session.black_background[i+1].object.physic_object.scale.x = session.menu_text[i].size.x*(session.text_content.settings[i-9].len() as f32)+15.0;
        session.black_background[i+1].object.physic_object.scale.y = session.menu_text[i].size.y+15.0;
        session.black_background[i+1].object.physic_object.pos.x = session.menu_text[i].pos.x-7.5;
        session.black_background[i+1].object.physic_object.pos.y = session.menu_text[i].pos.y-7.5;
        session.black_background[i+1].object.physic_object.pos.z = 0.71;
        session.black_background[i+1].object.draw = true;
        session.black_background[i+1].object.physic_object.rot = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
        session.black_background[i+1].exec(&mut session.eng);
    }
}

pub fn updatesubemnuaudio(session: &mut GameSession){
    session.menu_text[13].size.x = 40.0 * session.ui_scale;
    session.menu_text[13].size.y = 80.0 * session.ui_scale;
    if session.submenu == Audio{
        session.menu_text[13].pos.x = smooth_value_towards(session.menu_text[13].pos.x, session.menu_text[0].pos.x, &session.eng, 2.0);
    }else{
        session.menu_text[13].pos.x = smooth_value_towards(session.menu_text[13].pos.x, session.menu_text[0].pos.x - session.eng.render.resolution_x as f32, &session.eng, 2.0);
    }
    session.menu_text[13].pos.y = session.viewport.object.physic_object.pos.y -(session.eng.render.resolution_y as f32 * 0.6) - (session.menu_text[3].size.y + 30.0);
    session.menu_text[13].pos.z = 0.7;
    session.menu_text[13].draw = true;
    session.menu_text[13].signal = false;
    session.menu_text[13].per_symbol = false;
    session.menu_text[13].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
    session.menu_text[13].exec(&mut session.eng, &session.text_content.settings[1]);

    session.black_background[14].object.physic_object.scale.x = session.menu_text[13].size.x*(session.text_content.settings[1].len() as f32)+15.0;
    session.black_background[14].object.physic_object.scale.y = session.menu_text[13].size.y+15.0;
    session.black_background[14].object.physic_object.pos.x = session.menu_text[13].pos.x-7.5;
    session.black_background[14].object.physic_object.pos.y = session.menu_text[13].pos.y-7.5;
    session.black_background[14].object.physic_object.pos.z = 0.71;
    session.black_background[14].object.draw = true;
    session.black_background[14].object.physic_object.rot = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
    session.black_background[14].exec(&mut session.eng);

    for i in 14..(session.text_content.audio.len()+14){
        session.menu_text[i].size.x = 30.0 * session.ui_scale;
        session.menu_text[i].size.y = 60.0 * session.ui_scale;
        if session.submenu == Audio{
            session.menu_text[i].pos.x = smooth_value_towards(session.menu_text[i].pos.x, session.menu_text[0].pos.x, &session.eng, 2.0);
        }else{
            session.menu_text[i].pos.x = smooth_value_towards(session.menu_text[i].pos.x, session.menu_text[0].pos.x - session.eng.render.resolution_x as f32, &session.eng, 2.0);
        }
        session.menu_text[i].pos.y = session.viewport.object.physic_object.pos.y -(session.eng.render.resolution_y as f32 * 0.6) + (session.menu_text[i].size.y + 30.0)*(i as f32 - 14.0);
        session.menu_text[i].pos.z = 0.7;
        session.menu_text[i].draw = true;
        session.menu_text[i].signal = true;
        session.menu_text[i].per_symbol = false;
        session.menu_text[i].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
        if session.menu_text[i].exec(&mut session.eng, &session.text_content.audio[i-14]) && session.eng.control.mousebtn[2] && session.action_cooldown <= 0.0{
            match i {
                14 => {
                    //session.returning_to_game = true;
                },
                15 => {
                    session.submenu = SubMenu::Settings;
                }
                _ => {}
            }
            session.input_type = InputType::Touchscreen;
            session.action_cooldown = 0.25;
        }

        session.black_background[i+1].object.physic_object.scale.x = session.menu_text[i].size.x*(session.text_content.audio[i-14].len() as f32)+15.0;
        session.black_background[i+1].object.physic_object.scale.y = session.menu_text[i].size.y+15.0;
        session.black_background[i+1].object.physic_object.pos.x = session.menu_text[i].pos.x-7.5;
        session.black_background[i+1].object.physic_object.pos.y = session.menu_text[i].pos.y-7.5;
        session.black_background[i+1].object.physic_object.pos.z = 0.71;
        session.black_background[i+1].object.draw = true;
        session.black_background[i+1].object.physic_object.rot = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
        session.black_background[i+1].exec(&mut session.eng);
    }
}

pub fn updatesubemnugraphics(session: &mut GameSession){
    session.menu_text[16].size.x = 40.0 * session.ui_scale;
    session.menu_text[16].size.y = 80.0 * session.ui_scale;
    if session.submenu == Graphics{
        session.menu_text[16].pos.x = smooth_value_towards(session.menu_text[16].pos.x, session.menu_text[0].pos.x, &session.eng, 2.0);
    }else{
        session.menu_text[16].pos.x = smooth_value_towards(session.menu_text[16].pos.x, session.menu_text[0].pos.x - session.eng.render.resolution_x as f32, &session.eng, 2.0);
    }
    session.menu_text[16].pos.y = session.viewport.object.physic_object.pos.y -(session.eng.render.resolution_y as f32 * 0.675) - (session.menu_text[3].size.y + 30.0);
    session.menu_text[16].pos.z = 0.7;
    session.menu_text[16].draw = true;
    session.menu_text[16].signal = false;
    session.menu_text[16].per_symbol = false;
    session.menu_text[16].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
    session.menu_text[16].exec(&mut session.eng, &session.text_content.settings[2]);

    session.black_background[17].object.physic_object.scale.x = session.menu_text[16].size.x*(session.text_content.settings[1].len() as f32)+15.0;
    session.black_background[17].object.physic_object.scale.y = session.menu_text[16].size.y+15.0;
    session.black_background[17].object.physic_object.pos.x = session.menu_text[16].pos.x-7.5;
    session.black_background[17].object.physic_object.pos.y = session.menu_text[16].pos.y-7.5;
    session.black_background[17].object.physic_object.pos.z = 0.71;
    session.black_background[17].object.draw = true;
    session.black_background[17].object.physic_object.rot = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
    session.black_background[17].exec(&mut session.eng);

    for i in 17..(session.text_content.graphics.len()+15){
        session.menu_text[i].size.x = 30.0 * session.ui_scale;
        session.menu_text[i].size.y = 60.0 * session.ui_scale;
        if session.submenu == Graphics{
            session.menu_text[i].pos.x = smooth_value_towards(session.menu_text[i].pos.x, session.menu_text[0].pos.x, &session.eng, 2.0);
        }else{
            session.menu_text[i].pos.x = smooth_value_towards(session.menu_text[i].pos.x, session.menu_text[0].pos.x - session.eng.render.resolution_x as f32, &session.eng, 2.0);
        }
        session.menu_text[i].pos.y = session.viewport.object.physic_object.pos.y -(session.eng.render.resolution_y as f32 * 0.675) + (session.menu_text[i].size.y + 30.0)*(i as f32 - 17.0);
        session.menu_text[i].pos.z = 0.7;
        session.menu_text[i].draw = true;
        session.menu_text[i].signal = true;
        session.menu_text[i].per_symbol = false;
        session.menu_text[i].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
        if session.menu_text[i].exec(&mut session.eng, &session.text_content.graphics[i-17]) && session.eng.control.mousebtn[2] && session.action_cooldown <= 0.0{
            match i {
                17 => {
                    //session.returning_to_game = true;
                },
                18 => {
                    //session.submenu = SubMenu::Settings;
                },
                19 => {
                    //session.submenu = SubMenu::Settings;
                },
                20 => {
                    //session.submenu = SubMenu::Settings;
                },
                21 => {
                    session.submenu = SubMenu::Settings;
                }
                _ => {}
            }
            session.input_type = InputType::Touchscreen;
            session.action_cooldown = 0.25;
        }

        session.black_background[i+1].object.physic_object.scale.x = session.menu_text[i].size.x*(session.text_content.graphics[i-17].len() as f32)+15.0;
        session.black_background[i+1].object.physic_object.scale.y = session.menu_text[i].size.y+15.0;
        session.black_background[i+1].object.physic_object.pos.x = session.menu_text[i].pos.x-7.5;
        session.black_background[i+1].object.physic_object.pos.y = session.menu_text[i].pos.y-7.5;
        session.black_background[i+1].object.physic_object.pos.z = 0.71;
        session.black_background[i+1].object.draw = true;
        session.black_background[i+1].object.physic_object.rot = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
        session.black_background[i+1].exec(&mut session.eng);
    }
}

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

    session.menu_background[0].object.physic_object.scale.x = session.eng.render.resolution_x as f32;
    session.menu_background[0].object.physic_object.scale.y = session.eng.render.resolution_y as f32;
    session.menu_background[0].object.physic_object.pos.x = 0.0;
    session.menu_background[0].object.physic_object.pos.y = session.viewport.object.physic_object.pos.y;
    session.menu_background[0].object.physic_object.pos.z = 0.99;
    session.menu_background[0].object.draw = true;
    session.menu_background[0].exec(&mut session.eng);

    session.menu_background[1].object.physic_object.scale.x = session.eng.render.resolution_x as f32;
    session.menu_background[1].object.physic_object.scale.y = session.eng.render.resolution_y as f32;
    session.menu_background[1].object.physic_object.pos.x = 0.0;
    session.menu_background[1].object.physic_object.pos.y = session.viewport.object.physic_object.pos.y - session.eng.render.resolution_y as f32 - 30.0;
    session.menu_background[1].object.physic_object.pos.z = 0.99;
    session.menu_background[1].object.draw = true;
    session.menu_background[1].exec(&mut session.eng);

    session.black_background[0].object.physic_object.scale.x = session.viewport.object.physic_object.scale.x + 50.0;
    session.black_background[0].object.physic_object.scale.y = session.viewport.object.physic_object.scale.y + 50.0;
    session.black_background[0].object.physic_object.pos.x = session.viewport.object.physic_object.pos.x - 25.0;
    session.black_background[0].object.physic_object.pos.y = session.viewport.object.physic_object.pos.y - 25.0;
    session.black_background[0].object.physic_object.pos.z = 0.66;
    session.black_background[0].object.draw = true;
    session.black_background[0].object.physic_object.rot = session.viewport.object.physic_object.rot;
    session.black_background[0].exec(&mut session.eng);

    updatesubemnumenu(session);
    updatesubemnusettings(session);
    updatesubemnugame(session);
    updatesubemnuaudio(session);
    updatesubemnugraphics(session);

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
        viewport.scale.y = session.eng.render.resolution_y as f32;
        viewport.pos.x = smooth_value_towards(
            viewport.pos.x,
            session.eng.render.resolution_x as f32 * 0.5,
            &session.eng,
            2.0,
        );
        match session.submenu {
            Main => {
                viewport.pos.y = smooth_value_towards(
                    viewport.pos.y,
                    0.0,
                    &session.eng,
                    2.0,
                );
            },
            Settings => {
                viewport.pos.y = smooth_value_towards(
                    viewport.pos.y,
                    session.eng.render.resolution_y as f32 * 0.9,
                    &session.eng,
                    2.0,
                );
            },
            Game => {
                viewport.pos.y = smooth_value_towards(
                    viewport.pos.y,
                    session.eng.render.resolution_y as f32 * 0.9,
                    &session.eng,
                    2.0,
                );
            },
            Audio => {
                viewport.pos.y = smooth_value_towards(
                    viewport.pos.y,
                    session.eng.render.resolution_y as f32 * 0.9,
                    &session.eng,
                    2.0,
                );
            },
            Graphics => {
                viewport.pos.y = smooth_value_towards(
                    viewport.pos.y,
                    session.eng.render.resolution_y as f32 * 0.9,
                    &session.eng,
                    2.0,
                );
            },
        }
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

pub fn handle_input(session: &mut GameSession) {
    if session.returning_to_game {
        return;
    }

    let start_pressed = super::control::start_pressed(session);
    if session.action_cooldown <= 0.0 && start_pressed {
        match session.submenu {
            Main => {
                session.returning_to_game = true;
            }
            Settings => {
                session.submenu = Main;
            }
            _ => {
                session.submenu = Settings;
            }
        }
        session.action_cooldown = 0.25;
    }

    match session.submenu {
        Main => {
            for mt in &mut session.menu_text{
                mt.implicit_on = false;
            }
            if session.eng.control.gamepad_connected{
                if session.eng.control.get_gamepad_button_state(session.controller_mappings.arrows[1] as u8) && session.gamepadsel > 0 && session.action_cooldown <= 0.0{
                    session.action_cooldown = 0.25;
                    session.gamepadsel -= 1;
                    session.input_type = InputType::Gamepad;
                }else if session.eng.control.get_gamepad_button_state(session.controller_mappings.arrows[0] as u8) && session.gamepadsel < 2 && session.action_cooldown <= 0.0{
                    session.action_cooldown = 0.25;
                    session.gamepadsel += 1;
                    session.input_type = InputType::Gamepad;
                }else if session.eng.control.get_gamepad_button_state(session.controller_mappings.abxy[0] as u8) && session.gamepadsel < 3 && session.action_cooldown <= 0.0{
                    match session.gamepadsel {
                        0 => {
                            session.returning_to_game = true;
                        },
                        1 => {},
                        2 => {
                            session.exit = true;
                        },
                        _ => {}
                    }
                    session.action_cooldown = 0.25;
                    session.input_type = InputType::Gamepad;
                }
                if session.input_type == InputType::Gamepad{
                    if session.gamepadsel > 2{
                        session.gamepadsel = 2;
                    }
                    session.menu_text[session.gamepadsel].implicit_on = true;
                }
            }
        },
        Settings => {},
        Game => {},
        Audio => {},
        Graphics => {},
    }
}