use super::{Awarness, GameSession};

pub fn handle(session: &mut GameSession) {
    if session.awareness != Awarness::Compromised(session.max_awareness) {
        return;
    }
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
}
