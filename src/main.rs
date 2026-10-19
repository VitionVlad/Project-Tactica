mod engine;
mod game;

use game::{control, game_over, initialization, pause, rest, tbt};

fn main() {
    let mut session = initialization::initialize();

    while session.eng.work() {
        if session.action_cooldown > 0.0 {
            session.action_cooldown -= session.eng.logic_frametime;
        }

        match session.game_state{
            game::GameState::InGame => {
                session.eng.render.active_renderpasses = [true, true, true, true];
                tbt::update(&mut session);
                control::update(&mut session);
                rest::update(&mut session);
                game_over::handle(&mut session);
            },
            game::GameState::Gameover => {},
            game::GameState::Paused => {
                if session.resize_cnt != session.eng.render.resize_count{
                    session.tredr = 0.1;
                    session.resize_cnt = session.eng.render.resize_count;
                }
                session.eng.render.active_renderpasses = [false, false, false, true];
                if session.tredr > 0.0{
                    session.eng.render.active_renderpasses = [true, true, true, true];
                    rest::update(&mut session);
                    session.tredr -= session.eng.logic_frametime;
                }
                pause::update(&mut session);
                pause::handle_input(&mut session);
            },
            game::GameState::MainMenu => {},
        }

        session.frame_count += 1;
        if session.frame_count > u32::MAX - 100000{
            session.frame_count = 0;
        }
    }

    session.eng.end();
}
