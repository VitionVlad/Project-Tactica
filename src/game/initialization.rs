use crate::{
    engine::{
        engine::Engine,
        image::Image,
        light::{Light, LightType},
        loader::{jsonparser::JsonF, rw::readfs},
        material::Material,
        math::{ivec3::Ivec3, vec3::Vec3},
        render::render::{CullMode, MeshUsage, TextureFormat},
        scene::{Scene, SceneVoxelizer},
        speaker::Speaker,
        ui::{UIplane, UItext},
        voxel::VoxelizerSettings,
    },
    game::{ControllerMappings, Key, KeyboardBindings},
};

use super::{Awarness, Enemy, GameSession, Pivot, TBTplate, TextContent};
use super::settings::load_settings;

pub fn initialize() -> GameSession {
    let mut eng = Engine::new(crate::engine::render::render::RenderDebug::None);
    eng.render.set_title("Project Tactica -- Dev build --");
    eng.render.set_new_resolution(1280, 720);

    let gmtxt = JsonF::load_from_file("assets/text.json");
    let chosen_language = 0;
    let text_content = TextContent::from_json(&gmtxt, chosen_language)
        .expect("failed to load text content from assets/text.json");
    let menu_item_capacity = text_content.pause.len()
        + text_content.settings.len()
        + text_content.game_set.len()
        + text_content.audio.len()
        + text_content.graphics.len();

    let vert = readfs("shaders/vert");
    let frag = readfs("shaders/frag");
    let dvert = readfs("shaders/vdeffered");
    let dfrag = readfs("shaders/fdeffered");
    let lfrag = readfs("shaders/fslight");
    let shadow = readfs("shaders/shadow");
    let textf = readfs("shaders/ftext");
    let textf2 = readfs("shaders/ftext2");
    let imgf = readfs("shaders/fimg");
    let bgf = readfs("shaders/fbg");
    let imgf2 = readfs("shaders/fimg2");
    let pbrs = readfs("shaders/pbr");

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
    let mati = Material::new(
        &eng,
        vert.clone(),
        imgf,
        vec![],
        [CullMode::CullModeNone, CullMode::CullModeNone],
    );
    let bgfi = Material::new(
        &eng,
        vert.clone(),
        bgf,
        vec![],
        [CullMode::CullModeNone, CullMode::CullModeNone],
    );
    let mati2 = Material::new(
        &eng,
        vert.clone(),
        imgf2,
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
    let pbrmat = Material::new(
        &eng,
        vert.clone(),
        pbrs,
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

    let mut awaspr = Vec::new();

    awaspr.push(UIplane::new_from_file(
        &mut eng,
        mati,
        ["assets/ui/awa1.png".to_string()].to_vec(),
        MeshUsage::PostPass,
    ));
    awaspr.push(UIplane::new_from_file(
        &mut eng,
        mati,
        ["assets/ui/awa2.png".to_string()].to_vec(),
        MeshUsage::PostPass,
    ));
    awaspr.push(UIplane::new_from_file(
        &mut eng,
        mati,
        ["assets/ui/awa3.png".to_string()].to_vec(),
        MeshUsage::PostPass,
    ));
    awaspr.push(UIplane::new_from_file(
        &mut eng,
        mati,
        ["assets/ui/awa4.png".to_string()].to_vec(),
        MeshUsage::PostPass,
    ));

    for pl in &mut awaspr {
        pl.object.physic_object.is_static = false;
        pl.object.physic_object.gravity = false;
        pl.object.physic_object.solid = false;
        pl.object.physic_object.compute_force = true;
        pl.object.physic_object.compute_torque = false;
        pl.object.physic_object.pin_pos = false;
        pl.object.physic_object.scale.x = 100.0;
        pl.object.physic_object.scale.y = 100.0;
        pl.object.physic_object.pos.z = 0.2;
        pl.object.physic_object.pos.x = -150.0;
        pl.object.physic_object.air_friction = 1.0;
    }

    let mut fpscnt = UItext::new_from_file(
        &mut eng,
        matt,
        "assets/consolas.png",
        "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`?!% ",
        MeshUsage::PostPass,
    );

    let mut text = Vec::new();
    for i in 0..4 {
        let t = UItext::new_from_file(
            &mut eng,
            matt2,
            "assets/comicmonoinv.png",
            "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`?!% ",
            MeshUsage::PostPass,
        );
        text.push(t);
    }

    let mut menu_text = Vec::new();
    for _ in 0..30 {
        let t = UItext::new_from_file(
            &mut eng,
            matt,
            "assets/comicmonoinv.png",
            "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`?!% ",
            MeshUsage::PostPass,
        );
        menu_text.push(t);
    }

    text.push(UItext::new_from_file(
        &mut eng,
        matt,
        "assets/comicmonoinv.png",
        "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`?!% ",
        MeshUsage::PostPass,
    ));

    let mut keyimg = Vec::new();
    for i in 0..4 {
        let t = UItext::new_from_file(
            &mut eng,
            matt2,
            "assets/btns.png",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789rtldaxbyqwzxtsjk",
            MeshUsage::PostPass,
        );
        keyimg.push(t);
    }

    keyimg.push(UItext::new_from_file(
        &mut eng,
        matt,
        "assets/btns.png",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789rtldaxbyqwzxtsjk",
        MeshUsage::PostPass,
    ));

    let imgbl = Image::new_color(&mut eng, [0, 0, 0, u8::MAX]);
    let mut spbg = Vec::new();
    for _ in 0..4 {
        let t = UIplane::new(&mut eng, mati2, vec![imgbl.clone()], MeshUsage::PostPass);
        spbg.push(t);
    }

    let imgw = Image::new_color(&mut eng, [u8::MAX, u8::MAX, u8::MAX, u8::MAX]);
    let mut wbg = Vec::new();
    for _ in 0..4 {
        let t = UIplane::new(&mut eng, mati, vec![imgw.clone()], MeshUsage::PostPass);
        wbg.push(t);
    }

    let mut bbg = Vec::new();
    for _ in 0..31 {
        let t = UIplane::new(&mut eng, mati, vec![imgbl], MeshUsage::PostPass);
        bbg.push(t);
    }

    let orimg = Image::new_color(&mut eng, [239, 0, 0, u8::MAX]);
    let blimg = Image::new_color(&mut eng, [5, 1, 174, u8::MAX]);
    let mut menubg = Vec::new();
    menubg.push(UIplane::new(
        &mut eng,
        bgfi,
        vec![orimg],
        MeshUsage::PostPass,
    ));
    menubg.push(UIplane::new(
        &mut eng,
        bgfi,
        vec![blimg],
        MeshUsage::PostPass,
    ));

    spbg.push(UIplane::new(
        &mut eng,
        mati,
        vec![imgbl.clone()],
        MeshUsage::PostPass,
    ));

    let lgi = Image::new_from_files(&eng, vec!["assets/ui/tlogo.png".to_string()]);

    let logoplane = UIplane::new(&mut eng, mati, vec![lgi], MeshUsage::PostPass);

    fpscnt.new_line_symbol = b'|';

    let mut scn = Scene::load_from_gltf(
        &mut eng,
        "assets/cpc.glb",
        matgeneral,
        Some(SceneVoxelizer {
            settings: VoxelizerSettings {
                size: Some(Ivec3 {
                    x: 400,
                    y: 225,
                    z: 204,
                }),
                origin: Some(Vec3 {
                    x: -21.5,
                    y: -1.0,
                    z: -17.0,
                }),
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
        }),
    );
    let black3d = Image::new(
        &eng,
        scn.voxel_representation.size,
        scn.voxel_representation.data.clone(),
        true,
        TextureFormat::R8g8b8a8Unorm,
        false,
    );
    println!(
        "voxelized scene size: {}MB",
        scn.voxel_representation.data.len() as f32 / 1048576.0
    );
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

    let mut ltsviewport = UIplane::new(
        &mut eng,
        pbrmat,
        vec![orimg],
        MeshUsage::LightingPass,
    );
    ltsviewport.object.physic_object.pos.z = 1.0;
    ltsviewport.signal = false;

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
            scn.objects[i].physic_object.compute_torque = false;
            physics_objects.push((i, scn.objects[i].physic_object.pos));
        } else if name.contains("enemy1_") {
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
        } else if name.contains("light1_") {
            eng.lights.push(Light::new(LightType::Spot));
            let lastltmt = eng.lights.last_mut().unwrap();
            lastltmt.pos = scn.objects[i].physic_object.pos;
            lastltmt.rot = Vec3 {
                x: -1.5708,
                y: 0.0,
                z: 0.0,
            }
            .to_quat();
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
        if name.contains("cover_") {
            scn.objects[i].mesh.ubo[52] = 1.0;
        }
    }

    eng.render.shadow_map_resolution = 512;

    //eng.render.resolution_scale = 0.75;
    eng.cameras[0].physic_object.solid = false;
    eng.cameras[0].physic_object.gravity = false;
    println!(
        "shadowmapresolution(ignore if shadowmaps are off): {}",
        eng.render.shadow_map_resolution
    );

    eng.render.resolution_scale = 0.6;

    let audiofl = [
        "assets/audio/bell.mp3",
        "assets/audio/paper.mp3",
        "assets/audio/type.mp3",
        "assets/audio/walking.mp3",
        "assets/audio/knife.mp3",
        "assets/audio/cityambient.mp3",
        "assets/audio/shutter.mp3",
        "assets/audio/paper2.mp3",
    ];

    let mut speakers = Vec::new();

    for fl in audiofl {
        let mut sp = Speaker::new(&mut eng, fl);
        sp.volume = 0.75;
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

    let mut session = GameSession {
        show_dbg_stat: false,
        eng,
        scn,
        viewport,
        ltviewport,
        ltsviewport,
        fpscnt,
        text,
        menu_text,
        keyimg,
        white_background: wbg,
        black_background: bbg,
        menu_background: menubg,
        logo: logoplane,
        speech_bubble_background: spbg,
        awarness_signs: awaspr,
        ui_scale: 1.0,
        pivot,
        tbtplates,
        enemies,
        physics_objects,
        actual_plate_id: 0,
        awareness: Awarness::Unseen(0),
        max_awareness: 0,
        making_step: false,
        object_holding: None,
        current_dir: Vec3::new(),
        action_cooldown: 0.0,
        returning_to_game: false,
        frame_count: 0,
        speakers: speakers,
        keyboard_bindings: KeyboardBindings {
            arrows: [
                Key {
                    keycode: 40,
                    key: "S".to_string(),
                },
                Key {
                    keycode: 44,
                    key: "W".to_string(),
                },
                Key {
                    keycode: 25,
                    key: "D".to_string(),
                },
                Key {
                    keycode: 22,
                    key: "A".to_string(),
                },
            ],
            abxy: [
                Key {
                    keycode: 26,
                    key: "E".to_string(),
                },
                Key {
                    keycode: 0,
                    key: String::new(),
                },
                Key {
                    keycode: 0,
                    key: String::new(),
                },
                Key {
                    keycode: 0,
                    key: String::new(),
                },
            ],
            start: Key {
                keycode: 49,
                key: String::new(),
            },
            select: Key {
                keycode: 0,
                key: String::new(),
            },
        },
        controller_mappings: ControllerMappings {
            arrows: [12, 10, 11, 13],
            arrows_key: [
                "d".to_string(),
                "t".to_string(),
                "r".to_string(),
                "l".to_string(),
            ],
            abxy: [0, 1, 2, 3],
            abxy_key: [
                "a".to_string(),
                "x".to_string(),
                "b".to_string(),
                "y".to_string(),
            ],
            axis: [0, 1, 2, 3],
            bumpers: [4, 5],
            start: 7,
            start_key: "t".to_string(),
            select: 6,
            select_key: "s".to_string(),
        },
        input_type: super::InputType::Keyboard,
        action_list: [None, None, None, None, None],
        active_action: None,
        chosen_language,
        text_content,
        game_state: crate::game::GameState::InGame,
        resize_cnt: 0,
        tredr: 0.0,
        exit: false,
        gamepadsel: 0,
        submenu: crate::game::SubMenu::Main,
        use_rt_lighting: true,
        current_time: 0.0,
        tmcoef: 1.0,
        bwgcoef: 1.0,
    };

    load_settings(&mut session);
    session
}

fn configure_actor(object: &mut crate::engine::physics::physics::PhysicsObject) {
    object.compute_torque = false;
    //object.gravity = false;
    object.step_height = 0.25;
    object.is_static = false;
    object.compute_force = false;
}
