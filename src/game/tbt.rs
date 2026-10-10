use super::{Action, Awarness, GameSession, tile_test, tile_test_point};

pub fn update(session: &mut GameSession) {
    if let Some(action) = session.active_action.take() {
        match action {
            Action::Step((_, plate_object_id)) => {
                session.actual_plate_id = session
                    .tbtplates
                    .iter()
                    .position(|plate| plate.id == plate_object_id)
                    .expect("step action references an unknown plate");
                session.making_step = true;
            }
            Action::StepWithElemination((_, plate_object_id)) => {
                let plate_id = session
                    .tbtplates
                    .iter()
                    .position(|plate| plate.id == plate_object_id)
                    .expect("elimination action references an unknown plate");
                session.actual_plate_id = plate_id;
                let enemy_id = session.tbtplates[plate_id]
                    .enemy_id
                    .expect("elimination action references an unoccupied plate");
                session.enemies[enemy_id].alive = false;
                for _ in 0..(session.max_awareness + 1) {
                    increase_awareness(&mut session.awareness, session.max_awareness);
                }
                let object_id = session.enemies[enemy_id].id;
                session.scn.objects[object_id].physic_object.pos.x = -1000.0;
                session.scn.objects[object_id].draw = false;
                session.speakers[4].play = true;
                session.speakers[4].move_sound_cursor(0.0);
                session.making_step = true;
            }
            Action::Skip => {
                session.making_step = true;
                session.action_cooldown = 0.5;
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

    if session.making_step {
        session.making_step = false;
        for plate in &mut session.tbtplates {
            plate.enemy_id = None;
        }

        let pivot_position = session.scn.objects[session.pivot.id].physic_object.pos;
        let mut enemy_detected = false;
        for enemy in &mut session.enemies {
            let enemy_position = session.scn.objects[enemy.id].physic_object.pos;
            if (enemy_position - pivot_position).length_squared().sqrt() < 1.5 {
                enemy_detected = true;
                if !enemy_on_player_plate {
                    increase_awareness(&mut session.awareness, session.max_awareness);
                }
            }
        }

        for enemy_index in 0..session.enemies.len() {
            if !session.enemies[enemy_index].alive {
                continue;
            }
            let enemy_id = session.enemies[enemy_index].id;
            if session.enemies[enemy_index].plate_id.is_none() {
                for plate_index in 0..session.tbtplates.len() {
                    if tile_test(
                        &session.scn.objects[enemy_id].physic_object,
                        &session.scn.objects[session.tbtplates[plate_index].id].physic_object,
                    ) && !session.tbtplates[plate_index].safe
                    {
                        session.enemies[enemy_index].plate_id = Some(plate_index);
                        session.tbtplates[plate_index].enemy_id = Some(enemy_index);
                        if session.enemies[enemy_index].initial_tile_id.is_none() {
                            session.enemies[enemy_index].initial_tile_id = Some(plate_index);
                        }
                        break;
                    }
                }
            } else {
                let direction = session.enemies[enemy_index].direction;
                let current_position = session.scn.objects[enemy_id].physic_object.pos;
                let mut found = false;
                for plate_index in 0..session.tbtplates.len() {
                    if tile_test_point(
                        &(current_position + direction),
                        &session.scn.objects[session.tbtplates[plate_index].id].physic_object,
                    ) && !session.tbtplates[plate_index].safe
                    {
                        session.enemies[enemy_index].plate_id = Some(plate_index);
                        session.tbtplates[plate_index].enemy_id = Some(enemy_index);
                        found = true;
                        if plate_index == session.actual_plate_id {
                            session.awareness = Awarness::Compromised(session.max_awareness);
                        }
                        break;
                    }
                }
                if !found {
                    for plate_index in 0..session.tbtplates.len() {
                        if tile_test_point(
                            &(current_position - direction),
                            &session.scn.objects[session.tbtplates[plate_index].id].physic_object,
                        ) && !session.tbtplates[plate_index].safe
                        {
                            session.enemies[enemy_index].plate_id = Some(plate_index);
                            session.tbtplates[plate_index].enemy_id = Some(enemy_index);
                            session.enemies[enemy_index].direction = -direction;
                            break;
                        }
                    }
                }
            }
        }

        if !enemy_detected && !enemy_on_player_plate {
            decrease_awareness(&mut session.awareness, session.max_awareness);
        }
    }

    for enemy in &session.enemies {
        if enemy.alive && enemy.plate_id.is_some() {
            let plate_id = session.tbtplates[enemy.plate_id.unwrap()].id;
            let plate_position = session.scn.objects[plate_id].physic_object.pos;
            let enemy_object = &mut session.scn.objects[enemy.id].physic_object;
            enemy_object.acceleration.x = -(enemy_object.pos.x - plate_position.x) * 10.0;
            enemy_object.acceleration.z = -(enemy_object.pos.z - plate_position.z) * 10.0;
        }
    }
}

pub fn increase_awareness(awareness: &mut Awarness, max_awareness: i8) {
    *awareness = match *awareness {
        Awarness::Unseen(value) if value < max_awareness as u8 => Awarness::Unseen(value + 1),
        Awarness::Unseen(_) => Awarness::Suspected(0),
        Awarness::Suspected(value) if value < max_awareness => Awarness::Suspected(value + 1),
        Awarness::Suspected(_) => Awarness::Detected(0),
        Awarness::Detected(value) if value < max_awareness => Awarness::Detected(value + 1),
        Awarness::Detected(_) => Awarness::Compromised(0),
        Awarness::Compromised(value) => Awarness::Compromised(value.min(max_awareness)),
    };
}

fn decrease_awareness(awareness: &mut Awarness, max_awareness: i8) {
    *awareness = match *awareness {
        Awarness::Unseen(value) => Awarness::Unseen(value.saturating_sub(1)),
        Awarness::Suspected(value) if value > 0 => Awarness::Suspected(value - 1),
        Awarness::Suspected(_) => Awarness::Unseen(max_awareness as u8),
        Awarness::Detected(value) if value > 0 => Awarness::Detected(value - 1),
        Awarness::Detected(_) => Awarness::Suspected(max_awareness),
        Awarness::Compromised(value) if value > 0 => Awarness::Compromised(value - 1),
        Awarness::Compromised(_) => Awarness::Detected(max_awareness),
    };
}
