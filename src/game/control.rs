use crate::{engine::math::vec3::Vec3, game::Awarness};

use super::{GameSession, tbt, tile_test_point};

const SPEED: f32 = 25.0;
const KEYS: [u32; 5] = [40, 44, 25, 22, 48];

pub fn update(session: &mut GameSession) {
    if session.pivot.tbtmode {
        update_tbt_controls(session);
    } else {
        update_adventure_controls(session);
    }
}

fn update_tbt_controls(session: &mut GameSession) {
    session.object_holding = None;
    let pivot_position = session.scn.objects[session.pivot.id].physic_object.pos;
    let target_position = session.scn.objects[session.tbtplates[session.actual_plate_id].id]
        .physic_object
        .pos;
    let delta = Vec3 {
        x: target_position.x - pivot_position.x,
        y: 0.0,
        z: target_position.z - pivot_position.z,
    };

    if delta.length_squared().sqrt() < 0.01 {
        if session.tbtplates[session.actual_plate_id].safe {
            session.pivot.tbtmode = false;
        }

        let directions = [
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
        ];
        let mut adjacent = [None; 4];
        for (index, plate) in session.tbtplates.iter().enumerate() {
            for (direction_index, direction) in directions.iter().enumerate() {
                if tile_test_point(
                    &(pivot_position + *direction),
                    &session.scn.objects[plate.id].physic_object,
                ) {
                    adjacent[direction_index] = Some((index, plate.enemy_id.is_some()));
                }
            }
        }

        let enemy_on_player_plate = session
            .enemies
            .iter()
            .any(|enemy| enemy.alive && enemy.plate_id == Some(session.actual_plate_id));
        if enemy_on_player_plate {
            session.awareness = Awarness::Compromised(session.max_awareness);
        }

        for (key_index, next_plate) in adjacent.iter().enumerate() {
            if session.eng.control.get_key_state(KEYS[key_index]) && next_plate.is_some() {
                let (plate_id, occupied) = next_plate.unwrap();
                session.actual_plate_id = plate_id;
                if occupied {
                    let enemy_id = session.tbtplates[plate_id].enemy_id.unwrap();
                    session.enemies[enemy_id].alive = false;
                    tbt::increase_awareness(&mut session.awareness, session.max_awareness);
                    let object_id = session.enemies[enemy_id].id;
                    session.scn.objects[object_id].physic_object.pos.x = -1000.0;
                    session.scn.objects[object_id].draw = false;
                }
                session.making_step = true;
                break;
            }
        }

        if session.eng.control.get_key_state(KEYS[4]) && session.action_cooldown <= 0.0 {
            session.making_step = true;
            session.action_cooldown = 0.5;
        }
    }

    let target = session.scn.objects[session.tbtplates[session.actual_plate_id].id]
        .physic_object
        .pos;
    let pivot = &mut session.scn.objects[session.pivot.id].physic_object;
    pivot.acceleration.x = -(pivot.pos.x - target.x) * 10.0;
    pivot.acceleration.z = -(pivot.pos.z - target.z) * 10.0;
}

fn update_adventure_controls(session: &mut GameSession) {
    let pivot_id = session.pivot.id;
    if session.eng.control.get_key_state(KEYS[0]) {
        session.scn.objects[pivot_id].physic_object.acceleration.z += SPEED;
        session.current_dir = Vec3 {
            x: 0.0,
            y: 1.4,
            z: 1.0,
        };
    }
    if session.eng.control.get_key_state(KEYS[1]) {
        session.scn.objects[pivot_id].physic_object.acceleration.z -= SPEED;
        session.current_dir = Vec3 {
            x: 0.0,
            y: 1.4,
            z: -1.0,
        };
    }
    if session.eng.control.get_key_state(KEYS[2]) {
        session.scn.objects[pivot_id].physic_object.acceleration.x += SPEED;
        session.current_dir = Vec3 {
            x: 1.0,
            y: 1.4,
            z: 0.0,
        };
    }
    if session.eng.control.get_key_state(KEYS[3]) {
        session.scn.objects[pivot_id].physic_object.acceleration.x -= SPEED;
        session.current_dir = Vec3 {
            x: -1.0,
            y: 1.4,
            z: 0.0,
        };
    }
    if session.eng.control.get_key_state(KEYS[4]) && session.action_cooldown <= 0.0 {
        if session.object_holding.is_none() {
            session.object_holding = Some((None, session.current_dir));
            let point = session.scn.objects[pivot_id].physic_object.pos + session.current_dir;
            for object_id in &session.physics_objects {
                let object = &mut session.scn.objects[object_id.0].physic_object;
                if (object.pos - point).length_squared().sqrt() <= 1.8 {
                    object.pos.y += 0.1;
                    object.acceleration = (point - object.pos) * 100.0;
                    session.object_holding = Some((Some(object_id.0), session.current_dir));
                    break;
                }
            }
        } else {
            session.object_holding = None;
        }
        session.action_cooldown = 0.25;
    }
}
