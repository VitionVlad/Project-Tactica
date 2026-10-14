#![allow(dead_code)]
#![allow(unused_variables)]

use crate::engine::render::render::{MaterialShaders, Texture, Vertexes};

use super::{
    engine::Engine,
    image::Image,
    material::Material,
    math::{vec2::Vec2, vec3::Vec3, vec4::Vec4},
    model::Model,
    object::Object,
    plane::PLANEUI,
};

#[derive(Clone)]
pub struct Clickzone {
    pos1: Vec2,
    pos2: Vec2,
}

impl Clickzone {
    pub fn check(&self, mouse: Vec2) -> bool {
        if mouse.x >= self.pos1.x
            && mouse.x <= self.pos2.x
            && mouse.y >= self.pos1.y
            && mouse.y <= self.pos2.y
        {
            return true;
        }
        return false;
    }
}

pub struct UIplane {
    pub object: Object,
    pub clickzone: Clickzone,
    pub signal: bool,
    pub allow_when_mouse_locked: bool,
    pub ubo_index: usize,
    pub signal_on_value: f32,
    pub signal_off_value: f32,
}

impl UIplane {
    pub fn new(
        eng: &mut Engine,
        mat: Material,
        images: Vec<Image>,
        usage: super::render::render::MeshUsage,
    ) -> UIplane {
        let model = Model::new(&eng, PLANEUI.to_vec());
        UIplane {
            object: Object::new(eng, model, mat, images, usage, true, "".to_string(), vec![]),
            clickzone: Clickzone {
                pos1: Vec2::new(),
                pos2: Vec2::new(),
            },
            signal: false,
            allow_when_mouse_locked: false,
            ubo_index: 50,
            signal_on_value: 1.0f32,
            signal_off_value: 0.0f32,
        }
    }
    pub fn new_blank() -> UIplane {
        UIplane {
            object: Object::new_blank(),
            clickzone: Clickzone {
                pos1: Vec2::new(),
                pos2: Vec2::new(),
            },
            signal: false,
            allow_when_mouse_locked: false,
            ubo_index: 50,
            signal_on_value: 1.0f32,
            signal_off_value: 0.0f32,
        }
    }
    pub fn new_from_file(eng: &mut Engine, mat: Material, paths: Vec<String>) -> UIplane {
        let image = Image::new_from_files(eng, paths);
        let model = Model::new(&eng, PLANEUI.to_vec());
        UIplane {
            object: Object::new(
                eng,
                model,
                mat,
                vec![image],
                super::render::render::MeshUsage::LightingPass,
                true,
                "".to_string(),
                vec![],
            ),
            clickzone: Clickzone {
                pos1: Vec2::new(),
                pos2: Vec2::new(),
            },
            signal: false,
            allow_when_mouse_locked: false,
            ubo_index: 50,
            signal_on_value: 1.0f32,
            signal_off_value: 0.0f32,
        }
    }
    pub fn exec(&mut self, eng: &mut Engine) -> bool {
        self.clickzone.pos1.x = self.object.physic_object.pos.x;
        self.clickzone.pos1.y = self.object.physic_object.pos.y;
        self.clickzone.pos2.x = self.object.physic_object.pos.x + self.object.physic_object.scale.x;
        self.clickzone.pos2.y = self.object.physic_object.pos.y + self.object.physic_object.scale.y;
        let btst = self.clickzone.check(Vec2 {
            x: eng.control.xpos as f32,
            y: eng.control.ypos as f32,
        });
        if self.signal {
            if btst
                && (self.allow_when_mouse_locked
                    || (!self.allow_when_mouse_locked && !eng.control.mouse_lock))
                && self.object.draw
            {
                self.object.mesh.ubo[self.ubo_index] = self.signal_on_value;
            } else {
                self.object.mesh.ubo[self.ubo_index] = self.signal_off_value;
            }
        }
        self.object.exec(eng);
        return btst;
    }
}

#[derive(Clone)]
pub struct UItext {
    plane: Model,
    pub font: Vec<Image>,
    pub symbols: Vec<u8>,
    pub planes: Vec<Object>,
    pub symbol_number: u32,
    pub material: Material,
    pub size: Vec2,
    pub pos: Vec3,
    pub clickzone: Clickzone,
    pub signal: bool,
    pub per_symbol: bool,
    pub rotation: Vec4,
    pub rotate_each_character: bool,
    pub auto_recalculate_clickzone: bool,
    pub allow_when_mouse_locked: bool,
    pub draw: bool,
    pub symbol_pressed: u8,
    pub symbol_index: usize,
    pub ubo_index: usize,
    pub signal_on_value: f32,
    pub signal_off_value: f32,
    pub new_line_symbol: u8,
    pub max_text_width: u32,
    pub next_line_on_whitespace: bool,
    blank: bool,
    usage: super::render::render::MeshUsage,
}

impl UItext {
    fn normalized_rotation(rotation: Vec4) -> Vec4 {
        let magnitude = rotation.magnitude();
        if magnitude <= f32::EPSILON {
            Vec4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            }
        } else {
            Vec4 {
                x: rotation.x / magnitude,
                y: rotation.y / magnitude,
                z: rotation.z / magnitude,
                w: rotation.w / magnitude,
            }
        }
    }

    fn rotate_point(point: Vec3, pivot: Vec3, rotation: Vec4) -> Vec3 {
        let offset = Vec4 {
            x: point.x - pivot.x,
            y: point.y - pivot.y,
            z: point.z - pivot.z,
            w: 0.0,
        };
        let rotated = rotation.multiply(offset).multiply(rotation.conjugate());
        Vec3 {
            x: pivot.x + rotated.x,
            y: pivot.y + rotated.y,
            z: pivot.z + rotated.z,
        }
    }

    fn rotated_bounds(pos: Vec3, size: Vec2, pivot: Vec3, rotation: Vec4) -> (Vec2, Vec2) {
        let corners = [
            pos,
            Vec3 {
                x: pos.x + size.x,
                y: pos.y,
                z: pos.z,
            },
            Vec3 {
                x: pos.x,
                y: pos.y + size.y,
                z: pos.z,
            },
            Vec3 {
                x: pos.x + size.x,
                y: pos.y + size.y,
                z: pos.z,
            },
        ];
        let first = Self::rotate_point(corners[0], pivot, rotation);
        let mut min = Vec2 {
            x: first.x,
            y: first.y,
        };
        let mut max = min;
        for corner in corners.iter().skip(1) {
            let point = Self::rotate_point(*corner, pivot, rotation);
            min.x = min.x.min(point.x);
            min.y = min.y.min(point.y);
            max.x = max.x.max(point.x);
            max.y = max.y.max(point.y);
        }
        (min, max)
    }

    pub fn new(
        eng: &mut Engine,
        mat: Material,
        images: Vec<Image>,
        symbols: &str,
        usage: super::render::render::MeshUsage,
    ) -> UItext {
        UItext {
            plane: Model::new(&eng, PLANEUI.to_vec()),
            font: images,
            symbols: symbols.as_bytes().to_vec(),
            planes: vec![],
            symbol_number: symbols.len() as u32,
            material: mat,
            size: Vec2 { x: 20.0, y: 40.0 },
            pos: Vec3::new(),
            clickzone: Clickzone {
                pos1: Vec2::new(),
                pos2: Vec2::new(),
            },
            signal: false,
            per_symbol: true,
            rotation: Vec4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            rotate_each_character: false,
            auto_recalculate_clickzone: true,
            allow_when_mouse_locked: false,
            blank: false,
            symbol_pressed: b' ',
            symbol_index: 0,
            ubo_index: 48,
            signal_on_value: 1.0f32,
            signal_off_value: 0.0f32,
            new_line_symbol: b'\n',
            max_text_width: 0,
            next_line_on_whitespace: false,
            draw: true,
            usage: usage,
        }
    }
    pub fn new_blank() -> UItext {
        UItext {
            plane: Model {
                vertexbuf: Vertexes {
                    modelid: 0,
                    destroyed: true,
                },
                points: [Vec3::new(), Vec3::new()],
            },
            font: vec![Image {
                textures: Texture {
                    texid: 0,
                    destroyed: true,
                },
            }],
            symbols: "".as_bytes().to_vec(),
            planes: vec![],
            symbol_number: 0,
            material: Material {
                material_shaders: MaterialShaders {
                    materialid: 0,
                    destroyed: true,
                },
            },
            size: Vec2 { x: 20.0, y: 40.0 },
            pos: Vec3::new(),
            clickzone: Clickzone {
                pos1: Vec2::new(),
                pos2: Vec2::new(),
            },
            signal: false,
            per_symbol: true,
            rotation: Vec4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            rotate_each_character: false,
            auto_recalculate_clickzone: true,
            allow_when_mouse_locked: false,
            blank: true,
            symbol_pressed: b' ',
            symbol_index: 0,
            ubo_index: 48,
            signal_on_value: 1.0f32,
            signal_off_value: 0.0f32,
            new_line_symbol: b'\n',
            max_text_width: 0,
            next_line_on_whitespace: false,
            draw: false,
            usage: super::render::render::MeshUsage::PostPass,
        }
    }
    pub fn new_from_file(
        eng: &mut Engine,
        mat: Material,
        image: &str,
        symbols: &str,
        usage: super::render::render::MeshUsage,
    ) -> UItext {
        let img = Image::new_from_files(eng, vec![image.to_string()]);
        UItext {
            plane: Model::new(&eng, PLANEUI.to_vec()),
            font: vec![img],
            symbols: symbols.as_bytes().to_vec(),
            planes: vec![],
            symbol_number: symbols.len() as u32,
            material: mat,
            size: Vec2 { x: 20.0, y: 40.0 },
            pos: Vec3::new(),
            clickzone: Clickzone {
                pos1: Vec2::new(),
                pos2: Vec2::new(),
            },
            signal: false,
            per_symbol: true,
            rotation: Vec4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            rotate_each_character: false,
            auto_recalculate_clickzone: true,
            allow_when_mouse_locked: false,
            blank: false,
            symbol_pressed: b' ',
            symbol_index: 0,
            ubo_index: 48,
            signal_on_value: 1.0f32,
            signal_off_value: 0.0f32,
            new_line_symbol: b'\n',
            max_text_width: 0,
            next_line_on_whitespace: false,
            draw: true,
            usage: usage,
        }
    }
    pub fn exec(&mut self, eng: &mut Engine, text: &str) -> bool {
        if !self.blank {
            let bt = text.as_bytes();
            let mut mx: u32 = 1;
            let mut my: u32 = 1;
            let mut cx: u32 = 0;
            for i in 0..bt.len() {
                cx += 1;
                if bt[i] == self.new_line_symbol {
                    my += 1;
                    if cx - 1 >= mx {
                        mx = cx - 1;
                    }
                    cx = 1;
                }
            }
            if cx - 1 >= mx {
                mx = cx - 1;
            }
            self.clickzone.pos1.x = self.pos.x;
            self.clickzone.pos1.y = self.pos.y;
            self.clickzone.pos2.x = self.pos.x + self.size.x * (mx as f32 + 1.0);
            self.clickzone.pos2.y = self.pos.y + self.size.y * (my as f32);
            let rotation = Self::normalized_rotation(self.rotation);
            let rotation_is_identity = rotation.x.abs() <= f32::EPSILON
                && rotation.y.abs() <= f32::EPSILON
                && rotation.z.abs() <= f32::EPSILON;
            let clickzone_pos = Vec3 {
                x: self.clickzone.pos1.x,
                y: self.clickzone.pos1.y,
                z: self.pos.z,
            };
            let clickzone_size = Vec2 {
                x: self.clickzone.pos2.x - self.clickzone.pos1.x,
                y: self.clickzone.pos2.y - self.clickzone.pos1.y,
            };
            let mut rotated_symbol_bounds: Option<(Vec2, Vec2)> = None;
            if self.auto_recalculate_clickzone && !rotation_is_identity {
                if self.rotate_each_character {
                    rotated_symbol_bounds = Some((self.clickzone.pos1, self.clickzone.pos2));
                } else {
                    let (min, max) = Self::rotated_bounds(
                        clickzone_pos,
                        clickzone_size,
                        clickzone_pos,
                        rotation,
                    );
                    self.clickzone.pos1 = min;
                    self.clickzone.pos2 = max;
                }
            }
            let mut btst = self.clickzone.check(Vec2 {
                x: eng.control.xpos as f32,
                y: eng.control.ypos as f32,
            });
            let mut lbtst = btst;
            if self.planes.len() < bt.len() {
                for i in self.planes.len()..bt.len() {
                    self.planes.push(Object::new(
                        eng,
                        self.plane,
                        self.material,
                        self.font.clone(),
                        self.usage,
                        true,
                        "".to_string(),
                        vec![],
                    ));
                }
            }
            for i in 0..self.planes.len() {
                self.planes[i].draw = false;
            }
            let mut posy: f32 = self.pos.y;
            let mut bp: usize = 0;
            let mut bp2 = 0;
            if self.draw {
                for i in 0..bt.len() {
                    bp2 += 1;
                    for j in 0..self.symbols.len() {
                        if bt[i] == self.symbols[j] {
                            self.planes[i].draw = true;
                            self.planes[i].mesh.ubo[self.ubo_index] = self.symbol_number as f32;
                            self.planes[i].mesh.ubo[self.ubo_index + 1] = j as f32;
                            self.planes[i].physic_object.scale.x = self.size.x;
                            self.planes[i].physic_object.scale.y = self.size.y;
                            self.planes[i].physic_object.scale.z = 1.0;
                            let original_glyph_pos = Vec3 {
                                x: self.pos.x + ((i as i32 - bp as i32) as f32) * self.size.x,
                                y: posy,
                                z: self.pos.z,
                            };
                            let pivot = if self.rotate_each_character {
                                Vec3 {
                                    x: original_glyph_pos.x + self.size.x * 0.5,
                                    y: original_glyph_pos.y + self.size.y * 0.5,
                                    z: original_glyph_pos.z,
                                }
                            } else {
                                clickzone_pos
                            };
                            let glyph_pos = if rotation_is_identity {
                                original_glyph_pos
                            } else {
                                Self::rotate_point(original_glyph_pos, pivot, rotation)
                            };
                            self.planes[i].physic_object.pos = glyph_pos;
                            self.planes[i].physic_object.rot = rotation;

                            if self.per_symbol {
                                if self.auto_recalculate_clickzone && !rotation_is_identity {
                                    let (min, max) = Self::rotated_bounds(
                                        original_glyph_pos,
                                        self.size,
                                        pivot,
                                        rotation,
                                    );
                                    self.clickzone.pos1 = min;
                                    self.clickzone.pos2 = max;
                                } else {
                                    self.clickzone.pos1.x = glyph_pos.x;
                                    self.clickzone.pos1.y = glyph_pos.y;
                                    self.clickzone.pos2.x = glyph_pos.x + self.size.x;
                                    self.clickzone.pos2.y = glyph_pos.y + self.size.y;
                                }
                                lbtst = self.clickzone.check(Vec2 {
                                    x: eng.control.xpos as f32,
                                    y: eng.control.ypos as f32,
                                });
                                if lbtst {
                                    self.symbol_pressed = self.symbols[j];
                                    self.symbol_index = i;
                                    btst = true;
                                }
                            }
                            if self.auto_recalculate_clickzone
                                && self.rotate_each_character
                                && !rotation_is_identity
                            {
                                let (min, max) = Self::rotated_bounds(
                                    original_glyph_pos,
                                    self.size,
                                    pivot,
                                    rotation,
                                );
                                if let Some((bounds_min, bounds_max)) = &mut rotated_symbol_bounds {
                                    bounds_min.x = bounds_min.x.min(min.x);
                                    bounds_min.y = bounds_min.y.min(min.y);
                                    bounds_max.x = bounds_max.x.max(max.x);
                                    bounds_max.y = bounds_max.y.max(max.y);
                                } else {
                                    rotated_symbol_bounds = Some((min, max));
                                }
                            }
                            if self.signal
                                && lbtst
                                && (self.allow_when_mouse_locked
                                    || (!self.allow_when_mouse_locked && !eng.control.mouse_lock))
                            {
                                self.planes[i].mesh.ubo[self.ubo_index + 2] = self.signal_on_value;
                            } else {
                                self.planes[i].mesh.ubo[self.ubo_index + 2] = self.signal_off_value;
                            }
                            break;
                        }
                        if bt[i] == self.new_line_symbol {
                            posy += self.size.y;
                            bp = i + 1;
                            bp2 = 0;
                            break;
                        }
                        if bp2 > self.max_text_width
                            && self.max_text_width != 0
                            && ((bt[i] == b' ' && self.next_line_on_whitespace)
                                || !self.next_line_on_whitespace)
                        {
                            posy += self.size.y;
                            bp = i + 1;
                            bp2 = 0;
                        }
                    }
                }
            }
            if let Some((min, max)) = rotated_symbol_bounds {
                self.clickzone.pos1 = min;
                self.clickzone.pos2 = max;
                btst = self.clickzone.check(Vec2 {
                    x: eng.control.xpos as f32,
                    y: eng.control.ypos as f32,
                });
            }
            for i in 0..self.planes.len() {
                self.planes[i].exec(eng);
            }
            return btst;
        }
        return false;
    }
}