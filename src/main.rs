mod engine;
mod game;

use game::{control, game_over, initialization, pause, rest, tbt};

use crate::game::{GameState, demo_end, mainmenu, settings::save_settings};

fn main() {
    let mut session = initialization::initialize();

    while session.eng.work() && !session.exit{
        if session.action_cooldown > 0.0 {
            session.action_cooldown -= session.eng.logic_frametime;
        }

        let previous_game_state = session.game_state;
        let previous_submenu = session.submenu;
        let previous_gamepad_selection = session.gamepadsel;

        match session.game_state{
            game::GameState::InGame => {
                if session.resize_cnt != session.eng.render.resize_count{
                    session.resize_cnt = session.eng.render.resize_count;
                    save_settings(&session);
                }
                session.eng.render.active_renderpasses = [true, true, true, true];
                if session.pivot.tbtmode{
                    tbt::update(&mut session);
                }
                control::update(&mut session);
                rest::update(&mut session);
                game_over::handle(&mut session);
            },
            game::GameState::Gameover => {
                if session.resize_cnt != session.eng.render.resize_count{
                    session.tredr = ((1.0 / session.eng.gpu_fps as f64)*2.0).max(0.1);
                    session.resize_cnt = session.eng.render.resize_count;
                    save_settings(&session);
                }
                session.eng.render.active_renderpasses = [false, false, false, true];
                if session.tredr > 0.0{
                    session.eng.render.active_renderpasses = [true, true, true, true];
                    session.scn.exec(&mut session.eng);
                    match session.use_rt_lighting{
                        true => {
                            session.ltviewport.object.physic_object.scale.x = session.eng.render.resolution_x as f32 * session.eng.render.resolution_scale;
                            session.ltviewport.object.physic_object.scale.y = session.eng.render.resolution_y as f32 * session.eng.render.resolution_scale;
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
                        
                            session.ltsviewport.object.draw = false;
                            session.ltsviewport.exec(&mut session.eng);
                        }
                        false => {
                            session.ltsviewport.object.physic_object.scale.x = session.eng.render.resolution_x as f32 * session.eng.render.resolution_scale;
                            session.ltsviewport.object.physic_object.scale.y = session.eng.render.resolution_y as f32 * session.eng.render.resolution_scale;
                            session.ltsviewport.object.physic_object.pos.z = 0.9;
                            session.ltsviewport.object.mesh.ubo[48] = session.frame_count as f32;
                            session.ltsviewport.object.mesh.ubo[49] = session.scn.voxel_representation.size[0] as f32;
                            session.ltsviewport.object.mesh.ubo[50] = session.scn.voxel_representation.size[1] as f32;
                            session.ltsviewport.object.mesh.ubo[51] = session.scn.voxel_representation.size[2] as f32;
                            session.ltsviewport.object.mesh.ubo[52] = session.scn.voxel_representation.origin.x as f32;
                            session.ltsviewport.object.mesh.ubo[53] = session.scn.voxel_representation.origin.y as f32;
                            session.ltsviewport.object.mesh.ubo[54] = session.scn.voxel_representation.origin.z as f32;
                            session.ltsviewport.object.mesh.ubo[55] = session.scn.voxel_representation.voxel_size;
                            session.ltsviewport.object.draw = true;
                            session.ltsviewport.exec(&mut session.eng);
                        
                            session.ltviewport.object.draw = false;
                            session.ltviewport.exec(&mut session.eng);
                        }
                    }
                    session.tredr -= session.eng.logic_frametime;
                    if !session.tredr.is_normal(){
                        session.tredr = 0.0;
                    }
                }
                game_over::handle_gom(&mut session);
            },
            game::GameState::DemoEnd => {
                if session.resize_cnt != session.eng.render.resize_count{
                    session.tredr = ((1.0 / session.eng.gpu_fps as f64)*2.0).max(0.1);
                    session.resize_cnt = session.eng.render.resize_count;
                    save_settings(&session);
                }
                session.eng.render.active_renderpasses = [false, false, false, true];
                if session.tredr > 0.0{
                    session.eng.render.active_renderpasses = [true, true, true, true];
                    session.scn.exec(&mut session.eng);
                    match session.use_rt_lighting{
                        true => {
                            session.ltviewport.object.physic_object.scale.x = session.eng.render.resolution_x as f32 * session.eng.render.resolution_scale;
                            session.ltviewport.object.physic_object.scale.y = session.eng.render.resolution_y as f32 * session.eng.render.resolution_scale;
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
                        
                            session.ltsviewport.object.draw = false;
                            session.ltsviewport.exec(&mut session.eng);
                        }
                        false => {
                            session.ltsviewport.object.physic_object.scale.x = session.eng.render.resolution_x as f32 * session.eng.render.resolution_scale;
                            session.ltsviewport.object.physic_object.scale.y = session.eng.render.resolution_y as f32 * session.eng.render.resolution_scale;
                            session.ltsviewport.object.physic_object.pos.z = 0.9;
                            session.ltsviewport.object.mesh.ubo[48] = session.frame_count as f32;
                            session.ltsviewport.object.mesh.ubo[49] = session.scn.voxel_representation.size[0] as f32;
                            session.ltsviewport.object.mesh.ubo[50] = session.scn.voxel_representation.size[1] as f32;
                            session.ltsviewport.object.mesh.ubo[51] = session.scn.voxel_representation.size[2] as f32;
                            session.ltsviewport.object.mesh.ubo[52] = session.scn.voxel_representation.origin.x as f32;
                            session.ltsviewport.object.mesh.ubo[53] = session.scn.voxel_representation.origin.y as f32;
                            session.ltsviewport.object.mesh.ubo[54] = session.scn.voxel_representation.origin.z as f32;
                            session.ltsviewport.object.mesh.ubo[55] = session.scn.voxel_representation.voxel_size;
                            session.ltsviewport.object.draw = true;
                            session.ltsviewport.exec(&mut session.eng);
                        
                            session.ltviewport.object.draw = false;
                            session.ltviewport.exec(&mut session.eng);
                        }
                    }
                    session.tredr -= session.eng.logic_frametime;
                    if !session.tredr.is_normal(){
                        session.tredr = 0.0;
                    }
                }
                demo_end::handle_demoend(&mut session);
            },
            game::GameState::Paused => {
                if session.resize_cnt != session.eng.render.resize_count{
                    session.tredr = ((1.0 / session.eng.gpu_fps as f64)*2.0).max(0.1);
                    session.resize_cnt = session.eng.render.resize_count;
                    save_settings(&session);
                }
                session.eng.render.active_renderpasses = [false, false, false, true];
                if session.tredr > 0.0{
                    session.eng.render.active_renderpasses = [true, true, true, true];
                    session.scn.exec(&mut session.eng);
                    match session.use_rt_lighting{
                        true => {
                            session.ltviewport.object.physic_object.scale.x = session.eng.render.resolution_x as f32 * session.eng.render.resolution_scale;
                            session.ltviewport.object.physic_object.scale.y = session.eng.render.resolution_y as f32 * session.eng.render.resolution_scale;
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
                        
                            session.ltsviewport.object.draw = false;
                            session.ltsviewport.exec(&mut session.eng);
                        }
                        false => {
                            session.ltsviewport.object.physic_object.scale.x = session.eng.render.resolution_x as f32 * session.eng.render.resolution_scale;
                            session.ltsviewport.object.physic_object.scale.y = session.eng.render.resolution_y as f32 * session.eng.render.resolution_scale;
                            session.ltsviewport.object.physic_object.pos.z = 0.9;
                            session.ltsviewport.object.mesh.ubo[48] = session.frame_count as f32;
                            session.ltsviewport.object.mesh.ubo[49] = session.scn.voxel_representation.size[0] as f32;
                            session.ltsviewport.object.mesh.ubo[50] = session.scn.voxel_representation.size[1] as f32;
                            session.ltsviewport.object.mesh.ubo[51] = session.scn.voxel_representation.size[2] as f32;
                            session.ltsviewport.object.mesh.ubo[52] = session.scn.voxel_representation.origin.x as f32;
                            session.ltsviewport.object.mesh.ubo[53] = session.scn.voxel_representation.origin.y as f32;
                            session.ltsviewport.object.mesh.ubo[54] = session.scn.voxel_representation.origin.z as f32;
                            session.ltsviewport.object.mesh.ubo[55] = session.scn.voxel_representation.voxel_size;
                            session.ltsviewport.object.draw = true;
                            session.ltsviewport.exec(&mut session.eng);
                        
                            session.ltviewport.object.draw = false;
                            session.ltviewport.exec(&mut session.eng);
                        }
                    }
                    session.tredr -= session.eng.logic_frametime;
                    if !session.tredr.is_normal(){
                        session.tredr = 0.0;
                    }
                }
                pause::update(&mut session);
                pause::handle_input(&mut session);
            },
            game::GameState::MainMenu => {
                if session.resize_cnt != session.eng.render.resize_count{
                    session.tredr = ((1.0 / session.eng.gpu_fps as f64)*2.0).max(0.1);
                    session.resize_cnt = session.eng.render.resize_count;
                    save_settings(&session);
                }
                session.eng.render.active_renderpasses = [false, false, false, true];
                if session.tredr > 0.0{
                    session.eng.render.active_renderpasses = [true, true, true, true];
                    session.scn.exec(&mut session.eng);
                    match session.use_rt_lighting{
                        true => {
                            session.ltviewport.object.physic_object.scale.x = session.eng.render.resolution_x as f32 * session.eng.render.resolution_scale;
                            session.ltviewport.object.physic_object.scale.y = session.eng.render.resolution_y as f32 * session.eng.render.resolution_scale;
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
                        
                            session.ltsviewport.object.draw = false;
                            session.ltsviewport.exec(&mut session.eng);
                        }
                        false => {
                            session.ltsviewport.object.physic_object.scale.x = session.eng.render.resolution_x as f32 * session.eng.render.resolution_scale;
                            session.ltsviewport.object.physic_object.scale.y = session.eng.render.resolution_y as f32 * session.eng.render.resolution_scale;
                            session.ltsviewport.object.physic_object.pos.z = 0.9;
                            session.ltsviewport.object.mesh.ubo[48] = session.frame_count as f32;
                            session.ltsviewport.object.mesh.ubo[49] = session.scn.voxel_representation.size[0] as f32;
                            session.ltsviewport.object.mesh.ubo[50] = session.scn.voxel_representation.size[1] as f32;
                            session.ltsviewport.object.mesh.ubo[51] = session.scn.voxel_representation.size[2] as f32;
                            session.ltsviewport.object.mesh.ubo[52] = session.scn.voxel_representation.origin.x as f32;
                            session.ltsviewport.object.mesh.ubo[53] = session.scn.voxel_representation.origin.y as f32;
                            session.ltsviewport.object.mesh.ubo[54] = session.scn.voxel_representation.origin.z as f32;
                            session.ltsviewport.object.mesh.ubo[55] = session.scn.voxel_representation.voxel_size;
                            session.ltsviewport.object.draw = true;
                            session.ltsviewport.exec(&mut session.eng);
                        
                            session.ltviewport.object.draw = false;
                            session.ltviewport.exec(&mut session.eng);
                        }
                    }
                    session.tredr -= session.eng.logic_frametime;
                    if !session.tredr.is_normal(){
                        session.tredr = 0.0;
                    }
                }
                mainmenu::update(&mut session);
                mainmenu::handle_input(&mut session);
            },
        }

        if (previous_game_state != session.game_state && session.game_state != GameState::InGame) || previous_submenu != session.submenu {
            game::do_paper_sliding_sound(&mut session);
        }
        if previous_gamepad_selection != session.gamepadsel
            && session.input_type == game::InputType::Gamepad
        {
            game::do_paper_writing_sound(&mut session);
        }

        session.frame_count += 1;
        session.current_time += session.eng.logic_frametime * session.tmcoef;
        if session.current_time >= 9_007_199_254_740_992.0 {
            session.current_time = 9_007_199_254_740_991.0;
            session.tmcoef = -session.tmcoef;
        }
        if session.frame_count > 100000{
            session.frame_count = 0;
        }
    }

    session.eng.end();
}
