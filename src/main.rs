mod engine;
mod game;

use game::{control, game_over, initialization, rest, tbt};

fn main() {
    let mut session = initialization::initialize();

    while session.eng.work() {
        if session.action_cooldown > 0.0 {
            session.action_cooldown -= session.eng.logic_frametime;
        }

        tbt::update(&mut session);
        control::update(&mut session);
        rest::update(&mut session);
        game_over::handle(&mut session);

        session.frame_count += 1;
        if session.frame_count > u32::MAX - 1000{
            session.frame_count = 0;
        }
    }

    session.eng.end();
}
