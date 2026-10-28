mod engine;
mod game;

use game::{control, game_over, initialization, pause, rest, tbt};

use crate::game::settings::save_settings;

fn main() {
    let mut session = initialization::initialize();

    while session.eng.work() && !session.exit{
        if session.action_cooldown > 0.0 {
            session.action_cooldown -= session.eng.logic_frametime;
        }

        match session.game_state{
            game::GameState::InGame => {
                if session.resize_cnt != session.eng.render.resize_count{
                    session.resize_cnt = session.eng.render.resize_count;
                    save_settings(&session);
                }
                session.eng.render.active_renderpasses = [true, true, true, true];
                tbt::update(&mut session);
                control::update(&mut session);
                rest::update(&mut session);
                game_over::handle(&mut session);
            },
            game::GameState::Gameover => {},
            game::GameState::Paused => {
                if session.resize_cnt != session.eng.render.resize_count{
                    session.tredr = ((1.0 / session.eng.gpu_fps as f32)*2.0).max(0.1);
                    session.resize_cnt = session.eng.render.resize_count;
                    save_settings(&session);
                }
                session.eng.render.active_renderpasses = [false, false, false, true];
                if session.tredr > 0.0{
                    session.eng.render.active_renderpasses = [true, true, true, true];
                    session.scn.exec(&mut session.eng);
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
                    session.tredr -= session.eng.logic_frametime;
                }
                pause::update(&mut session);
                pause::handle_input(&mut session);
            },
            game::GameState::MainMenu => {},
        }

        session.frame_count += 1;
        session.current_time += session.eng.logic_frametime*session.tmcoef;
        if session.current_time >= 16777216.0{
            session.current_time = 16777215.0;
            session.tmcoef = -session.tmcoef;
        }
        if session.frame_count > 100000{
            session.frame_count = 0;
        }
    }

    session.eng.end();
}
