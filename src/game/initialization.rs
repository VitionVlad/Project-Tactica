use crate::{engine::{
    engine::Engine, image::Image, light::{Light, LightType}, loader::rw::readfs, material::Material, math::{ivec3::Ivec3, vec3::Vec3}, render::render::{CullMode, MeshUsage, TextureFormat}, scene::{Scene, SceneVoxelizer}, speaker::Speaker, ui::{UIplane, UItext}, voxel::VoxelizerSettings,
}, game::ControllerMappings};

use super::{Awarness, Enemy, GameSession, Pivot, TBTplate};

pub fn initialize() -> GameSession {
    let mut eng = Engine::new(false);
    eng.render.set_title("rttest");
    eng.render.set_new_resolution(1280, 720);

    let vert = readfs("shaders/vert");
    let frag = readfs("shaders/frag");
    let dvert = readfs("shaders/vdeffered");
    let dfrag = readfs("shaders/fdeffered");
    let lfrag = readfs("shaders/fslight");
    let shadow = readfs("shaders/shadow");
    let textf = readfs("shaders/ftext");
    let textf2 = readfs("shaders/ftext2");

    let matt = Material::new(
        &eng,
        vert.clone(),
        textf,
        vec![],
        [CullMode::CullModeNone, CullMode::CullModeNone],
    );
    let matt2 = Material::new(
        &eng,
        vert.clone(),
        textf2,
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
        "assets/consolas.png",
        "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`~$% ",
        MeshUsage::PostPass,
    );

    let mut text = Vec::new();
    for i in 0..4 {
        let mut t = UItext::new_from_file(
            &mut eng,
            matt2,
            "assets/comicmonoinv.png",
            "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`~$% ",
            MeshUsage::PostPass,
        );
        t.new_line_symbol = b'|';
        text.push(t);
    }

    fpscnt.new_line_symbol = b'|';

    let mut scn = Scene::load_from_gltf(&mut eng, "assets/cpc.glb", matgeneral, Some(SceneVoxelizer {
        settings: VoxelizerSettings{ 
            size: Some(Ivec3{ x: 400, y: 225, z: 204}), 
            origin: Some(Vec3 { x: -21.5, y: -1.0, z: -17.0 }),
            voxel_size: 0.1,
            //size: Some(Ivec3{ x: 800, y: 450, z: 480}), 
            //origin: Some(Vec3 { x: -21.5, y: -1.0, z: -17.0 }),
            //voxel_size: 0.05,
            //size: None, 
            //origin: None,
        },
        object_filters: vec![
            ("pivot".to_string(), false),
            ("ph_".to_string(), false),
            ("enemy1_".to_string(), false),
            ("light1_".to_string(), false),
        ],
    }));
    let black3d = Image::new(&eng, scn.voxel_representation.size, scn.voxel_representation.data.clone(), true, TextureFormat::R8g8b8a8Unorm, false);
    println!("voxelized scene size: {}MB", scn.voxel_representation.data.len() as f32/1048576.0);
    //let black3d = Image::new(
    //    &eng,
    //    [1, 1, 1],
    //    vec![u8::MAX; 4],
    //    true,
    //    TextureFormat::R8g8b8a8Unorm,
    //    false,
    //);
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
    eng.used_light_count = 0;
    eng.lights = Vec::new();
    //eng.lights[0].shadow = false;
    //eng.lights[0].light_type = crate::engine::light::LightType::Spot;
    //eng.lights[0].pos = Vec3 {
    //    x: 0.0,
    //    y: 5.0,
    //    z: 0.0,
    //};
    //eng.lights[0].color = Vec3 {
    //    x: 1.25,
    //    y: 1.25,
    //    z: 1.25,
    //};
    eng.cameras[0].physic_object.pos.y = 5.0;
    eng.cameras[0].fov = 40.0;

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
        }else if name.contains("tactica_") {
            tbtplates.push(TBTplate {
                id: i,
                safe: false,
                enemy_id: None,
            });
        }else if name.contains("safe_") {
            tbtplates.push(TBTplate {
                id: i,
                safe: true,
                enemy_id: None,
            });
        }else if name.contains("ph_") {
            scn.objects[i].physic_object.is_static = false;
            scn.objects[i].physic_object.compute_torque = false;
            physics_objects.push((i, scn.objects[i].physic_object.pos));
        }else if name.contains("enemy1_") {
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
        }else if name.contains("light1_") {
            eng.lights.push(Light::new(LightType::Spot));
            let lastltmt = eng.lights.last_mut().unwrap();
            lastltmt.pos = scn.objects[i].physic_object.pos;
            lastltmt.rot = Vec3{ x: -1.5708, y: 0.0, z: 0.0}.to_quat();
            lastltmt.color = Vec3 {
                x: 1.25,
                y: 0.56,
                z: 0.33,
            };
            lastltmt.shadow = true;
            eng.used_light_count += 1;
        }
        if name.contains("ns_") {
            scn.objects[i].physic_object.solid = false;
        }
    }

    eng.render.shadow_map_resolution = 512;

    //eng.render.resolution_scale = 0.75;
    eng.cameras[0].physic_object.solid = false;
    eng.cameras[0].physic_object.gravity = false;
    println!("shadowmapresolution(ignore if shadowmaps are off): {}", eng.render.shadow_map_resolution);

    eng.render.resolution_scale = 0.5;

    let audiofl = [
        "assets/audio/bell.mp3",
        "assets/audio/paper.mp3",
        "assets/audio/type.mp3",
        "assets/audio/walking.mp3",
        "assets/audio/knife.mp3",
        "assets/audio/cityambient.mp3",
    ];

    let mut speakers = Vec::new();

    for fl in audiofl {
        let mut sp = Speaker::new(&mut eng, fl);
        sp.volume = 0.5;
        sp.loopsound = false;
        sp.play = false;
        sp.use_pan = false;
        sp.pos_dependency = false;
        speakers.push(sp);
    }

    speakers[3].loopsound = true;
    speakers[3].play = true;
    speakers[3].volume = 0.0;

    speakers[5].loopsound = true;
    speakers[5].play = true;
    speakers[5].volume = 0.1;

    GameSession {
        eng,
        scn,
        viewport,
        ltviewport,
        fpscnt,
        text,
        ui_scale: 1.0,
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
        speakers: speakers,
        controller_mappings: ControllerMappings{
            arrows: [12, 10, 11, 13],
            abxy: [0, 1, 2, 3],
            axis: [0, 1, 2, 3],
            bumpers: [4, 5],
            start: 7,
            select: 6,
        },
        action_list: [None, None, None, None, None],
    }
}

fn configure_actor(object: &mut crate::engine::physics::physics::PhysicsObject) {
    object.compute_torque = false;
    //object.gravity = false;
    object.step_height = 0.25;
    object.is_static = false;
    object.compute_force = false;
}