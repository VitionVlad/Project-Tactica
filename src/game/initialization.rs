use crate::engine::{
    engine::Engine,
    image::Image,
    loader::rw::readfs,
    material::Material,
    math::vec3::Vec3,
    render::render::{CullMode, MeshUsage, TextureFormat},
    scene::Scene,
    ui::{UIplane, UItext},
};

use super::{Awarness, Enemy, GameSession, Pivot, TBTplate};

pub fn initialize() -> GameSession {
    let mut eng = Engine::new(false);
    eng.render.set_title("rttest");
    eng.render.set_new_resolution(1280, 720);

    let vert = readfs("shaders/vert");
    let frag = readfs("shaders/frag");
    let dvert = readfs("shaders/vdeffered");
    let dfrag = readfs("shaders/fdeffered");
    let lfrag = readfs("shaders/klight");
    let shadow = readfs("shaders/shadow");
    let textf = readfs("shaders/ftext");

    let matt = Material::new(
        &eng,
        vert.clone(),
        textf,
        vec![],
        [CullMode::CullModeNone, CullMode::CullModeNone],
    );
    let mat = Material::new(
        &eng,
        vert.clone(),
        frag,
        vec![],
        [CullMode::CullModeNone, CullMode::CullModeNone],
    );
    let lightmat = Material::new(
        &eng,
        vert.clone(),
        lfrag,
        vec![],
        [CullMode::CullModeNone, CullMode::CullModeNone],
    );
    let matgeneral = Material::new(
        &eng,
        dvert,
        dfrag,
        shadow,
        [CullMode::CullModeBackBit, CullMode::CullModeFrontBit],
    );

    let black = Image::new_color(&eng, [11, 23, 40, u8::MAX]);
    let mut viewport = UIplane::new(&mut eng, mat, vec![black], MeshUsage::PostPass);
    viewport.object.physic_object.pos.z = 1.0;
    viewport.signal = false;

    let mut fpscnt = UItext::new_from_file(
        &mut eng,
        matt,
        "assets/lat.png",
        "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`~$% ",
        MeshUsage::PostPass,
    );
    fpscnt.new_line_symbol = b'|';

    let mut scn = Scene::load_from_gltf(&mut eng, "assets/test.glb", matgeneral, false, 0.2);
    let black3d = Image::new(
        &eng,
        [1, 1, 1],
        vec![u8::MAX; 4],
        true,
        TextureFormat::R8g8b8a8Unorm,
        false,
    );
    let bluenoise = Image::new_from_files(&eng, vec!["assets/noise.png".to_string()]);
    let mut ltviewport = UIplane::new(
        &mut eng,
        lightmat,
        vec![black3d, bluenoise],
        MeshUsage::LightingPass,
    );
    ltviewport.object.physic_object.pos.z = 1.0;
    ltviewport.signal = false;
    scn.voxel_representation.data.resize(0, 0);

    eng.render.shadow_map_count = 0;
    eng.used_light_count = 1;
    eng.lights[0].shadow = false;
    eng.lights[0].light_type = crate::engine::light::LightType::Spot;
    eng.lights[0].pos = Vec3 {
        x: 0.0,
        y: 5.0,
        z: 0.0,
    };
    eng.lights[0].color = Vec3 {
        x: 1.25,
        y: 1.25,
        z: 1.25,
    };
    eng.cameras[0].physic_object.pos.y = 5.0;
    eng.cameras[0].fov = 30.0;

    let mut pivot = Pivot {
        id: 0,
        tbtmode: false,
        start_pos: Vec3::new(),
    };
    let mut tbtplates = Vec::new();
    let mut enemies = Vec::new();
    let mut physics_objects = Vec::new();

    for i in 0..scn.objects.len() {
        let name = scn.objects[i].name.clone();
        if name.contains("pivot") {
            pivot.id = i;
            pivot.start_pos = scn.objects[i].physic_object.pos;
            configure_actor(&mut scn.objects[i].physic_object);
        } else if name.contains("tactica_") {
            tbtplates.push(TBTplate {
                id: i,
                safe: false,
                enemy_id: None,
            });
        } else if name.contains("safe_") {
            tbtplates.push(TBTplate {
                id: i,
                safe: true,
                enemy_id: None,
            });
        } else if name.contains("ph_") {
            scn.objects[i].physic_object.is_static = false;
            physics_objects.push((i, scn.objects[i].physic_object.pos));
        } else if name.contains("enemy") {
            enemies.push(Enemy {
                id: i,
                direction: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                plate_id: None,
                alive: true,
                start_pos: scn.objects[i].physic_object.pos,
                initial_tile_id: None,
            });
            configure_actor(&mut scn.objects[i].physic_object);
        }
    }

    eng.cameras[0].physic_object.solid = false;
    eng.cameras[0].physic_object.gravity = false;
    println!(
        "shadowmapresolution(ignore if shadowmaps are off): {}",
        eng.render.shadow_map_resolution
    );

    GameSession {
        eng,
        scn,
        viewport,
        ltviewport,
        fpscnt,
        pivot,
        tbtplates,
        enemies,
        physics_objects,
        actual_plate_id: 0,
        awareness: Awarness::Unseen(0),
        max_awareness: 2,
        making_step: false,
        object_holding: None,
        current_dir: Vec3::new(),
        action_cooldown: 0.0,
        frame_count: 0,
    }
}

fn configure_actor(object: &mut crate::engine::physics::physics::PhysicsObject) {
    object.compute_torque = false;
    object.gravity = false;
    object.is_static = false;
    object.compute_force = false;
}
