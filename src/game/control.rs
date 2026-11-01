use crate::{
    engine::math::vec3::Vec3,
    game::{Action::Skip, Awarness, Direction, DIRECTIONS},
};

use super::{tile_test_point, GameSession, InputType};

const SPEED: f32 = 25.0;

pub fn update(session: &mut GameSession) {
    update_input_type(session);
    if session.action_cooldown <= 0.0 && start_pressed(session) {
        session.game_state = super::GameState::Paused;
        session.action_cooldown = 0.25;
        return;
    }

    if session.pivot.tbtmode {
        update_tbt_controls(session);
    } else {
        update_adventure_controls(session);
    }
}

pub fn start_pressed(session: &mut GameSession) -> bool {
    if session
        .eng
        .control
        .get_key_state(session.keyboard_bindings.start.keycode as u32)
    {
        session.input_type = InputType::Keyboard;
        return true;
    }

    if session.eng.control.gamepad_connected
        && session
            .eng
            .control
            .get_gamepad_button_state(session.controller_mappings.start as u8)
    {
        session.input_type = InputType::Gamepad;
        return true;
    }

    false
}

fn update_input_type(session: &mut GameSession) {
    let keyboard_pressed = session
        .keyboard_bindings
        .arrows
        .iter()
        .any(|key| session.eng.control.get_key_state(key.keycode as u32))
        || session
            .eng
            .control
            .get_key_state(session.keyboard_bindings.abxy[0].keycode as u32)
        || session
            .eng
            .control
            .get_key_state(session.keyboard_bindings.start.keycode as u32);
    if keyboard_pressed {
        session.input_type = InputType::Keyboard;
    }

    if session.eng.control.gamepad_connected {
        let gamepad_button_pressed = session
            .controller_mappings
            .arrows
            .iter()
            .any(|button| session.eng.control.get_gamepad_button_state(*button as u8))
            || session
                .eng
                .control
                .get_gamepad_button_state(session.controller_mappings.abxy[0] as u8)
            || session
                .eng
                .control
                .get_gamepad_button_state(session.controller_mappings.start as u8);
        let gamepad_axis_moved = session.controller_mappings.axis.iter().take(2).any(|axis| {
            session
                .eng
                .control
                .get_gamepad_axis_state(*axis as u8)
                .abs()
                > 0.1
        });

        if gamepad_button_pressed || gamepad_axis_moved {
            session.input_type = InputType::Gamepad;
        }
    }
}

fn update_tbt_controls(session: &mut GameSession) {
    for i in 0..5 {
        session.action_list[i] = None;
    }
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
        session.action_list[4] = Some(Skip);
        if session.tbtplates[session.actual_plate_id].safe {
            session.pivot.tbtmode = false;
        }
        let mut adjacent = [false; 4];
        for plate in &session.tbtplates {
            for (direction_index, direction) in DIRECTIONS.iter().enumerate() {
                if tile_test_point(
                    &(pivot_position + *direction),
                    &session.scn.objects[plate.id].physic_object,
                ) {
                    adjacent[direction_index] = true;
                    session.action_list[direction_index] = Some(match plate.enemy_id.is_some() {
                        true => super::Action::StepWithElemination(match direction_index {
                            0 => (Direction::Up, plate.id),
                            1 => (Direction::Down, plate.id),
                            2 => (Direction::Left, plate.id),
                            3 => (Direction::Right, plate.id),
                            _ => (Direction::Up, plate.id),
                        }),
                        false => super::Action::Step(match direction_index {
                            0 => (Direction::Up, plate.id),
                            1 => (Direction::Down, plate.id),
                            2 => (Direction::Left, plate.id),
                            3 => (Direction::Right, plate.id),
                            _ => (Direction::Up, plate.id),
                        }),
                    });
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

        if session.action_cooldown <= 0.0 {
            session.action_list[4] = Some(super::Action::Skip);
        } else {
            session.action_list[4] = None;
        }

        for (key_index, next_plate) in adjacent.iter().enumerate() {
            if session
                .eng
                .control
                .get_key_state(session.keyboard_bindings.arrows[key_index].keycode as u32)
                && *next_plate
            {
                session.active_action = session.action_list[key_index];
                break;
            }
        }

        if session
            .eng
            .control
            .get_key_state(session.keyboard_bindings.abxy[0].keycode as u32)
            && session.action_cooldown <= 0.0
        {
            if session.active_action.is_none() {
                session.active_action = session.action_list[4];
            }
        }

        if session.eng.control.gamepad_connected {
            for (key_index, next_plate) in adjacent.iter().enumerate() {
                if session
                    .eng
                    .control
                    .get_gamepad_button_state(session.controller_mappings.arrows[key_index] as u8)
                    && *next_plate
                {
                    if session.active_action.is_none() {
                        session.active_action = session.action_list[key_index];
                    }
                    break;
                }
            }

            if session
                .eng
                .control
                .get_gamepad_button_state(session.controller_mappings.abxy[0] as u8)
                && session.action_cooldown <= 0.0
            {
                if session.active_action.is_none() {
                    session.active_action = session.action_list[4];
                }
            }
        }
    } else {
        session.speakers[3].volume = 0.25;
    }

    let target = session.scn.objects[session.tbtplates[session.actual_plate_id].id]
        .physic_object
        .pos;
    let pivot = &mut session.scn.objects[session.pivot.id].physic_object;
    pivot.acceleration.x = -(pivot.pos.x - target.x) * 10.0;
    pivot.acceleration.z = -(pivot.pos.z - target.z) * 10.0;
}

fn update_adventure_controls(session: &mut GameSession) {

    if session
        .eng
        .control
        .get_key_state(0)
    {
        session.game_state = super::GameState::Gameover;
    }

    let pivot_id = session.pivot.id;
    if session
        .eng
        .control
        .get_key_state(session.keyboard_bindings.arrows[0].keycode as u32)
    {
        session.scn.objects[pivot_id].physic_object.acceleration.z += SPEED;
        session.current_dir = Vec3 {
            x: 0.0,
            y: 1.4,
            z: 1.0,
        };
        session.speakers[3].volume = 0.25;
    }
    if session
        .eng
        .control
        .get_key_state(session.keyboard_bindings.arrows[1].keycode as u32)
    {
        session.scn.objects[pivot_id].physic_object.acceleration.z -= SPEED;
        session.current_dir = Vec3 {
            x: 0.0,
            y: 1.4,
            z: -1.0,
        };
        session.speakers[3].volume = 0.25;
    }
    if session
        .eng
        .control
        .get_key_state(session.keyboard_bindings.arrows[2].keycode as u32)
    {
        session.scn.objects[pivot_id].physic_object.acceleration.x += SPEED;
        session.current_dir = Vec3 {
            x: 1.0,
            y: 1.4,
            z: 0.0,
        };
        session.speakers[3].volume = 0.25;
    }
    if session
        .eng
        .control
        .get_key_state(session.keyboard_bindings.arrows[3].keycode as u32)
    {
        session.scn.objects[pivot_id].physic_object.acceleration.x -= SPEED;
        session.current_dir = Vec3 {
            x: -1.0,
            y: 1.4,
            z: 0.0,
        };
        session.speakers[3].volume = 0.25;
    }
    if session
        .eng
        .control
        .get_key_state(session.keyboard_bindings.abxy[0].keycode as u32)
        && session.action_cooldown <= 0.0
    {
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

    if session.eng.control.gamepad_connected {
        let gmv = (
            session
                .eng
                .control
                .get_gamepad_axis_state(session.controller_mappings.axis[1] as u8),
            session
                .eng
                .control
                .get_gamepad_axis_state(session.controller_mappings.axis[0] as u8),
        );
        session.scn.objects[pivot_id].physic_object.acceleration.z += SPEED * gmv.0;
        session.scn.objects[pivot_id].physic_object.acceleration.x += SPEED * gmv.1;
        if gmv.0.abs() > 0.1 || gmv.1.abs() > 0.1 {
            session.speakers[3].volume = 0.25;
        }
    }
}
