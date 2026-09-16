use std::println;

use crate::engine::{engine::Engine, image::Image, loader::rw::readfs, material::Material, math::vec3::Vec3, physics::physics::PhysicsObject, render::render::TextureFormat, scene::Scene, ui::{UIplane, UItext}};

mod engine;

pub struct Pivot{
  pub id: usize,
  pub tbtmode: bool,
  pub start_pos: Vec3,
}

#[derive(PartialEq, Clone, Eq, Hash, Ord, PartialOrd, Debug)]
pub enum Awarness{
  Unseen(u8),
  Suspected(i8),
  Detected(i8),
  Compromised(i8)
}

pub struct Enemy{
  pub id: usize,
  pub direction: Vec3,
  pub plate_id: Option<usize>,
  pub alive: bool,
  pub start_pos: Vec3,
  pub initial_tile_id: Option<usize>,
}

pub struct TBTplate{
  pub id: usize,
  pub safe: bool,
  pub enemy_id: Option<usize>,
}

fn tile_test(pivot: &PhysicsObject, plate: &PhysicsObject) -> bool {
  let pivot_collider = &pivot.collider;
  let plate_collider = &plate.collider;
  let overlaps_x = pivot_collider.aabb_min.x <= plate_collider.aabb_max.x
    && plate_collider.aabb_min.x <= pivot_collider.aabb_max.x;
  let overlaps_z = pivot_collider.aabb_min.z <= plate_collider.aabb_max.z
    && plate_collider.aabb_min.z <= pivot_collider.aabb_max.z;

  overlaps_x && overlaps_z
}

fn tile_test_point(point: &Vec3, plate: &PhysicsObject) -> bool {
  let plate_collider = &plate.collider;
  let overlaps_x = point.x <= plate_collider.aabb_max.x
    && plate_collider.aabb_min.x <= point.x;
  let overlaps_z = point.z <= plate_collider.aabb_max.z
    && plate_collider.aabb_min.z <= point.z;

  overlaps_x && overlaps_z
}

fn main() {
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
    //let imgf = readfs("shaders/fimg");

    let matt = Material::new(
        &eng,
        vert.clone(),
        textf,
        vec![],
        [
            engine::render::render::CullMode::CullModeNone,
            engine::render::render::CullMode::CullModeNone,
        ],
    );
    let mat = Material::new(
        &eng,
        vert.clone(),
        frag,
        vec![],
        [
            engine::render::render::CullMode::CullModeNone,
            engine::render::render::CullMode::CullModeNone,
        ],
    );
    let lightmat = Material::new(
        &eng,
        vert.clone(),
        lfrag,
        vec![],
        [
            engine::render::render::CullMode::CullModeNone,
            engine::render::render::CullMode::CullModeNone,
        ],
    );
    let matgeneral = Material::new(
        &eng,
        dvert.clone(),
        dfrag,
        shadow.clone(),
        [
            engine::render::render::CullMode::CullModeBackBit,
            engine::render::render::CullMode::CullModeFrontBit,
        ],
    );

    let black = Image::new_color(&eng, [11, 23, 40, u8::MAX]);

    let mut viewport = UIplane::new(&mut eng, mat, vec![black], engine::render::render::MeshUsage::PostPass);
    viewport.object.physic_object.pos.z = 1.0;
    viewport.signal = false;

    let mut fpscnt = UItext::new_from_file(
      &mut eng,
      matt,
      "assets/lat.png",
      "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`~$% ",
      engine::render::render::MeshUsage::PostPass
    );
    fpscnt.new_line_symbol = b'|';

    let mut scn = Scene::load_from_gltf(&mut eng, "assets/test.glb", matgeneral, false, 0.2f32);

    //println!("{}, {}, {}, decomp_size: {}", scn.voxel_representation.size[0], scn.voxel_representation.size[1], scn.voxel_representation.size[2], scn.voxel_representation.data.len());

    //let d3d = unpack_svo_to_texture(&scn.voxel_representation.data, 5, 32);

    //let black3d = Image::new(&eng, scn.voxel_representation.size, scn.voxel_representation.data.clone(), true, TextureFormat::R8g8b8a8Unorm, false);

    //let black3d = Image::new(&eng, [scn.voxel_representation.data.len() as u32, 1, 1], scn.voxel_representation.data.clone(), false, TextureFormat::R8, false);

    let black3d = Image::new(&eng, [1, 1, 1], vec![u8::MAX, u8::MAX, u8::MAX, u8::MAX], true, TextureFormat::R8g8b8a8Unorm, false);

    let bluenoise = Image::new_from_files(&eng, vec!["assets/noise.png".to_string()]);

    let mut ltviewport = UIplane::new(&mut eng, lightmat, vec![black3d, bluenoise], engine::render::render::MeshUsage::LightingPass);
    ltviewport.object.physic_object.pos.z = 1.0;
    ltviewport.signal = false;

    scn.voxel_representation.data.resize(0, 0);

    const SPEED: f32 = 25.0f32;

    let mut tm = 0.0;

    //for i in 0..scn.objects.len() {
    //  scn.objects[i].draw_distance = f32::MAX;
    //  scn.objects[i].render_in_behind = false;
    //}

    eng.render.shadow_map_count = 0;
    eng.used_light_count = 1;

    eng.lights[0].shadow = false;
    eng.lights[0].light_type = engine::light::LightType::Spot;
    eng.lights[0].pos.x = 0.0f32;
    eng.lights[0].pos.y = 5.0f32;
    eng.lights[0].pos.z = 0.0f32;
    eng.lights[0].color.x = 1.25f32;
    eng.lights[0].color.y = 1.25f32;
    eng.lights[0].color.z = 1.25f32;

    eng.cameras[0].physic_object.pos.y = 5.0;
    eng.cameras[0].fov = 30.0;

    let mut fcnt = 0u32;

    //eng.render.resolution_scale = 0.5;

    println!("shadowmapresolution(ignore if shadowmaps are off): {}", eng.render.shadow_map_resolution);

    let mut pivot = Pivot{id: 0, tbtmode: false, start_pos: Vec3{x: 0.0, y: 0.0, z: 0.0}};

    let mut tbtplates = Vec::new();

    let mut enemies = Vec::new();

    let mut actual_plate_id = 0;

    let awarness_max_level = 2;

    for i in 0..scn.objects.len(){
      if scn.objects[i].name.contains("pivot"){
        println!("pivot id = {}", i);
        pivot.id = i;
        pivot.start_pos = scn.objects[pivot.id].physic_object.pos;
        scn.objects[pivot.id].physic_object.compute_torque = false;
        scn.objects[pivot.id].physic_object.gravity = false;
        scn.objects[pivot.id].physic_object.is_static = false;
      }else if scn.objects[i].name.contains("tactica_"){
        println!("TBT plate found with name {}", scn.objects[i].name);
        tbtplates.push(TBTplate{id: i, safe: false, enemy_id: None});
      }else if scn.objects[i].name.contains("safe_"){
        println!("safe plate found with name {}", scn.objects[i].name);
        tbtplates.push(TBTplate{id: i, safe: true, enemy_id: None});
      }else if scn.objects[i].name.contains("enemy"){
        println!("enemy found with name {}", scn.objects[i].name);
        enemies.push(Enemy{id: i, direction: Vec3{x: 0.0, y: 0.0, z: -1.0}, plate_id: None, alive: true, start_pos: scn.objects[i].physic_object.pos, initial_tile_id: None});
        scn.objects[i].physic_object.compute_torque = false;
        scn.objects[i].physic_object.gravity = false;
        scn.objects[i].physic_object.is_static = false;
      }
    }

    eng.cameras[0].physic_object.solid = false;
    eng.cameras[0].physic_object.gravity = false;

    let mut level_awarness = Awarness::Unseen(0);

    let mut making_step = false;

    let keycodes = vec![
      40, //w
      44, //s
      25, //a
      22, //d
      48, //space
    ];

    while eng.work(){
      if tm > 0.0{
        tm -= eng.logic_frametime;
      }

      //step logic
      if making_step{
        let mut enemdet = false;
        making_step = false;

        for plates in tbtplates.iter_mut(){
          plates.enemy_id = None;
        }
        for enemies in enemies.iter_mut(){
          let dist = (scn.objects[enemies.id].physic_object.pos - scn.objects[pivot.id].physic_object.pos).length_squared().sqrt();
          if dist < 1.5f32{
            enemdet = true;
            match level_awarness {
              Awarness::Unseen(x) => {
                if x < awarness_max_level as u8{
                  level_awarness = Awarness::Unseen(x+1);
                }else{
                  level_awarness = Awarness::Suspected(0);
                }
              }
              Awarness::Suspected(x) => {
                if x < awarness_max_level{
                  level_awarness = Awarness::Suspected(x+1);
                }else{
                  level_awarness = Awarness::Detected(0);
                }
              }
              Awarness::Detected(x) => {
                if x < awarness_max_level{
                  level_awarness = Awarness::Detected(x+1);
                }else{
                  level_awarness = Awarness::Compromised(0);
                }
              }
              Awarness::Compromised(x) => {
                level_awarness = Awarness::Compromised(x+1);
              }
            }
          }
          if enemies.alive && enemies.plate_id.is_some() && enemies.plate_id.unwrap() == actual_plate_id{
            level_awarness = Awarness::Compromised(awarness_max_level);
          }
        }

        for i in 0..enemies.len(){
          if enemies[i].alive{
            let enemy = &mut enemies[i];
            if enemy.plate_id.is_none(){
              for j in 0..tbtplates.len() {
                if tile_test(&scn.objects[enemy.id].physic_object, &scn.objects[tbtplates[j].id].physic_object) && !tbtplates[j].safe {
                  enemies[i].plate_id = Some(j);
                  tbtplates[j].enemy_id = Some(i);
                  if enemies[i].initial_tile_id.is_none(){
                    enemies[i].initial_tile_id = Some(j);
                  }
                  break;
                }
              }
            }else if enemy.plate_id.is_some(){
              let mut found_dir = false;
              for j in 0..tbtplates.len() {
                if tile_test_point(&(scn.objects[enemy.id].physic_object.pos+enemy.direction), &scn.objects[tbtplates[j].id].physic_object) && !tbtplates[j].safe {
                  enemy.plate_id = Some(j);
                  tbtplates[j].enemy_id = Some(i);
                  found_dir = true;
                  break;
                }
              }
              if !found_dir{
                for j in 0..tbtplates.len() {
                  if tile_test_point(&(scn.objects[enemy.id].physic_object.pos-enemy.direction), &scn.objects[tbtplates[j].id].physic_object) && !tbtplates[j].safe {
                    enemy.plate_id = Some(j);
                    tbtplates[j].enemy_id = Some(i);
                    enemy.direction = -enemy.direction;
                    break;
                  }
                }
              }
            }
          }
        }

        if !enemdet{
          match level_awarness {
              Awarness::Unseen(x) => {
                if x > 0{
                  level_awarness = Awarness::Unseen(x-1);
                }else{
                  level_awarness = Awarness::Unseen(0);
                }
              }
              Awarness::Suspected(x) => {
                if x > 0{
                  level_awarness = Awarness::Suspected(x-1);
                }else{
                  level_awarness = Awarness::Unseen(awarness_max_level as u8);
                }
              }
              Awarness::Detected(x) => {
                if x > 0{
                  level_awarness = Awarness::Detected(x-1);
                }else{
                  level_awarness = Awarness::Suspected(awarness_max_level);
                }
              }
              Awarness::Compromised(x) => {
                if x > 0{
                  level_awarness = Awarness::Compromised(x-1);
                }else{
                  level_awarness = Awarness::Detected(awarness_max_level);
                }
              }
            }
        }
      }

      for enemy in enemies.iter_mut(){
        if enemy.alive && enemy.plate_id.is_some(){
          let plate_id = enemy.plate_id.unwrap();
          scn.objects[enemy.id].physic_object.acceleration.x = -(scn.objects[enemy.id].physic_object.pos.x - scn.objects[tbtplates[plate_id].id].physic_object.pos.x)*10.0;
          scn.objects[enemy.id].physic_object.acceleration.z = -(scn.objects[enemy.id].physic_object.pos.z - scn.objects[tbtplates[plate_id].id].physic_object.pos.z)*10.0;
        }
      }

      //controls
      if pivot.tbtmode{

        let deltapos = scn.objects[tbtplates[actual_plate_id].id].physic_object.pos - scn.objects[pivot.id].physic_object.pos;
        if deltapos.length_squared().sqrt() < 0.01{
          if tbtplates[actual_plate_id].safe {
            pivot.tbtmode = false;
          }

          let mut direction = [
            None,
            None,
            None,
            None,
          ];

          for i in 0..tbtplates.len(){
            let plate = &tbtplates[i];
            if tile_test_point(&(scn.objects[pivot.id].physic_object.pos+Vec3{x: 0.0, y: 0.0, z: 1.0}), &scn.objects[plate.id].physic_object){
              direction[0] = Some((i, plate.enemy_id.is_some()));
            }
            if tile_test_point(&(scn.objects[pivot.id].physic_object.pos+Vec3{x: 0.0, y: 0.0, z: -1.0}), &scn.objects[plate.id].physic_object){
              direction[1] = Some((i, plate.enemy_id.is_some()));
            }
            if tile_test_point(&(scn.objects[pivot.id].physic_object.pos+Vec3{x: 1.0, y: 0.0, z: 0.0}), &scn.objects[plate.id].physic_object){
              direction[2] = Some((i, plate.enemy_id.is_some()));
            }
            if tile_test_point(&(scn.objects[pivot.id].physic_object.pos+Vec3{x: -1.0, y: 0.0, z: 0.0}), &scn.objects[plate.id].physic_object){
              direction[3] = Some((i, plate.enemy_id.is_some()));
            }
          }

          if eng.control.get_key_state(keycodes[0]) && direction[0].is_some(){//w
            let dir = direction[0].unwrap();
            match dir.1{
              false => {
                actual_plate_id = dir.0;
              },
              true => {
                actual_plate_id = dir.0;
                enemies[tbtplates[actual_plate_id].enemy_id.unwrap()].alive = false;
                scn.objects[enemies[tbtplates[actual_plate_id].enemy_id.unwrap()].id].physic_object.pos.x = -1000.0;
                scn.objects[enemies[tbtplates[actual_plate_id].enemy_id.unwrap()].id].draw = false;
              }
            }
            making_step = true;
          }else if eng.control.get_key_state(keycodes[1]) && direction[1].is_some(){//s
            let dir = direction[1].unwrap();
            match dir.1{
              false => {
                actual_plate_id = dir.0;
              },
              true => {
                actual_plate_id = dir.0;
                enemies[tbtplates[actual_plate_id].enemy_id.unwrap()].alive = false;
                scn.objects[enemies[tbtplates[actual_plate_id].enemy_id.unwrap()].id].physic_object.pos.x = -1000.0;
                scn.objects[enemies[tbtplates[actual_plate_id].enemy_id.unwrap()].id].draw = false;
              }
            }
            making_step = true;
          }else if eng.control.get_key_state(keycodes[2]) && direction[2].is_some(){//a
            let dir = direction[2].unwrap();
            match dir.1{
              false => {
                actual_plate_id = dir.0;
              },
              true => {
                actual_plate_id = dir.0;
                enemies[tbtplates[actual_plate_id].enemy_id.unwrap()].alive = false;
                scn.objects[enemies[tbtplates[actual_plate_id].enemy_id.unwrap()].id].physic_object.pos.x = -1000.0;
                scn.objects[enemies[tbtplates[actual_plate_id].enemy_id.unwrap()].id].draw = false;
              }
            }
            making_step = true;
          }else if eng.control.get_key_state(keycodes[3]) && direction[3].is_some(){//d
            let dir = direction[3].unwrap();
            match dir.1{
              false => {
                actual_plate_id = dir.0;
              },
              true => {
                actual_plate_id = dir.0;
                enemies[tbtplates[actual_plate_id].enemy_id.unwrap()].alive = false;
                scn.objects[enemies[tbtplates[actual_plate_id].enemy_id.unwrap()].id].physic_object.pos.x = -1000.0;
                scn.objects[enemies[tbtplates[actual_plate_id].enemy_id.unwrap()].id].draw = false;
              }
            }
            making_step = true;
          }else if eng.control.get_key_state(keycodes[4]) && tm <= 0.0{ //skip step
            making_step = true;
            tm = 0.5;
          }
        }

        scn.objects[pivot.id].physic_object.acceleration.x = -(scn.objects[pivot.id].physic_object.pos.x - scn.objects[tbtplates[actual_plate_id].id].physic_object.pos.x)*10.0;
        scn.objects[pivot.id].physic_object.acceleration.z = -(scn.objects[pivot.id].physic_object.pos.z - scn.objects[tbtplates[actual_plate_id].id].physic_object.pos.z)*10.0;
      }else{
        if eng.control.get_key_state(keycodes[0]){//w
          scn.objects[pivot.id].physic_object.acceleration.z += SPEED;
        }
        if eng.control.get_key_state(keycodes[1]){//s
          scn.objects[pivot.id].physic_object.acceleration.z -= SPEED;
        }
        if eng.control.get_key_state(keycodes[2]){//a
          scn.objects[pivot.id].physic_object.acceleration.x += SPEED;
        }
        if eng.control.get_key_state(keycodes[3]){//d
          scn.objects[pivot.id].physic_object.acceleration.x -= SPEED;
        }
      }

      eng.cameras[0].physic_object.pos.x = scn.objects[pivot.id].physic_object.pos.x + 5.0f32;
      eng.cameras[0].physic_object.pos.y = scn.objects[pivot.id].physic_object.pos.y + 7.5f32;
      eng.cameras[0].physic_object.pos.z = scn.objects[pivot.id].physic_object.pos.z + 10.0f32;
      eng.cameras[0].look_at(scn.objects[pivot.id].physic_object.pos+Vec3{x: 0.0, y: 0.5, z: 0.0});

      let fpstxt = format!("CPU_fps:{}|GPU_fps:{}|detection_level: {:?}", eng.cpu_fps, eng.gpu_fps, level_awarness);
      fpscnt.size.x = 15_f32;
      fpscnt.size.y = 30_f32;
      fpscnt.pos.x = 0.0;
      fpscnt.pos.y = 0.0;
      fpscnt.pos.z = 0.1;
      fpscnt.draw = true;
      fpscnt.exec(&mut eng, &fpstxt);

      scn.exec(&mut eng);
      ltviewport.object.physic_object.scale.x = eng.render.resolution_x as f32 * eng.render.resolution_scale;
      ltviewport.object.physic_object.scale.y = eng.render.resolution_y as f32 * eng.render.resolution_scale;
      ltviewport.object.physic_object.pos.z = 0.9;
      ltviewport.object.mesh.ubo[48] = fcnt as f32;
      ltviewport.object.mesh.ubo[49] = scn.voxel_representation.size[0] as f32;
      ltviewport.object.mesh.ubo[50] = scn.voxel_representation.size[1] as f32;
      ltviewport.object.mesh.ubo[51] = scn.voxel_representation.size[2] as f32;
      ltviewport.object.mesh.ubo[52] = scn.voxel_representation.origin.x as f32;
      ltviewport.object.mesh.ubo[53] = scn.voxel_representation.origin.y as f32;
      ltviewport.object.mesh.ubo[54] = scn.voxel_representation.origin.z as f32;
      ltviewport.object.mesh.ubo[55] = scn.voxel_representation.voxel_size;
      ltviewport.exec(&mut eng);

      if !pivot.tbtmode {
        for i in 0..tbtplates.len() {
          if tile_test(&scn.objects[pivot.id].physic_object, &scn.objects[tbtplates[i].id].physic_object) && !tbtplates[i].safe {
            //scn.objects[pivot.id].physic_object.pos.x = scn.objects[plate.id].physic_object.pos.x;
            //scn.objects[pivot.id].physic_object.pos.z = scn.objects[plate.id].physic_object.pos.z;
            actual_plate_id = i;
            pivot.tbtmode = true;
            break;
          }
        }
      }

      viewport.object.physic_object.scale.x = eng.render.resolution_x as f32;
      viewport.object.physic_object.scale.y = eng.render.resolution_y as f32;
      viewport.object.mesh.ubo[48] = fcnt as f32;
      viewport.object.mesh.ubo[49] = 0.0;
      viewport.object.mesh.ubo[50] = 0.2;
      viewport.object.mesh.ubo[51] = 0.18;
      viewport.exec(&mut eng);

      if level_awarness == Awarness::Compromised(awarness_max_level){
        level_awarness = Awarness::Unseen(0);
        pivot.tbtmode = false;
        making_step = false;
        scn.objects[pivot.id].physic_object.pos = pivot.start_pos;
        for enemy in enemies.iter_mut(){
          enemy.alive = true;
          scn.objects[enemy.id].physic_object.pos = enemy.start_pos;
          if enemy.initial_tile_id.is_some(){
            enemy.plate_id = enemy.initial_tile_id;
          }
        }
      }
      
      fcnt += 1;
      if fcnt > 100000{
        fcnt = 0;
      }
    }
    eng.end();
}