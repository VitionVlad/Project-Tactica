use crate::engine::math::vec2::Vec2;
use crate::engine::{math::vec3::Vec3, physics::physics::PhysicsObject};
use crate::game::SubMenu::{self, Audio, Game, Graphics, Main, Settings};
use crate::game::{do_paper_sliding_sound, do_paper_sound, do_paper2_sound, fit_vec2_to_screen};
use crate::game::{InputType, settings::save_settings, smooth_value_towards};

use super::{GameSession, sync_writing_system_layout};

pub fn updatesubemnumenu(session: &mut GameSession){
    session.lgtext.size.x = 50.0 * session.ui_scale;
    session.lgtext.size.y = 100.0 * session.ui_scale;
    session.lgtext.pos.x = session.menu_text[session.writing_system as usize][0].pos.x;
    session.lgtext.pos.y = session.menu_text[session.writing_system as usize][0].pos.y - 250.0 * session.ui_scale;
    session.lgtext.pos.z = 0.7;
    session.lgtext.draw = true;
    session.lgtext.signal = false;
    session.lgtext.implicit_on = false;
    session.lgtext.per_symbol = false;
    session.lgtext.new_line_symbol = b'|';
    session.lgtext.rotation = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
    session.lgtext.exec(&mut session.eng, "Project|Tatica!");

    session.black_background[24].object.physic_object.scale.x = session.lgtext.size.x*7.0 + 15.0;
    session.black_background[24].object.physic_object.scale.y = session.lgtext.size.y*2.0 + 15.0;
    session.black_background[24].object.physic_object.pos.x = session.lgtext.pos.x - 7.5;
    session.black_background[24].object.physic_object.pos.y = session.lgtext.pos.y - 7.5;
    session.black_background[24].object.physic_object.pos.z = 0.71;
    session.black_background[24].object.draw = true;
    session.black_background[24].object.physic_object.rot = session.lgtext.rotation;
    session.black_background[24].exec(&mut session.eng);

    for i in 0..session.text_content[session.chosen_language].pause.len(){
        session.menu_text[session.writing_system as usize][i].size.x = 25.0 * session.ui_scale;
        session.menu_text[session.writing_system as usize][i].size.y = 50.0 * session.ui_scale;
        session.menu_text[session.writing_system as usize][i].pos.x = session.black_background[0].object.physic_object.pos.x - menu_page_offset(session.eng.render.resolution_x as f32, session.eng.render.resolution_y as f32) + session.menu_text[session.writing_system as usize][i].size.x*2.0;
        session.menu_text[session.writing_system as usize][i].pos.y = session.eng.render.resolution_y as f32 / 2.0 + (session.menu_text[session.writing_system as usize][i].size.y + 30.0)*(i as f32) + session.viewport.object.physic_object.pos.y;
        session.menu_text[session.writing_system as usize][i].pos.z = 0.7;
        session.menu_text[session.writing_system as usize][i].draw = true;
        session.menu_text[session.writing_system as usize][i].signal = session.submenu == Main;
        session.menu_text[session.writing_system as usize][i].per_symbol = false;
        session.menu_text[session.writing_system as usize][i].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
        let hovered = session.menu_text[session.writing_system as usize][i].exec(&mut session.eng, &session.text_content[session.chosen_language].main_menu[i]);
        super::do_paper_writing_sound_on_hover(session, hovered, session.submenu == Main, i);
        if hovered && session.submenu == Main && session.eng.control.mousebtn[2] && session.action_cooldown <= 0.0{
            match i {
                0 => {
                    do_paper_sliding_sound(session);
                    session.submenu = SubMenu::Comic(0);
                    //session.returning_to_game = true;
                }
                1 => {
                    session.submenu = SubMenu::Settings;
                }
                2 => {
                    session.exit = true;
                }
                _ => {}
            }
            do_paper_sound(session);
            session.input_type = InputType::Touchscreen;
            session.action_cooldown = 0.25;
        }

        session.black_background[i+1].object.physic_object.scale.x = session.menu_text[session.writing_system as usize][i].size.x*(session.text_content[session.chosen_language].main_menu[i].len() as f32)+15.0;
        session.black_background[i+1].object.physic_object.scale.y = session.menu_text[session.writing_system as usize][i].size.y+15.0;
        session.black_background[i+1].object.physic_object.pos.x = session.menu_text[session.writing_system as usize][i].pos.x-7.5;
        session.black_background[i+1].object.physic_object.pos.y = session.menu_text[session.writing_system as usize][i].pos.y-7.5;
        session.black_background[i+1].object.physic_object.pos.z = 0.71;
        session.black_background[i+1].object.draw = true;
        session.black_background[i+1].object.physic_object.rot = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
        session.black_background[i+1].exec(&mut session.eng);
    }
}

pub fn updatesubemnusettings(session: &mut GameSession){
    session.menu_text[session.writing_system as usize][3].size.x = 40.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][3].size.y = 80.0 * session.ui_scale;
    if session.submenu != Settings && session.submenu != Main{
        session.menu_text[session.writing_system as usize][3].pos.x = smooth_value_towards(session.menu_text[session.writing_system as usize][3].pos.x, session.menu_text[session.writing_system as usize][0].pos.x + session.eng.render.resolution_x as f32*0.9, &session.eng, 2.0);
    }else{
        session.menu_text[session.writing_system as usize][3].pos.x = smooth_value_towards(session.menu_text[session.writing_system as usize][3].pos.x, session.menu_text[session.writing_system as usize][0].pos.x, &session.eng, 2.0);
    }
    session.menu_text[session.writing_system as usize][3].pos.y = session.viewport.object.physic_object.pos.y -(session.eng.render.resolution_y as f32 * 0.6) - (session.menu_text[session.writing_system as usize][3].size.y + 30.0);
    session.menu_text[session.writing_system as usize][3].pos.z = 0.7;
    session.menu_text[session.writing_system as usize][3].draw = true;
    session.menu_text[session.writing_system as usize][3].signal = false;
    session.menu_text[session.writing_system as usize][3].per_symbol = false;
    session.menu_text[session.writing_system as usize][3].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
    session.menu_text[session.writing_system as usize][3].exec(&mut session.eng, &session.text_content[session.chosen_language].pause[1]);

    session.black_background[4].object.physic_object.scale.x = session.menu_text[session.writing_system as usize][3].size.x*(session.text_content[session.chosen_language].pause[1].len() as f32)+15.0;
    session.black_background[4].object.physic_object.scale.y = session.menu_text[session.writing_system as usize][3].size.y+15.0;
    session.black_background[4].object.physic_object.pos.x = session.menu_text[session.writing_system as usize][3].pos.x-7.5;
    session.black_background[4].object.physic_object.pos.y = session.menu_text[session.writing_system as usize][3].pos.y-7.5;
    session.black_background[4].object.physic_object.pos.z = 0.71;
    session.black_background[4].object.draw = true;
    session.black_background[4].object.physic_object.rot = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
    session.black_background[4].exec(&mut session.eng);

    for i in 4..(session.text_content[session.chosen_language].settings.len()+4){
        session.menu_text[session.writing_system as usize][i].size.x = 20.0 * session.ui_scale;
        session.menu_text[session.writing_system as usize][i].size.y = 40.0 * session.ui_scale;
        if session.submenu != Settings && session.submenu != Main{
            session.menu_text[session.writing_system as usize][i].pos.x = smooth_value_towards(session.menu_text[session.writing_system as usize][i].pos.x, session.menu_text[session.writing_system as usize][0].pos.x + session.eng.render.resolution_x as f32*0.9, &session.eng, 2.0);
        }else{
            session.menu_text[session.writing_system as usize][i].pos.x = smooth_value_towards(session.menu_text[session.writing_system as usize][i].pos.x, session.menu_text[session.writing_system as usize][0].pos.x, &session.eng, 2.0);
        }
        session.menu_text[session.writing_system as usize][i].pos.y = session.viewport.object.physic_object.pos.y -(session.eng.render.resolution_y as f32 * 0.6) + (session.menu_text[session.writing_system as usize][i].size.y + 30.0)*(i as f32 - 4.0);
        session.menu_text[session.writing_system as usize][i].pos.z = 0.7;
        session.menu_text[session.writing_system as usize][i].draw = true;
        session.menu_text[session.writing_system as usize][i].signal = session.submenu == Settings;
        session.menu_text[session.writing_system as usize][i].per_symbol = false;
        session.menu_text[session.writing_system as usize][i].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
        let hovered = session.menu_text[session.writing_system as usize][i].exec(&mut session.eng, &session.text_content[session.chosen_language].settings[i-4]);
        super::do_paper_writing_sound_on_hover(session, hovered, session.submenu == Settings, i);
        if hovered && session.submenu == Settings && session.eng.control.mousebtn[2] && session.action_cooldown <= 0.0{
            match i {
                4 => {
                    session.submenu = SubMenu::Game;
                    do_paper_sound(session);
                }
                5 => {
                    session.submenu = SubMenu::Audio;
                    do_paper_sound(session);
                }
                6 => {
                    session.submenu = SubMenu::Graphics;
                    do_paper_sound(session);
                }
                7 => {
                    session.submenu = SubMenu::Main;
                    do_paper2_sound(session);
                }
                _ => {}
            }
            session.input_type = InputType::Touchscreen;
            session.action_cooldown = 0.25;
        }

        session.black_background[i+1].object.physic_object.scale.x = session.menu_text[session.writing_system as usize][i].size.x*(session.text_content[session.chosen_language].settings[i-4].len() as f32)+15.0;
        session.black_background[i+1].object.physic_object.scale.y = session.menu_text[session.writing_system as usize][i].size.y+15.0;
        session.black_background[i+1].object.physic_object.pos.x = session.menu_text[session.writing_system as usize][i].pos.x-7.5;
        session.black_background[i+1].object.physic_object.pos.y = session.menu_text[session.writing_system as usize][i].pos.y-7.5;
        session.black_background[i+1].object.physic_object.pos.z = 0.71;
        session.black_background[i+1].object.draw = true;
        session.black_background[i+1].object.physic_object.rot = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
        session.black_background[i+1].exec(&mut session.eng);
    }
}

pub fn updatesubemnugame(session: &mut GameSession){
    session.menu_text[session.writing_system as usize][8].size.x = 40.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][8].size.y = 80.0 * session.ui_scale;
    if session.submenu == Game{
        session.menu_text[session.writing_system as usize][8].pos.x = smooth_value_towards(session.menu_text[session.writing_system as usize][8].pos.x, session.menu_text[session.writing_system as usize][0].pos.x, &session.eng, 2.0);
    }else{
        session.menu_text[session.writing_system as usize][8].pos.x = smooth_value_towards(session.menu_text[session.writing_system as usize][8].pos.x, session.menu_text[session.writing_system as usize][0].pos.x - 1000.0*session.ui_scale, &session.eng, 2.0);
    }
    session.menu_text[session.writing_system as usize][8].pos.y = session.viewport.object.physic_object.pos.y -(session.eng.render.resolution_y as f32 * 0.6) - (session.menu_text[session.writing_system as usize][3].size.y + 30.0);
    session.menu_text[session.writing_system as usize][8].pos.z = 0.7;
    session.menu_text[session.writing_system as usize][8].draw = true;
    session.menu_text[session.writing_system as usize][8].signal = false;
    session.menu_text[session.writing_system as usize][8].per_symbol = false;
    session.menu_text[session.writing_system as usize][8].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
    session.menu_text[session.writing_system as usize][8].exec(&mut session.eng, &session.text_content[session.chosen_language].settings[0]);

    session.black_background[9].object.physic_object.scale.x = session.menu_text[session.writing_system as usize][8].size.x*(session.text_content[session.chosen_language].settings[0].len() as f32)+15.0;
    session.black_background[9].object.physic_object.scale.y = session.menu_text[session.writing_system as usize][8].size.y+15.0;
    session.black_background[9].object.physic_object.pos.x = session.menu_text[session.writing_system as usize][8].pos.x-7.5;
    session.black_background[9].object.physic_object.pos.y = session.menu_text[session.writing_system as usize][8].pos.y-7.5;
    session.black_background[9].object.physic_object.pos.z = 0.71;
    session.black_background[9].object.draw = true;
    session.black_background[9].object.physic_object.rot = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
    session.black_background[9].exec(&mut session.eng);

    for i in 9..(session.text_content[session.chosen_language].game_set.len()+7){
        session.menu_text[session.writing_system as usize][i].size.x = 20.0 * session.ui_scale;
        session.menu_text[session.writing_system as usize][i].size.y = 40.0 * session.ui_scale;
        if session.submenu == Game{
            session.menu_text[session.writing_system as usize][i].pos.x = smooth_value_towards(session.menu_text[session.writing_system as usize][i].pos.x, session.menu_text[session.writing_system as usize][0].pos.x, &session.eng, 2.0);
        }else{
            session.menu_text[session.writing_system as usize][i].pos.x = smooth_value_towards(session.menu_text[session.writing_system as usize][i].pos.x, session.menu_text[session.writing_system as usize][0].pos.x - 1000.0*session.ui_scale, &session.eng, 2.0);
        }
        session.menu_text[session.writing_system as usize][i].pos.y = session.viewport.object.physic_object.pos.y -(session.eng.render.resolution_y as f32 * 0.6) + (session.menu_text[session.writing_system as usize][i].size.y + 30.0)*(i as f32 - 9.0);
        session.menu_text[session.writing_system as usize][i].pos.z = 0.7;
        session.menu_text[session.writing_system as usize][i].draw = true;
        session.menu_text[session.writing_system as usize][i].signal = session.submenu == Game;
        session.menu_text[session.writing_system as usize][i].per_symbol = false;
        session.menu_text[session.writing_system as usize][i].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
        let mut txt = session.text_content[session.chosen_language].game_set[i-9].clone();
        match i {
                9 => {
                    txt += &session.text_content[session.chosen_language].language_name;
                    txt += " ";
                },
                10 => {
                    txt += &session.ui_scale.to_string();
                    txt += " ";
                },
                11 => {
                    match session.show_dbg_stat {
                        true => {
                            txt += &session.text_content[session.chosen_language].game_set[4].clone();
                        },
                        false => {
                            txt += &session.text_content[session.chosen_language].game_set[5].clone();
                        },
                    }
                    //session.returning_to_game = true;
                }
                _ => {}
            }
        let hovered = session.menu_text[session.writing_system as usize][i].exec(&mut session.eng, &txt);
        super::do_paper_writing_sound_on_hover(session, hovered, session.submenu == Game, i);
        if hovered && session.submenu == Game && session.eng.control.mousebtn[2] && session.action_cooldown <= 0.0{
            match i {
                9 => {
                    select_next_language(session);
                    do_paper_sound(session);
                    save_settings(session);
                },
                10 => {
                    //session.returning_to_game = true;
                    session.ui_scale = ((session.ui_scale*10.0) as i32 + 1) as f32 / 10.0;
                    if session.ui_scale > 1.5{
                        session.ui_scale = 0.5;
                    }
                    save_settings(session);
                    do_paper_sound(session);
                },
                11 => {
                    //session.returning_to_game = true;
                    session.show_dbg_stat = !session.show_dbg_stat;
                    save_settings(session);
                    do_paper_sound(session);
                }
                12 => {
                    session.submenu = SubMenu::Settings;
                    do_paper2_sound(session);
                }
                _ => {}
            }
            session.input_type = InputType::Touchscreen;
            session.action_cooldown = 0.25;
        }

        session.black_background[i+1].object.physic_object.scale.x = session.menu_text[session.writing_system as usize][i].size.x*(txt.len() as f32)+15.0;
        session.black_background[i+1].object.physic_object.scale.y = session.menu_text[session.writing_system as usize][i].size.y+15.0;
        session.black_background[i+1].object.physic_object.pos.x = session.menu_text[session.writing_system as usize][i].pos.x-7.5;
        session.black_background[i+1].object.physic_object.pos.y = session.menu_text[session.writing_system as usize][i].pos.y-7.5;
        session.black_background[i+1].object.physic_object.pos.z = 0.71;
        session.black_background[i+1].object.draw = true;
        session.black_background[i+1].object.physic_object.rot = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
        session.black_background[i+1].exec(&mut session.eng);
    }
}

pub fn updatesubemnuaudio(session: &mut GameSession){
    session.menu_text[session.writing_system as usize][13].size.x = 40.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][13].size.y = 80.0 * session.ui_scale;
    if session.submenu == Audio{
        session.menu_text[session.writing_system as usize][13].pos.x = smooth_value_towards(session.menu_text[session.writing_system as usize][13].pos.x, session.menu_text[session.writing_system as usize][0].pos.x, &session.eng, 2.0);
    }else{
        session.menu_text[session.writing_system as usize][13].pos.x = smooth_value_towards(session.menu_text[session.writing_system as usize][13].pos.x, session.menu_text[session.writing_system as usize][0].pos.x - 1000.0*session.ui_scale, &session.eng, 2.0);
    }
    session.menu_text[session.writing_system as usize][13].pos.y = session.viewport.object.physic_object.pos.y -(session.eng.render.resolution_y as f32 * 0.6) - (session.menu_text[session.writing_system as usize][3].size.y + 30.0);
    session.menu_text[session.writing_system as usize][13].pos.z = 0.7;
    session.menu_text[session.writing_system as usize][13].draw = true;
    session.menu_text[session.writing_system as usize][13].signal = false;
    session.menu_text[session.writing_system as usize][13].per_symbol = false;
    session.menu_text[session.writing_system as usize][13].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
    session.menu_text[session.writing_system as usize][13].exec(&mut session.eng, &session.text_content[session.chosen_language].settings[1]);

    session.black_background[14].object.physic_object.scale.x = session.menu_text[session.writing_system as usize][13].size.x*(session.text_content[session.chosen_language].settings[1].len() as f32)+15.0;
    session.black_background[14].object.physic_object.scale.y = session.menu_text[session.writing_system as usize][13].size.y+15.0;
    session.black_background[14].object.physic_object.pos.x = session.menu_text[session.writing_system as usize][13].pos.x-7.5;
    session.black_background[14].object.physic_object.pos.y = session.menu_text[session.writing_system as usize][13].pos.y-7.5;
    session.black_background[14].object.physic_object.pos.z = 0.71;
    session.black_background[14].object.draw = true;
    session.black_background[14].object.physic_object.rot = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
    session.black_background[14].exec(&mut session.eng);

    for i in 14..(session.text_content[session.chosen_language].audio.len()+14){
        session.menu_text[session.writing_system as usize][i].size.x = 20.0 * session.ui_scale;
        session.menu_text[session.writing_system as usize][i].size.y = 40.0 * session.ui_scale;
        if session.submenu == Audio{
            session.menu_text[session.writing_system as usize][i].pos.x = smooth_value_towards(session.menu_text[session.writing_system as usize][i].pos.x, session.menu_text[session.writing_system as usize][0].pos.x, &session.eng, 2.0);
        }else{
            session.menu_text[session.writing_system as usize][i].pos.x = smooth_value_towards(session.menu_text[session.writing_system as usize][i].pos.x, session.menu_text[session.writing_system as usize][0].pos.x - 1000.0*session.ui_scale, &session.eng, 2.0);
        }
        session.menu_text[session.writing_system as usize][i].pos.y = session.viewport.object.physic_object.pos.y -(session.eng.render.resolution_y as f32 * 0.6) + (session.menu_text[session.writing_system as usize][i].size.y + 30.0)*(i as f32 - 14.0);
        session.menu_text[session.writing_system as usize][i].pos.z = 0.7;
        session.menu_text[session.writing_system as usize][i].draw = true;
        session.menu_text[session.writing_system as usize][i].signal = session.submenu == Audio;
        session.menu_text[session.writing_system as usize][i].per_symbol = false;
        session.menu_text[session.writing_system as usize][i].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
        let mut txt = session.text_content[session.chosen_language].audio[i-14].clone();
        if i == 14{
            txt += &session.eng.audio.vol.to_string();
            txt += " ";
        }
        let hovered = session.menu_text[session.writing_system as usize][i].exec(&mut session.eng, &txt);
        super::do_paper_writing_sound_on_hover(session, hovered, session.submenu == Audio, i);
        if hovered && session.submenu == Audio && session.eng.control.mousebtn[2] && session.action_cooldown <= 0.0{
            match i {
                14 => {
                    session.eng.audio.vol = ((session.eng.audio.vol*10.0) as i32 + 1) as f32 / 10.0;
                    if session.eng.audio.vol > 1.0{
                        session.eng.audio.vol = 0.0;
                    }
                    save_settings(session);
                    do_paper_sound(session);
                },
                15 => {
                    session.submenu = SubMenu::Settings;
                    do_paper2_sound(session);
                }
                _ => {}
            }
            session.input_type = InputType::Touchscreen;
            session.action_cooldown = 0.25;
        }

        session.black_background[i+1].object.physic_object.scale.x = session.menu_text[session.writing_system as usize][i].size.x*(txt.len() as f32)+15.0;
        session.black_background[i+1].object.physic_object.scale.y = session.menu_text[session.writing_system as usize][i].size.y+15.0;
        session.black_background[i+1].object.physic_object.pos.x = session.menu_text[session.writing_system as usize][i].pos.x-7.5;
        session.black_background[i+1].object.physic_object.pos.y = session.menu_text[session.writing_system as usize][i].pos.y-7.5;
        session.black_background[i+1].object.physic_object.pos.z = 0.71;
        session.black_background[i+1].object.draw = true;
        session.black_background[i+1].object.physic_object.rot = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
        session.black_background[i+1].exec(&mut session.eng);
    }
}

pub fn updatesubemnugraphics(session: &mut GameSession){
    session.menu_text[session.writing_system as usize][16].size.x = 40.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][16].size.y = 80.0 * session.ui_scale;
    if session.submenu == Graphics{
        session.menu_text[session.writing_system as usize][16].pos.x = smooth_value_towards(session.menu_text[session.writing_system as usize][16].pos.x, session.menu_text[session.writing_system as usize][0].pos.x, &session.eng, 2.0);
    }else{
        session.menu_text[session.writing_system as usize][16].pos.x = smooth_value_towards(session.menu_text[session.writing_system as usize][16].pos.x, session.menu_text[session.writing_system as usize][0].pos.x - 1000.0*session.ui_scale, &session.eng, 2.0);
    }
    session.menu_text[session.writing_system as usize][16].pos.y = session.viewport.object.physic_object.pos.y -(session.eng.render.resolution_y as f32 * 0.675) - (session.menu_text[session.writing_system as usize][3].size.y + 30.0);
    session.menu_text[session.writing_system as usize][16].pos.z = 0.7;
    session.menu_text[session.writing_system as usize][16].draw = true;
    session.menu_text[session.writing_system as usize][16].signal = false;
    session.menu_text[session.writing_system as usize][16].per_symbol = false;
    session.menu_text[session.writing_system as usize][16].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
    session.menu_text[session.writing_system as usize][16].exec(&mut session.eng, &session.text_content[session.chosen_language].settings[2]);

    session.black_background[17].object.physic_object.scale.x = session.menu_text[session.writing_system as usize][16].size.x*(session.text_content[session.chosen_language].settings[2].len() as f32)+15.0;
    session.black_background[17].object.physic_object.scale.y = session.menu_text[session.writing_system as usize][16].size.y+15.0;
    session.black_background[17].object.physic_object.pos.x = session.menu_text[session.writing_system as usize][16].pos.x-7.5;
    session.black_background[17].object.physic_object.pos.y = session.menu_text[session.writing_system as usize][16].pos.y-7.5;
    session.black_background[17].object.physic_object.pos.z = 0.71;
    session.black_background[17].object.draw = true;
    session.black_background[17].object.physic_object.rot = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
    session.black_background[17].exec(&mut session.eng);

    for i in 17..(session.text_content[session.chosen_language].graphics.len()+10){
        session.menu_text[session.writing_system as usize][i].size.x = 20.0 * session.ui_scale;
        session.menu_text[session.writing_system as usize][i].size.y = 40.0 * session.ui_scale;
        if session.submenu == Graphics{
            session.menu_text[session.writing_system as usize][i].pos.x = smooth_value_towards(session.menu_text[session.writing_system as usize][i].pos.x, session.menu_text[session.writing_system as usize][0].pos.x, &session.eng, 2.0);
        }else{
            session.menu_text[session.writing_system as usize][i].pos.x = smooth_value_towards(session.menu_text[session.writing_system as usize][i].pos.x, session.menu_text[session.writing_system as usize][0].pos.x - 1000.0*session.ui_scale, &session.eng, 2.0);
        }
        session.menu_text[session.writing_system as usize][i].pos.y = session.viewport.object.physic_object.pos.y -(session.eng.render.resolution_y as f32 * 0.675) + (session.menu_text[session.writing_system as usize][i].size.y + 30.0)*(i as f32 - 17.0);
        session.menu_text[session.writing_system as usize][i].pos.z = 0.7;
        session.menu_text[session.writing_system as usize][i].draw = true;
        session.menu_text[session.writing_system as usize][i].signal = session.submenu == Graphics;
        session.menu_text[session.writing_system as usize][i].per_symbol = false;
        session.menu_text[session.writing_system as usize][i].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
        let mut txt = session.text_content[session.chosen_language].graphics[i-17].clone();
        match i {
                17 => {
                    match session.use_rt_lighting {
                        true => {
                            txt += &session.text_content[session.chosen_language].graphics[12].clone();
                        },
                        false => {
                            txt += &session.text_content[session.chosen_language].graphics[11].clone();
                        },
                    }
                    //session.returning_to_game = true;
                },
                18 => {
                    txt += &session.eng.render.resolution_scale.to_string();
                    txt += " ";
                    //session.submenu = SubMenu::Settings;
                },
                19 => {
                    match session.eng.render.shadow_map_resolution {
                        512 => {
                            txt += &session.text_content[session.chosen_language].graphics[8].clone();
                        },
                        1024 => {
                            txt += &session.text_content[session.chosen_language].graphics[9].clone();
                        },
                        2048 => {
                            txt += &session.text_content[session.chosen_language].graphics[10].clone();
                        },
                        _ => {
                            txt += &session.eng.render.shadow_map_resolution.to_string();
                            txt += " ";
                        }
                    }
                },
                20 => {
                    match session.eng.render.fullscreen {
                        true => {
                            txt += &session.text_content[session.chosen_language].graphics[6].clone();
                        },
                        false => {
                            txt += &session.text_content[session.chosen_language].graphics[7].clone();
                        },
                    }
                },
                21 => {
                    match session.eng.render.vsync {
                        true => {
                            txt += &session.text_content[session.chosen_language].graphics[6].clone();
                        },
                        false => {
                            txt += &session.text_content[session.chosen_language].graphics[7].clone();
                        },
                    }
                },
                _ => {}
            }
        let hovered = session.menu_text[session.writing_system as usize][i].exec(&mut session.eng, &txt);
        super::do_paper_writing_sound_on_hover(session, hovered, session.submenu == Graphics, i);
        if hovered && session.submenu == Graphics && session.eng.control.mousebtn[2] && session.action_cooldown <= 0.0{
            match i {
                17 => {
                    session.use_rt_lighting = !session.use_rt_lighting;
                    session.tredr = ((1.0 / session.eng.gpu_fps as f64)*2.0).max(0.1);
                    save_settings(session);
                    do_paper_sound(session);
                },
                18 => {
                    session.eng.render.resolution_scale = ((session.eng.render.resolution_scale*10.0) as i32 + 1) as f32 / 10.0;
                    if session.eng.render.resolution_scale > 1.0{
                        session.eng.render.resolution_scale = 0.1;
                    }
                    session.tredr = ((1.0 / session.eng.gpu_fps as f64)*2.0).max(0.1);
                    save_settings(session);
                    do_paper_sound(session);
                },
                19 => {
                    match session.eng.render.shadow_map_resolution {
                        512 => {
                            session.eng.render.shadow_map_resolution = 1024;
                        },
                        1024 => {
                            session.eng.render.shadow_map_resolution = 2048;
                        },
                        2048 => {
                            session.eng.render.shadow_map_resolution = 512;
                        },
                        _ => { session.eng.render.shadow_map_resolution = 512; }
                    }
                    session.tredr = ((1.0 / session.eng.gpu_fps as f64)*2.0).max(0.1);
                    save_settings(session);
                    do_paper_sound(session);
                },
                20 => {
                    session.eng.render.fullscreen = !session.eng.render.fullscreen;
                    save_settings(session);
                    do_paper_sound(session);
                },
                21 => {
                    session.eng.render.vsync = !session.eng.render.vsync;
                    save_settings(session);
                    do_paper_sound(session);
                },
                22 => {
                    session.submenu = SubMenu::Settings;
                    do_paper2_sound(session);
                }
                _ => {}
            }
            session.input_type = InputType::Touchscreen;
            session.action_cooldown = 0.25;
        }

        session.black_background[i+1].object.physic_object.scale.x = session.menu_text[session.writing_system as usize][i].size.x*(txt.len() as f32)+15.0;
        session.black_background[i+1].object.physic_object.scale.y = session.menu_text[session.writing_system as usize][i].size.y+15.0;
        session.black_background[i+1].object.physic_object.pos.x = session.menu_text[session.writing_system as usize][i].pos.x-7.5;
        session.black_background[i+1].object.physic_object.pos.y = session.menu_text[session.writing_system as usize][i].pos.y-7.5;
        session.black_background[i+1].object.physic_object.pos.z = 0.71;
        session.black_background[i+1].object.draw = true;
        session.black_background[i+1].object.physic_object.rot = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
        session.black_background[i+1].exec(&mut session.eng);
    }
}

fn comic_panel_widths(screen_height: f32) -> [f32; 3] {
    [
        screen_height * (16.0 / 9.0),
        screen_height * (4.0 / 9.0),
        screen_height * (4.0 / 9.0),
    ]
}

fn menu_page_offset(screen_width: f32, screen_height: f32) -> f32 {
    comic_panel_widths(screen_height).iter().sum::<f32>() + screen_width + 120.0
}

fn comic_panel_layout(viewport_x: f32, screen_height: f32) -> ([f32; 3], [f32; 3]) {
    let panel_widths = comic_panel_widths(screen_height);
    let panel_positions = [
        viewport_x - panel_widths[0] - panel_widths[1] - panel_widths[2] - 90.0,
        viewport_x - panel_widths[1] - panel_widths[2] - 60.0,
        viewport_x - panel_widths[2] - 30.0,
    ];
    (panel_widths, panel_positions)
}

fn comic_slide_viewport_x(panel_widths: [f32; 3], screen_width: f32, slide: u8) -> f32 {
    match slide {
        0 => screen_width + panel_widths[1] + panel_widths[2] + 90.0,
        1 => screen_width + panel_widths[2] + 60.0,
        _ => screen_width + 30.0,
    }
}

pub fn updatecomic(session: &mut GameSession){
    let screen_height = session.eng.render.resolution_y as f32;
    let (panel_widths, panel_positions) = comic_panel_layout(
        session.viewport.object.physic_object.pos.x,
        screen_height,
    );
    for i in 0..session.comic_images.len() {
        let panel = &mut session.comic_images[i];
        panel.object.physic_object.scale.x = panel_widths[i];
        panel.object.physic_object.scale.y = screen_height;
        panel.object.physic_object.pos.x = panel_positions[i];
        panel.object.physic_object.pos.y = session.viewport.object.physic_object.pos.y;
        panel.object.physic_object.pos.z = 0.99;
        panel.object.draw = true;
        panel.exec(&mut session.eng);
    }

    //21, 22

    let text = session.text_content[session.writing_system as usize].comic[0].clone();
    session.menu_text[session.writing_system as usize][23].size.x = 20.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][23].size.y = 40.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][23].pos.x = session.comic_images[0].object.physic_object.pos.x + session.comic_images[0].object.physic_object.scale.x - 60.0 - text.len() as f32*session.menu_text[session.writing_system as usize][23].size.x;
    session.menu_text[session.writing_system as usize][23].pos.y = session.eng.render.resolution_y as f32 - 60.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][23].pos.z = 0.7;
    session.menu_text[session.writing_system as usize][23].signal = false;
    if session.submenu == SubMenu::Comic(0){
        session.menu_text[session.writing_system as usize][23].signal = true;
    }
    session.menu_text[session.writing_system as usize][23].draw = true;
    session.menu_text[session.writing_system as usize][23].per_symbol = false;
    if session.menu_text[session.writing_system as usize][23].exec(&mut session.eng, &text) && session.eng.control.mousebtn[2]{
        session.submenu = SubMenu::Comic(1);
    }

    session.black_background[25].object.physic_object.scale.x = session.menu_text[session.writing_system as usize][23].size.x*(text.len() as f32)+15.0;
    session.black_background[25].object.physic_object.scale.y = session.menu_text[session.writing_system as usize][23].size.y+15.0;
    session.black_background[25].object.physic_object.pos.x = session.menu_text[session.writing_system as usize][23].pos.x-7.5;
    session.black_background[25].object.physic_object.pos.y = session.menu_text[session.writing_system as usize][23].pos.y-7.5;
    session.black_background[25].object.physic_object.pos.z = 0.71;
    session.black_background[25].object.draw = true;
    session.black_background[25].exec(&mut session.eng);

    let ctext = session.text_content[session.writing_system as usize].comic[1].clone();
    let lg = ctext.len() as f32/2.0-1.0;
    session.menu_text[session.writing_system as usize][26].size.x = 20.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][26].size.y = 40.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][26].pos.x = session.comic_images[0].object.physic_object.pos.x + session.comic_images[0].object.physic_object.scale.x - 30.0 * session.ui_scale - lg*session.menu_text[session.writing_system as usize][26].size.x;
    session.menu_text[session.writing_system as usize][26].pos.y = 60.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][26].pos.z = 0.7;
    session.menu_text[session.writing_system as usize][26].signal = false;
    session.menu_text[session.writing_system as usize][26].draw = true;
    session.menu_text[session.writing_system as usize][26].per_symbol = false;
    session.menu_text[session.writing_system as usize][26].new_line_symbol = b'|';
    session.menu_text[session.writing_system as usize][26].exec(&mut session.eng, &ctext);

    session.black_background[28].object.physic_object.scale.x = session.menu_text[session.writing_system as usize][26].size.x*(lg as f32)+15.0;
    session.black_background[28].object.physic_object.scale.y = session.menu_text[session.writing_system as usize][26].size.y*2.0+15.0;
    session.black_background[28].object.physic_object.pos.x = session.menu_text[session.writing_system as usize][26].pos.x-7.5;
    session.black_background[28].object.physic_object.pos.y = session.menu_text[session.writing_system as usize][26].pos.y-7.5;
    session.black_background[28].object.physic_object.pos.z = 0.71;
    session.black_background[28].object.draw = true;
    session.black_background[28].exec(&mut session.eng);

    //

    session.menu_text[session.writing_system as usize][24].size.x = 20.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][24].size.y = 40.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][24].pos.x = session.comic_images[1].object.physic_object.pos.x + session.comic_images[1].object.physic_object.scale.x - 60.0 - text.len() as f32*session.menu_text[session.writing_system as usize][24].size.x;
    session.menu_text[session.writing_system as usize][24].pos.y = session.eng.render.resolution_y as f32 - 60.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][24].pos.z = 0.7;
    session.menu_text[session.writing_system as usize][24].signal = false;
    if session.submenu == SubMenu::Comic(1){
        session.menu_text[session.writing_system as usize][24].signal = true;
    }
    session.menu_text[session.writing_system as usize][24].draw = true;
    session.menu_text[session.writing_system as usize][24].per_symbol = false;
    if session.menu_text[session.writing_system as usize][24].exec(&mut session.eng, &text) && session.eng.control.mousebtn[2]{
        session.submenu = SubMenu::Comic(2);
    }

    session.black_background[26].object.physic_object.scale.x = session.menu_text[session.writing_system as usize][24].size.x*(text.len() as f32)+15.0;
    session.black_background[26].object.physic_object.scale.y = session.menu_text[session.writing_system as usize][24].size.y+15.0;
    session.black_background[26].object.physic_object.pos.x = session.menu_text[session.writing_system as usize][24].pos.x-7.5;
    session.black_background[26].object.physic_object.pos.y = session.menu_text[session.writing_system as usize][24].pos.y-7.5;
    session.black_background[26].object.physic_object.pos.z = 0.71;
    session.black_background[26].object.draw = true;
    session.black_background[26].exec(&mut session.eng);

    let ctext = session.text_content[session.writing_system as usize].comic[2].clone();
    let lg = ctext.len() as f32/5.0-1.0;
    session.menu_text[session.writing_system as usize][27].size.x = 10.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][27].size.y = 20.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][27].pos.x = session.comic_images[1].object.physic_object.pos.x + session.comic_images[1].object.physic_object.scale.x - 30.0 * session.ui_scale - lg as f32*session.menu_text[session.writing_system as usize][27].size.x;
    session.menu_text[session.writing_system as usize][27].pos.y = 60.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][27].pos.z = 0.7;
    session.menu_text[session.writing_system as usize][27].signal = false;
    session.menu_text[session.writing_system as usize][27].draw = true;
    session.menu_text[session.writing_system as usize][27].per_symbol = false;
    session.menu_text[session.writing_system as usize][27].new_line_symbol = b'|';
    session.menu_text[session.writing_system as usize][27].exec(&mut session.eng, &ctext);

    session.black_background[29].object.physic_object.scale.x = session.menu_text[session.writing_system as usize][27].size.x*(lg as f32)+15.0;
    session.black_background[29].object.physic_object.scale.y = session.menu_text[session.writing_system as usize][27].size.y*5.0+15.0;
    session.black_background[29].object.physic_object.pos.x = session.menu_text[session.writing_system as usize][27].pos.x-7.5;
    session.black_background[29].object.physic_object.pos.y = session.menu_text[session.writing_system as usize][27].pos.y-7.5;
    session.black_background[29].object.physic_object.pos.z = 0.71;
    session.black_background[29].object.draw = true;
    session.black_background[29].exec(&mut session.eng);

    //

    session.menu_text[session.writing_system as usize][25].size.x = 20.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][25].size.y = 40.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][25].pos.x = session.comic_images[2].object.physic_object.pos.x + session.comic_images[2].object.physic_object.scale.x - 60.0 - text.len() as f32*session.menu_text[session.writing_system as usize][25].size.x;
    session.menu_text[session.writing_system as usize][25].pos.y = session.eng.render.resolution_y as f32 - 60.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][25].pos.z = 0.7;
    session.menu_text[session.writing_system as usize][25].signal = false;
    if session.submenu != SubMenu::Comic(0) && session.submenu != SubMenu::Comic(1){
        session.menu_text[session.writing_system as usize][25].signal = true;
    }
    session.menu_text[session.writing_system as usize][25].draw = true;
    session.menu_text[session.writing_system as usize][25].per_symbol = false;
    if session.menu_text[session.writing_system as usize][25].exec(&mut session.eng, &text) && session.eng.control.mousebtn[2]{
        session.submenu = SubMenu::Main;
        session.returning_to_game = true;
    }

    session.black_background[27].object.physic_object.scale.x = session.menu_text[session.writing_system as usize][25].size.x*(text.len() as f32)+15.0;
    session.black_background[27].object.physic_object.scale.y = session.menu_text[session.writing_system as usize][25].size.y+15.0;
    session.black_background[27].object.physic_object.pos.x = session.menu_text[session.writing_system as usize][25].pos.x-7.5;
    session.black_background[27].object.physic_object.pos.y = session.menu_text[session.writing_system as usize][25].pos.y-7.5;
    session.black_background[27].object.physic_object.pos.z = 0.71;
    session.black_background[27].object.draw = true;
    session.black_background[27].exec(&mut session.eng);

    let ctext = session.text_content[session.writing_system as usize].comic[3].clone();
    let lg = ctext.len() as f32/5.0-1.0;
    session.menu_text[session.writing_system as usize][28].size.x = 10.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][28].size.y = 20.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][28].pos.x = session.comic_images[2].object.physic_object.pos.x + session.comic_images[2].object.physic_object.scale.x - 30.0 * session.ui_scale - lg*session.menu_text[session.writing_system as usize][28].size.x;
    session.menu_text[session.writing_system as usize][28].pos.y = 60.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][28].pos.z = 0.7;
    session.menu_text[session.writing_system as usize][28].signal = false;
    session.menu_text[session.writing_system as usize][28].draw = true;
    session.menu_text[session.writing_system as usize][28].per_symbol = false;
    session.menu_text[session.writing_system as usize][28].new_line_symbol = b'|';
    session.menu_text[session.writing_system as usize][28].exec(&mut session.eng, &ctext);

    session.black_background[30].object.physic_object.scale.x = session.menu_text[session.writing_system as usize][28].size.x*(lg as f32)+15.0;
    session.black_background[30].object.physic_object.scale.y = session.menu_text[session.writing_system as usize][28].size.y*5.0+15.0;
    session.black_background[30].object.physic_object.pos.x = session.menu_text[session.writing_system as usize][28].pos.x-7.5;
    session.black_background[30].object.physic_object.pos.y = session.menu_text[session.writing_system as usize][28].pos.y-7.5;
    session.black_background[30].object.physic_object.pos.z = 0.71;
    session.black_background[30].object.draw = true;
    session.black_background[30].exec(&mut session.eng);

    let ctext = session.text_content[session.writing_system as usize].comic[4].clone();
    let lg = ctext.len() as f32/4.0-1.0;
    session.menu_text[session.writing_system as usize][29].size.x = 10.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][29].size.y = 20.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][29].pos.x = session.comic_images[2].object.physic_object.pos.x + session.comic_images[2].object.physic_object.scale.x - 30.0 * session.ui_scale - lg*session.menu_text[session.writing_system as usize][29].size.x;
    session.menu_text[session.writing_system as usize][29].pos.y = 90.0 * session.ui_scale + session.menu_text[session.writing_system as usize][28].size.y*5.0;
    session.menu_text[session.writing_system as usize][29].pos.z = 0.7;
    session.menu_text[session.writing_system as usize][29].signal = false;
    session.menu_text[session.writing_system as usize][29].draw = true;
    session.menu_text[session.writing_system as usize][29].per_symbol = false;
    session.menu_text[session.writing_system as usize][29].new_line_symbol = b'|';
    session.menu_text[session.writing_system as usize][29].exec(&mut session.eng, &ctext);

    session.black_background[31].object.physic_object.scale.x = session.menu_text[session.writing_system as usize][29].size.x*(lg as f32)+15.0;
    session.black_background[31].object.physic_object.scale.y = session.menu_text[session.writing_system as usize][29].size.y*4.0+15.0;
    session.black_background[31].object.physic_object.pos.x = session.menu_text[session.writing_system as usize][29].pos.x-7.5;
    session.black_background[31].object.physic_object.pos.y = session.menu_text[session.writing_system as usize][29].pos.y-7.5;
    session.black_background[31].object.physic_object.pos.z = 0.71;
    session.black_background[31].object.draw = true;
    session.black_background[31].exec(&mut session.eng);

    let ctext = session.text_content[session.writing_system as usize].comic[5].clone();
    let lg = ctext.len() as f32;
    session.menu_text[session.writing_system as usize][30].size.x = 10.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][30].size.y = 20.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][30].pos.x = session.comic_images[2].object.physic_object.pos.x + 30.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][30].pos.y = 120.0 * session.ui_scale + session.menu_text[session.writing_system as usize][28].size.y*9.0;
    session.menu_text[session.writing_system as usize][30].pos.z = 0.7;
    session.menu_text[session.writing_system as usize][30].signal = false;
    session.menu_text[session.writing_system as usize][30].draw = true;
    session.menu_text[session.writing_system as usize][30].per_symbol = false;
    session.menu_text[session.writing_system as usize][30].new_line_symbol = b'|';
    session.menu_text[session.writing_system as usize][30].rotation = Vec3{ x: 0.0, y: 0.0, z: -0.05}.to_quat();
    session.menu_text[session.writing_system as usize][30].exec(&mut session.eng, &ctext);

    session.black_background[32].object.physic_object.scale.x = session.menu_text[session.writing_system as usize][30].size.x*(lg as f32)+15.0;
    session.black_background[32].object.physic_object.scale.y = session.menu_text[session.writing_system as usize][30].size.y+15.0;
    session.black_background[32].object.physic_object.pos.x = session.menu_text[session.writing_system as usize][30].pos.x-7.5;
    session.black_background[32].object.physic_object.pos.y = session.menu_text[session.writing_system as usize][30].pos.y-7.5;
    session.black_background[32].object.physic_object.pos.z = 0.71;
    session.black_background[32].object.physic_object.rot = session.menu_text[session.writing_system as usize][30].rotation;
    session.black_background[32].object.draw = true;
    session.black_background[32].exec(&mut session.eng);

    let ctext = session.text_content[session.writing_system as usize].comic[6].clone();
    let lg = ctext.len() as f32/4.0-1.0;
    session.menu_text[session.writing_system as usize][31].size.x = 10.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][31].size.y = 20.0 * session.ui_scale;
    session.menu_text[session.writing_system as usize][31].pos.x = session.comic_images[2].object.physic_object.pos.x + session.comic_images[2].object.physic_object.scale.x - 30.0 * session.ui_scale - lg*session.menu_text[session.writing_system as usize][31].size.x;
    session.menu_text[session.writing_system as usize][31].pos.y = 150.0 * session.ui_scale + session.menu_text[session.writing_system as usize][28].size.y*10.0;
    session.menu_text[session.writing_system as usize][31].pos.z = 0.7;
    session.menu_text[session.writing_system as usize][31].signal = false;
    session.menu_text[session.writing_system as usize][31].draw = true;
    session.menu_text[session.writing_system as usize][31].per_symbol = false;
    session.menu_text[session.writing_system as usize][31].new_line_symbol = b'|';
    session.menu_text[session.writing_system as usize][31].exec(&mut session.eng, &ctext);

    session.black_background[33].object.physic_object.scale.x = session.menu_text[session.writing_system as usize][31].size.x*(lg as f32)+15.0;
    session.black_background[33].object.physic_object.scale.y = session.menu_text[session.writing_system as usize][31].size.y*4.0+15.0;
    session.black_background[33].object.physic_object.pos.x = session.menu_text[session.writing_system as usize][31].pos.x-7.5;
    session.black_background[33].object.physic_object.pos.y = session.menu_text[session.writing_system as usize][31].pos.y-7.5;
    session.black_background[33].object.physic_object.pos.z = 0.71;
    session.black_background[33].object.draw = true;
    session.black_background[33].exec(&mut session.eng);
}

pub fn update(session: &mut GameSession) {
    session.speakers[3].volume = 0.0;
    for speaker in &mut session.speakers {
        speaker.exec(&mut session.eng);
    }

    if session.show_dbg_stat{
        let fps_text = format!(
            "CPU_cps:{}|GPU_fps:{}|{}x{}",
            session.eng.cpu_fps, session.eng.gpu_fps, session.eng.render.resolution_x, session.eng.render.resolution_y
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

    for text in &mut session.text[session.writing_system as usize] {
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

    session.menu_background[1].object.physic_object.scale.x = session.eng.render.resolution_x as f32;
    session.menu_background[1].object.physic_object.scale.y = session.eng.render.resolution_y as f32;
    session.menu_background[1].object.physic_object.pos.x = session.viewport.object.physic_object.pos.x - menu_page_offset(session.eng.render.resolution_x as f32, session.eng.render.resolution_y as f32);
    session.menu_background[1].object.physic_object.pos.y = session.viewport.object.physic_object.pos.y;
    session.menu_background[1].object.physic_object.pos.z = 0.99;
    session.menu_background[1].object.draw = true;
    session.menu_background[1].object.mesh.ubo[52] = session.current_time as f32;
    session.menu_background[1].object.mesh.ubo[53] = 2000.0;
    session.menu_background[1].object.mesh.ubo[54] = 2000.0;
    session.menu_background[1].object.mesh.ubo[55] = session.frame_count as f32;
    session.menu_background[1].exec(&mut session.eng);

    session.menu_background[0].object.physic_object.scale.x = session.eng.render.resolution_x as f32;
    session.menu_background[0].object.physic_object.scale.y = session.eng.render.resolution_y as f32;
    session.menu_background[0].object.physic_object.pos.x = session.viewport.object.physic_object.pos.x - menu_page_offset(session.eng.render.resolution_x as f32, session.eng.render.resolution_y as f32);
    session.menu_background[0].object.physic_object.pos.y = session.viewport.object.physic_object.pos.y - session.eng.render.resolution_y as f32 - 30.0;
    session.menu_background[0].object.physic_object.pos.z = 0.99;
    session.menu_background[0].object.draw = true;
    session.menu_background[0].object.mesh.ubo[52] = session.current_time as f32;
    session.menu_background[0].object.mesh.ubo[53] = 2000.0;
    session.menu_background[0].object.mesh.ubo[54] = 2000.0;
    session.menu_background[0].object.mesh.ubo[55] = session.frame_count as f32;
    session.menu_background[0].exec(&mut session.eng);

    session.black_background[0].object.physic_object.scale.x = session.viewport.object.physic_object.scale.x + 50.0;
    session.black_background[0].object.physic_object.scale.y = session.viewport.object.physic_object.scale.y + 50.0;
    session.black_background[0].object.physic_object.pos.x = session.viewport.object.physic_object.pos.x - 25.0;
    session.black_background[0].object.physic_object.pos.y = session.viewport.object.physic_object.pos.y - 25.0;
    session.black_background[0].object.physic_object.pos.z = 0.66;
    session.black_background[0].object.draw = true;
    session.black_background[0].object.physic_object.rot = session.viewport.object.physic_object.rot;
    session.black_background[0].exec(&mut session.eng);

    let sizes = [
        fit_vec2_to_screen(Vec2{x: 2151.0, y: 1571.0}, 0.8, &session.eng), //map
        //fit_vec2_to_screen(Vec2{x: 1892.0 / 2.0, y: 804.0 / 2.0}, 0.4, &session.eng), //mark
        fit_vec2_to_screen(Vec2{x: 700.0  / 2.0, y: 1060.0 / 2.0}, 0.4, &session.eng), //passport
        fit_vec2_to_screen(Vec2{x: 757.0  / 2.0, y: 1080.0 / 2.0}, 0.4, &session.eng), //photo
        fit_vec2_to_screen(Vec2{x: 1562.0 / 2.0, y: 1154.0 / 2.0}, 0.4, &session.eng), //post
    ];

    //let sizes = [
    //    Vec2{x: 2151.0, y: 1571.0}, //map
    //    Vec2{x: 1892.0, y: 804.0}, //mark
    //    Vec2{x: 700.0, y: 1060.0}, //passport
    //    Vec2{x: 757.0, y: 1080.0}, //photo
    //    Vec2{x: 1562.0, y: 1154.0}, //post
    //];

    for i in 0..session.main_menu_images.len() {
        let img = &mut session.main_menu_images[i];
        img.object.physic_object.scale.x = sizes[i].x;
        img.object.physic_object.scale.y = sizes[i].y;
        img.object.physic_object.pos.x = session.menu_background[0].object.physic_object.pos.x + (session.eng.render.resolution_x as f32)*0.15 + ((session.eng.render.resolution_x as f32)/7.5)*i as f32;
        img.object.physic_object.pos.y = session.viewport.object.physic_object.pos.y + session.eng.render.resolution_y as f32 * 0.1 + ((session.eng.render.resolution_y as f32)/10.0)*i as f32;
        img.object.physic_object.rot = Vec3{x: 0.0, y: 0.0 as f32, z: -0.15+0.1*i as f32}.to_quat();
        img.object.physic_object.pos.z = 0.85 - 0.01 * i as f32;
        img.object.draw = true;
        img.exec(&mut session.eng);
    }

    updatesubemnumenu(session);
    updatesubemnusettings(session);
    updatesubemnugame(session);
    updatesubemnuaudio(session);
    updatesubemnugraphics(session);
    updatecomic(session);

    if session.returning_to_game {
        //session.viewport.object.mesh.ubo[52] = smooth_value_towards(
        //    session.viewport.object.mesh.ubo[52],
        //    session.bwgcoef,
        //    &session.eng,
        //    2.0,
        //);

        let viewport = &mut session.viewport.object.physic_object;
        viewport.pos.x = smooth_value_towards(viewport.pos.x, 0.0, &session.eng, 2.0);
        viewport.pos.y = smooth_value_towards(viewport.pos.y, 0.0, &session.eng, 2.0);
        viewport.scale.x = session.eng.render.resolution_x as f32;
        viewport.scale.y = session.eng.render.resolution_y as f32;

        //session.viewport.object.mesh.ubo[52] = 10.0;
        session.viewport.object.mesh.ubo[53] = 10.0;

        if viewport_is_normal(
            viewport,
            session.eng.render.resolution_x as f32,
            session.eng.render.resolution_y as f32,
        ) {
            for wbg in &mut session.white_background {
                wbg.object.draw = false;
                wbg.exec(&mut session.eng);
            }
        
            for bbg in &mut session.black_background {
                bbg.object.draw = false;
                bbg.exec(&mut session.eng);
            }
        
            for mbg in &mut session.menu_background {
                mbg.object.draw = false;
                mbg.exec(&mut session.eng);
            }
        
            for txt in &mut session.menu_text[session.writing_system as usize] {
                txt.draw = false;
                txt.exec(&mut session.eng, " ");
            }

            for img in &mut session.main_menu_images {
                img.object.draw = false;
                img.exec(&mut session.eng);
            }
        
            for img in &mut session.comic_images {
                img.object.draw = false;
                img.exec(&mut session.eng);
            }

            session.speakers[6].move_sound_cursor(0.0);
            session.speakers[6].play = true;

            viewport.scale.x = session.eng.render.resolution_x as f32;
            viewport.scale.y = session.eng.render.resolution_y as f32;
            viewport.pos.x = 0.0;
            viewport.pos.y = 0.0;
            viewport.rot = Vec3::new().to_quat();
            session.returning_to_game = false;
            session.game_state = super::GameState::InGame;
        }
    } else {
        //session.viewport.object.mesh.ubo[52] = smooth_value_towards(
        //    session.viewport.object.mesh.ubo[52],
        //    1.0,
        //    &session.eng,
        //    2.0,
        //);
        //session.viewport.object.mesh.ubo[52] = 10.0;
        session.viewport.object.mesh.ubo[53] = 10.0;

        let screen_width = session.eng.render.resolution_x as f32;
        let screen_height = session.eng.render.resolution_y as f32;
        let panel_widths = comic_panel_widths(screen_height);
        let menu_viewport_x = menu_page_offset(screen_width, screen_height);
        let viewport = &mut session.viewport.object.physic_object;
        viewport.scale.x = session.eng.render.resolution_x as f32;
        viewport.scale.y = session.eng.render.resolution_y as f32;
        match session.submenu {
            Main => {
                viewport.pos.x = smooth_value_towards(viewport.pos.x, menu_viewport_x, &session.eng, 2.0);
                viewport.pos.y = smooth_value_towards(
                    viewport.pos.y,
                    0.0,
                    &session.eng,
                    2.0,
                );
            },
            Settings => {
                viewport.pos.x = smooth_value_towards(viewport.pos.x, menu_viewport_x, &session.eng, 2.0);
                viewport.pos.y = smooth_value_towards(
                    viewport.pos.y,
                    session.eng.render.resolution_y as f32 * 0.9,
                    &session.eng,
                    2.0,
                );
            },
            Game => {
                viewport.pos.x = smooth_value_towards(viewport.pos.x, menu_viewport_x, &session.eng, 2.0);
                viewport.pos.y = smooth_value_towards(
                    viewport.pos.y,
                    session.eng.render.resolution_y as f32 * 0.9,
                    &session.eng,
                    2.0,
                );
            },
            Audio => {
                viewport.pos.x = smooth_value_towards(viewport.pos.x, menu_viewport_x, &session.eng, 2.0);
                viewport.pos.y = smooth_value_towards(
                    viewport.pos.y,
                    session.eng.render.resolution_y as f32 * 0.9,
                    &session.eng,
                    2.0,
                );
            },
            Graphics => {
                viewport.pos.x = smooth_value_towards(viewport.pos.x, menu_viewport_x, &session.eng, 2.0);
                viewport.pos.y = smooth_value_towards(
                    viewport.pos.y,
                    session.eng.render.resolution_y as f32 * 0.9,
                    &session.eng,
                    2.0,
                );
            },
            SubMenu::Comic(slide) => {
                viewport.pos.y = smooth_value_towards(
                    viewport.pos.y,
                    0.0,
                    &session.eng,
                    2.0,
                );
                viewport.pos.x = smooth_value_towards(
                    viewport.pos.x,
                    comic_slide_viewport_x(panel_widths, screen_width, slide),
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
    //session.viewport.object.mesh.ubo[52] = 10.0;
    session.viewport.object.mesh.ubo[53] = 10.0;
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
    if session.action_cooldown <= 0.0 && (start_pressed || session.eng.control.get_gamepad_button_state(session.controller_mappings.abxy[1] as u8)) {
        match session.submenu {
            Main => {
                do_paper_sliding_sound(session);
                session.returning_to_game = true;
            },
            Settings => {
                session.submenu = Main;
                session.gamepadsel = 1;
            },
            Game => {
                session.submenu = Settings;
                session.gamepadsel = 4;
            },
            Audio => {
                session.submenu = Settings;
                session.gamepadsel = 5;
            },
            Graphics => {
                session.submenu = Settings;
                session.gamepadsel = 6;
            },
            _ => {},
        }
        do_paper2_sound(session);
        session.action_cooldown = 0.25;
    }

    let (first_item, last_item) = match session.submenu {
        Main => (0, 2),
        Settings => (4, 7),
        Game => (9, session.text_content[session.chosen_language].game_set.len() + 6),
        Audio => (14, session.text_content[session.chosen_language].audio.len() + 13),
        Graphics => (17, session.text_content[session.chosen_language].graphics.len() + 9),
        _ => (0, 2),
    };
    if session.gamepadsel < first_item || session.gamepadsel > last_item {
        session.gamepadsel = first_item;
    }

    for menu_text in &mut session.menu_text[session.writing_system as usize] {
        menu_text.implicit_on = false;
    }

    if session.eng.control.gamepad_connected {
        let move_previous = session
            .eng
            .control
            .get_gamepad_button_state(session.controller_mappings.arrows[1] as u8);
        let move_next = session
            .eng
            .control
            .get_gamepad_button_state(session.controller_mappings.arrows[0] as u8);
        let confirm = session
            .eng
            .control
            .get_gamepad_button_state(session.controller_mappings.abxy[0] as u8);

        if session.action_cooldown <= 0.0 && move_previous && session.gamepadsel > first_item {
            session.gamepadsel -= 1;
            session.action_cooldown = 0.25;
            session.input_type = InputType::Gamepad;
        } else if session.action_cooldown <= 0.0 && move_next && session.gamepadsel < last_item {
            session.gamepadsel += 1;
            session.action_cooldown = 0.25;
            session.input_type = InputType::Gamepad;
        } else if session.action_cooldown <= 0.0 && confirm {
            activate_gamepad_selection(session);
            session.action_cooldown = 0.25;
            session.input_type = InputType::Gamepad;
        }
    }

    if session.input_type == InputType::Gamepad {
        if let Some(selected_item) = session.menu_text[session.writing_system as usize].get_mut(session.gamepadsel) {
            selected_item.implicit_on = true;
        }
    }
}

fn activate_gamepad_selection(session: &mut GameSession) {
    match (session.submenu, session.gamepadsel) {
        (Main, 0) => {
            do_paper_sliding_sound(session);
            session.returning_to_game = true;
        },
        (Main, 1) => {
            session.submenu = Settings;
            session.gamepadsel = 4;
            do_paper_sound(session);
        }
        (Main, 2) => session.exit = true,
        (Settings, 4) => {
            session.submenu = Game;
            session.gamepadsel = 9;
            do_paper_sound(session);
        }
        (Settings, 5) => {
            session.submenu = Audio;
            session.gamepadsel = 14;
            do_paper_sound(session);
        }
        (Settings, 6) => {
            session.submenu = Graphics;
            session.gamepadsel = 17;
            do_paper_sound(session);
        }
        (Settings, 7) => {
            session.submenu = Main;
            session.gamepadsel = 1;
            do_paper2_sound(session);
        }
        (Game, 9) => {
            select_next_language(session);
            do_paper_sound(session);
            save_settings(session);
        }
        (Game, 10) => {
            session.ui_scale = ((session.ui_scale * 10.0) as i32 + 1) as f32 / 10.0;
            if session.ui_scale > 1.5 {
                session.ui_scale = 0.5;
            }
            save_settings(session);
            do_paper_sound(session);
        }
        (Game, 11) => {
            session.show_dbg_stat = !session.show_dbg_stat;
            save_settings(session);
            do_paper_sound(session);
        }
        (Game, 12) => {
            session.submenu = Settings;
            session.gamepadsel = 4;
            do_paper2_sound(session);
        }
        (Audio, 14) => {
            session.eng.audio.vol = ((session.eng.audio.vol * 10.0) as i32 + 1) as f32 / 10.0;
            if session.eng.audio.vol > 1.0 {
                session.eng.audio.vol = 0.0;
            }
            save_settings(session);
            do_paper_sound(session);
        }
        (Audio, 15) => {
            session.submenu = Settings;
            session.gamepadsel = 5;
            do_paper2_sound(session);
        }
        (Graphics, 17) => {
            session.use_rt_lighting = !session.use_rt_lighting;
            session.tredr = ((1.0 / session.eng.gpu_fps as f64) * 2.0).max(0.1);
            save_settings(session);
            do_paper_sound(session);
        }
        (Graphics, 18) => {
            session.eng.render.resolution_scale =
                ((session.eng.render.resolution_scale * 10.0) as i32 + 1) as f32 / 10.0;
            if session.eng.render.resolution_scale > 1.0 {
                session.eng.render.resolution_scale = 0.1;
            }
            save_settings(session);
            session.tredr = ((1.0 / session.eng.gpu_fps as f64) * 2.0).max(0.1);
            do_paper_sound(session);
        }
        (Graphics, 19) => {
            session.eng.render.shadow_map_resolution =
                match session.eng.render.shadow_map_resolution {
                    512 => 1024,
                    1024 => 2048,
                    2048 => 512,
                    resolution => resolution,
                };
            session.tredr = ((1.0 / session.eng.gpu_fps as f64) * 2.0).max(0.1);
            do_paper_sound(session);
        }
        (Graphics, 20) => {
            session.eng.render.fullscreen = !session.eng.render.fullscreen;
            do_paper_sound(session);
        }
        (Graphics, 21) => {
            session.eng.render.vsync = !session.eng.render.vsync;
            save_settings(session);
            do_paper_sound(session);
        }
        (Graphics, 22) => {
            session.submenu = Settings;
            session.gamepadsel = 6;
            do_paper2_sound(session);
        }
        _ => {}
    }
}

fn select_next_language(session: &mut GameSession) {
    let previous_writing_system = session.writing_system as usize;
    for text in &mut session.text[previous_writing_system] {
        text.draw = false;
        text.exec(&mut session.eng, " ");
    }
    for text in &mut session.menu_text[previous_writing_system] {
        text.draw = false;
        text.exec(&mut session.eng, " ");
    }

    session.chosen_language = (session.chosen_language + 1) % session.text_content.len();
    session.writing_system = session.text_content[session.chosen_language].writing_system;
    sync_writing_system_layout(
        session,
        previous_writing_system,
        session.writing_system as usize,
    );
}