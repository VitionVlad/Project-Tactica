use crate::engine::math::vec3::Vec3;

use super::{GameSession, tile_test};

pub fn update(session: &mut GameSession) {
    let pivot_position = session.scn.objects[session.pivot.id].physic_object.pos;
    session.eng.cameras[0].physic_object.pos.x = pivot_position.x - 5.0;
    session.eng.cameras[0].physic_object.pos.y = pivot_position.y + 7.5;
    session.eng.cameras[0].physic_object.pos.z = pivot_position.z + 10.0;
    session.eng.cameras[0].look_at(
        pivot_position
            + Vec3 {
                x: 0.0,
                y: 0.5,
                z: 0.0,
            },
    );

    let fps_text = format!(
        "CPU_fps:{}|GPU_fps:{}|detection_level: {:?}",
        session.eng.cpu_fps, session.eng.gpu_fps, session.awareness
    );
    session.fpscnt.size.x = 15.0;
    session.fpscnt.size.y = 30.0;
    session.fpscnt.pos.x = 0.0;
    session.fpscnt.pos.y = 0.0;
    session.fpscnt.pos.z = 0.1;
    session.fpscnt.draw = true;
    session.fpscnt.exec(&mut session.eng, &fps_text);

    session.scn.exec(&mut session.eng);
    session.ltviewport.object.physic_object.scale.x =
        session.eng.render.resolution_x as f32 * session.eng.render.resolution_scale;
    session.ltviewport.object.physic_object.scale.y =
        session.eng.render.resolution_y as f32 * session.eng.render.resolution_scale;
    session.ltviewport.object.physic_object.pos.z = 0.9;
    session.ltviewport.object.mesh.ubo[48] = session.frame_count as f32;
    session.ltviewport.object.mesh.ubo[49] = session.scn.voxel_representation.size[0] as f32;
    session.ltviewport.object.mesh.ubo[50] = session.scn.voxel_representation.size[1] as f32;
    session.ltviewport.object.mesh.ubo[51] = session.scn.voxel_representation.size[2] as f32;
    session.ltviewport.object.mesh.ubo[52] = session.scn.voxel_representation.origin.x as f32;
    session.ltviewport.object.mesh.ubo[53] = session.scn.voxel_representation.origin.y as f32;
    session.ltviewport.object.mesh.ubo[54] = session.scn.voxel_representation.origin.z as f32;
    session.ltviewport.object.mesh.ubo[55] = session.scn.voxel_representation.voxel_size;
    session.ltviewport.exec(&mut session.eng);

    if !session.pivot.tbtmode {
        for (index, plate) in session.tbtplates.iter().enumerate() {
            if tile_test(
                &session.scn.objects[session.pivot.id].physic_object,
                &session.scn.objects[plate.id].physic_object,
            ) && !plate.safe
            {
                session.actual_plate_id = index;
                session.pivot.tbtmode = true;
                break;
            }
        }
    }

    if let Some(direction) = session.object_holding {
        let point = session.scn.objects[session.pivot.id].physic_object.pos + direction.1;
        if direction.0.is_some() {
            let object = &mut session.scn.objects[direction.0.unwrap()].physic_object;
            object.acceleration = (point - object.pos) * 100.0;
        }else{
            for object_id in &session.physics_objects {
                let object = &mut session.scn.objects[object_id.0].physic_object;
                if (object.pos - point).length_squared().sqrt() <= 1.8 {
                    object.acceleration = (point - object.pos) * 100.0;
                }
                session.object_holding = Some((Some(object_id.0), direction.1));
                break;
            }
        }
    }

    session.viewport.object.physic_object.scale.x = session.eng.render.resolution_x as f32;
    session.viewport.object.physic_object.scale.y = session.eng.render.resolution_y as f32;
    session.viewport.object.mesh.ubo[48] = session.frame_count as f32;
    session.viewport.object.mesh.ubo[49] = 0.0;
    session.viewport.object.mesh.ubo[50] = 0.2;
    session.viewport.object.mesh.ubo[51] = 0.18;
    session.viewport.exec(&mut session.eng);
}
