#![allow(dead_code)]
#![allow(unused_variables)]

use std::u8;

use crate::engine::{
    loader::{
        jsonparser::JsonF,
        rw::{readfs, writefs},
    },
    math::{ivec3::Ivec3, vec3::Vec3},
};

#[derive(Clone, Copy)]
pub struct Rledata {
    pub data: u8,
    pub location: u64,
    pub length: u64,
}

#[derive(Clone, Copy)]
pub struct VoxelizerSettings {
    pub size: Option<Ivec3>,
    pub origin: Option<Vec3>,
    pub voxel_size: f32,
}

impl VoxelizerSettings {
    pub fn new(voxel_size: f32) -> VoxelizerSettings {
        VoxelizerSettings {
            size: None,
            origin: None,
            voxel_size,
        }
    }
    pub fn new_with_size(voxel_size: f32, size: Ivec3) -> VoxelizerSettings {
        VoxelizerSettings {
            size: Some(size),
            origin: None,
            voxel_size,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Voxel {
    pos: Ivec3,
    color: [u8; 4],
}

#[derive(Clone)]
pub struct SVO {
    is_leaf: bool,
    color: [u8; 4],
    children: Vec<SVO>,
    octant_index: u8,
}

const LEAF_MARKER: u8 = 0;
const BRANCH_MARKER: u8 = 100;
const NODE_SIZE: usize = 5;

impl SVO {
    pub fn build_tree_recursive(
        voxels: Vec<Voxel>,
        boxmin: Ivec3,
        boxmax: Ivec3,
        current_level: u32,
        octant_index: u8,
    ) -> SVO {
        if current_level == 0 {
            for v in &voxels {
                if v.pos.x >= boxmin.x
                    && v.pos.x <= boxmax.x
                    && v.pos.y >= boxmin.y
                    && v.pos.y <= boxmax.y
                    && v.pos.z >= boxmin.z
                    && v.pos.z <= boxmax.z
                {
                    println!(
                        "Leaf voxel found at ({}, {}, {}) with color {:?}",
                        v.pos.x, v.pos.y, v.pos.z, v.color
                    );
                    return SVO {
                        is_leaf: true,
                        color: v.color,
                        children: Vec::new(),
                        octant_index: octant_index,
                    };
                }
            }
        }
        for v in &voxels {
            if v.pos.x >= boxmin.x
                && v.pos.x <= boxmax.x
                && v.pos.y >= boxmin.y
                && v.pos.y <= boxmax.y
                && v.pos.z >= boxmin.z
                && v.pos.z <= boxmax.z
            {
                let new_svo_boxes = vec![
                    Self::build_tree_recursive(
                        voxels.clone(),
                        boxmin,
                        Ivec3 {
                            x: (boxmin.x + boxmax.x) / 2,
                            y: (boxmin.y + boxmax.y) / 2,
                            z: (boxmin.z + boxmax.z) / 2,
                        },
                        current_level - 1,
                        0,
                    ),
                    Self::build_tree_recursive(
                        voxels.clone(),
                        Ivec3 {
                            x: (boxmin.x + boxmax.x) / 2,
                            y: boxmin.y,
                            z: boxmin.z,
                        },
                        Ivec3 {
                            x: boxmax.x,
                            y: (boxmin.y + boxmax.y) / 2,
                            z: (boxmin.z + boxmax.z) / 2,
                        },
                        current_level - 1,
                        1,
                    ),
                    Self::build_tree_recursive(
                        voxels.clone(),
                        Ivec3 {
                            x: boxmin.x,
                            y: (boxmin.y + boxmax.y) / 2,
                            z: boxmin.z,
                        },
                        Ivec3 {
                            x: (boxmin.x + boxmax.x) / 2,
                            y: boxmax.y,
                            z: (boxmin.z + boxmax.z) / 2,
                        },
                        current_level - 1,
                        2,
                    ),
                    Self::build_tree_recursive(
                        voxels.clone(),
                        Ivec3 {
                            x: (boxmin.x + boxmax.x) / 2,
                            y: (boxmin.y + boxmax.y) / 2,
                            z: boxmin.z,
                        },
                        Ivec3 {
                            x: boxmax.x,
                            y: boxmax.y,
                            z: (boxmin.z + boxmax.z) / 2,
                        },
                        current_level - 1,
                        3,
                    ),
                    Self::build_tree_recursive(
                        voxels.clone(),
                        Ivec3 {
                            x: boxmin.x,
                            y: boxmin.y,
                            z: (boxmin.z + boxmax.z) / 2,
                        },
                        Ivec3 {
                            x: (boxmin.x + boxmax.x) / 2,
                            y: (boxmin.y + boxmax.y) / 2,
                            z: boxmax.z,
                        },
                        current_level - 1,
                        4,
                    ),
                    Self::build_tree_recursive(
                        voxels.clone(),
                        Ivec3 {
                            x: (boxmin.x + boxmax.x) / 2,
                            y: boxmin.y,
                            z: (boxmin.z + boxmax.z) / 2,
                        },
                        Ivec3 {
                            x: boxmax.x,
                            y: (boxmin.y + boxmax.y) / 2,
                            z: boxmax.z,
                        },
                        current_level - 1,
                        5,
                    ),
                    Self::build_tree_recursive(
                        voxels.clone(),
                        Ivec3 {
                            x: boxmin.x,
                            y: (boxmin.y + boxmax.y) / 2,
                            z: (boxmin.z + boxmax.z) / 2,
                        },
                        Ivec3 {
                            x: (boxmin.x + boxmax.x) / 2,
                            y: boxmax.y,
                            z: boxmax.z,
                        },
                        current_level - 1,
                        6,
                    ),
                    Self::build_tree_recursive(
                        voxels.clone(),
                        Ivec3 {
                            x: (boxmin.x + boxmax.x) / 2,
                            y: (boxmin.y + boxmax.y) / 2,
                            z: (boxmin.z + boxmax.z) / 2,
                        },
                        boxmax,
                        current_level - 1,
                        7,
                    ),
                ];
                return SVO {
                    is_leaf: false,
                    color: [0, 0, 0, 0],
                    children: new_svo_boxes,
                    octant_index: octant_index,
                };
            }
        }
        SVO {
            is_leaf: true,
            color: [0, 0, 0, 0],
            children: Vec::new(),
            octant_index: 9,
        }
    }
    pub fn estimate_branch_size(&self) -> usize {
        if self.is_leaf {
            1 + 4
        } else {
            let mut size = 1 + 4;
            for child in &self.children {
                size += child.estimate_branch_size();
            }
            size
        }
    }
    pub fn pack_recursive(&self, packed_data: &mut Vec<u8>) {
        if self.is_leaf {
            packed_data.push(LEAF_MARKER);
            packed_data.extend_from_slice(&self.color);
        } else {
            packed_data.push(BRANCH_MARKER);
            let node_size = Self::estimate_branch_size(&self);
            packed_data.extend_from_slice(&(node_size as u32).to_le_bytes());
            for child in &self.children {
                child.pack_recursive(packed_data);
            }
        }
    }
    pub fn pack_recursive_layer(&self, packed_data: &mut Vec<u8>, layer: u32) {
        if self.is_leaf {
            if layer == 0 && self.octant_index != 9 {
                packed_data.push(LEAF_MARKER + self.octant_index);
                packed_data.extend_from_slice(&self.color);
            }
        } else {
            if layer == 0 {
                let mut nn = 0u8;
                for i in 0..8 {
                    if self.children[i].octant_index != 9 {
                        nn += 1;
                    }
                }
                packed_data.push(BRANCH_MARKER + self.octant_index);
                packed_data.push(nn);
                for i in 0..nn {
                    packed_data.push(0);
                    packed_data.push(0);
                    packed_data.push(0);
                    packed_data.push(0);
                }
                //let node_size = Self::estimate_branch_size(&self);
                //packed_data.extend_from_slice(&(node_size as u32).to_le_bytes());
            } else {
                for child in &self.children {
                    child.pack_recursive_layer(packed_data, layer - 1);
                }
            }
        }
    }
}

pub fn compute_raw_data_indices(packed_data: &mut Vec<u8>) -> Vec<u32> {
    let mut indices = vec![];
    let mut i = 0;
    while i < packed_data.len() {
        match packed_data[i] < BRANCH_MARKER {
            true => {
                indices.push(i as u32);
                i += 4;
            }
            false => {
                indices.push(i as u32);
                let chn = packed_data[i + 1];
                i += (chn as usize) * 4 + 1;
            }
        }
        i += 1;
    }
    indices
}

pub fn fill_indices(packed_data: &mut Vec<u8>, indices: Vec<u32>) {
    let mut ii = 0;
    let mut i = 0;
    while i < packed_data.len() {
        match packed_data[i] < BRANCH_MARKER {
            true => {
                i += 4;
            }
            false => {
                let chn = packed_data[i + 1];
                for j in (0..chn * 4).step_by(4) {
                    let nextind = indices[ii].to_le_bytes();
                    ii += 1;
                    packed_data[i + 2 + j as usize] = nextind[0];
                    packed_data[i + 3 + j as usize] = nextind[1];
                    packed_data[i + 4 + j as usize] = nextind[2];
                    packed_data[i + 5 + j as usize] = nextind[3];
                }
                i += (chn as usize) * 4 + 1;
            }
        }
        i += 1;
    }
}

pub struct VoxelScene {
    pub data: Vec<u8>,
    pub voxels: Vec<Voxel>,
    pub levels: u32,
    pub size: [u32; 3],
    pub origin: Vec3,
    pub voxel_size: f32,
}

pub fn voxel_overlaps_triangle(
    voxel_center: Vec3,
    voxel_size: f32,
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
    dominant_axis: usize,
) -> bool {
    let half = voxel_size * 0.5;

    let (c, t0, t1, t2) = match dominant_axis {
        0 => (
            [voxel_center.y, voxel_center.z],
            [v0.y, v0.z],
            [v1.y, v1.z],
            [v2.y, v2.z],
        ),
        1 => (
            [voxel_center.x, voxel_center.z],
            [v0.x, v0.z],
            [v1.x, v1.z],
            [v2.x, v2.z],
        ),
        _ => (
            [voxel_center.x, voxel_center.y],
            [v0.x, v0.y],
            [v1.x, v1.y],
            [v2.x, v2.y],
        ),
    };

    // 2D separating axis test between the square voxel and projected triangle
    // Tests triangle edges as potential separating axes
    let edges = [
        [t1[0] - t0[0], t1[1] - t0[1]],
        [t2[0] - t1[0], t2[1] - t1[1]],
        [t0[0] - t2[0], t0[1] - t2[1]],
    ];

    for edge in &edges {
        // Normal to this edge (perpendicular in 2D)
        let axis = [-edge[1], edge[0]];

        // Project triangle vertices onto axis
        let p0 = axis[0] * t0[0] + axis[1] * t0[1];
        let p1 = axis[0] * t1[0] + axis[1] * t1[1];
        let p2 = axis[0] * t2[0] + axis[1] * t2[1];

        let tri_min = p0.min(p1).min(p2);
        let tri_max = p0.max(p1).max(p2);

        // Project voxel center onto axis, radius is half-diagonal on this axis
        let vox_proj = axis[0] * c[0] + axis[1] * c[1];
        let vox_radius = half * (axis[0].abs() + axis[1].abs());

        // Separating axis found — no overlap
        if tri_max < vox_proj - vox_radius || tri_min > vox_proj + vox_radius {
            return false;
        }
    }

    // Also test the two AABB axes (X and Y of projection plane)
    for axis in [[1.0f32, 0.0f32], [0.0, 1.0]] {
        let p0 = axis[0] * t0[0] + axis[1] * t0[1];
        let p1 = axis[0] * t1[0] + axis[1] * t1[1];
        let p2 = axis[0] * t2[0] + axis[1] * t2[1];

        let tri_min = p0.min(p1).min(p2);
        let tri_max = p0.max(p1).max(p2);
        let vox_proj = axis[0] * c[0] + axis[1] * c[1];

        if tri_max < vox_proj - half || tri_min > vox_proj + half {
            return false;
        }
    }

    true
}

impl VoxelScene {
    fn compress_data_impl(data: &[u8]) -> (Vec<Rledata>, Vec<u8>) {
        let mut entries = Vec::new();
        let mut tail = Vec::new();
        let mut i = 0usize;

        while i < data.len() {
            let current = data[i];
            let mut run_len = 1usize;
            while i + run_len < data.len() && data[i + run_len] == current {
                run_len += 1;
            }

            if run_len > 17 {
                entries.push(Rledata {
                    data: current,
                    location: i as u64,
                    length: run_len as u64,
                });
                i += run_len;
            } else {
                tail.extend_from_slice(&data[i..(i + run_len)]);
                i += run_len;
            }
        }

        (entries, tail)
    }
    fn decompress_data_impl(entries: &[Rledata], tail: &[u8], total_len: usize) -> Vec<u8> {
        let mut result = vec![0u8; total_len];
        let mut sorted_entries = entries.to_vec();
        sorted_entries.sort_by_key(|entry| entry.location);
        let mut raw_index = 0usize;
        let mut cursor = 0usize;
        for e in &sorted_entries {
            let start = e.location as usize;
            if start > cursor {
                let n = start - cursor;
                result[cursor..start].copy_from_slice(&tail[raw_index..raw_index + n]);
                raw_index += n;
            }
            let end = start + e.length as usize;
            result[start..end].fill(e.data);
            cursor = end;
        }
        if cursor < total_len {
            let n = total_len - cursor;
            result[cursor..total_len].copy_from_slice(&tail[raw_index..raw_index + n]);
        }

        result
    }
    pub fn new_blank() -> VoxelScene {
        VoxelScene {
            data: vec![],
            voxels: Vec::new(),
            levels: 0,
            voxel_size: 1.0,
            origin: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            size: [0, 0, 0],
        }
    }
    pub fn get_boundaries(vertices: Vec<f32>) -> ([i32; 3], [i32; 3]) {
        let mut upper_boundaries = [
            vertices[0].round() as i32,
            vertices[1].round() as i32,
            vertices[2].round() as i32,
        ];
        let mut lower_boundaries = [
            vertices[0].round() as i32,
            vertices[1].round() as i32,
            vertices[2].round() as i32,
        ];
        let vl = vertices.len() / 3;
        for i in 0..vl {
            let x = vertices[i * 3 + 0];
            let y = vertices[i * 3 + 1];
            let z = vertices[i * 3 + 2];
            if x > upper_boundaries[0] as f32 {
                upper_boundaries[0] = x.round() as i32 + 1;
            }
            if x < lower_boundaries[0] as f32 {
                lower_boundaries[0] = x.round() as i32 - 1;
            }
            if y > upper_boundaries[1] as f32 {
                upper_boundaries[1] = y.round() as i32 + 1;
            }
            if y < lower_boundaries[1] as f32 {
                lower_boundaries[1] = y.round() as i32 - 1;
            }
            if z > upper_boundaries[2] as f32 {
                upper_boundaries[2] = z.round() as i32 + 1;
            }
            if z < lower_boundaries[2] as f32 {
                lower_boundaries[2] = z.round() as i32 - 1;
            }
        }
        (lower_boundaries, upper_boundaries)
    }
    pub fn world_to_voxel(&self, pos: Vec3) -> (isize, isize, isize) {
        let mut local = pos - self.origin;
        local.x /= self.voxel_size;
        local.y /= self.voxel_size;
        local.z /= self.voxel_size;
        (local.x as isize, local.y as isize, local.z as isize)
    }
    pub fn set(&mut self, x: u32, y: u32, z: u32, color: [u8; 4], vx: bool) {
        if x < self.size[0] && y < self.size[1] && z < self.size[2] {
            if vx {
                self.voxels.push(Voxel {
                    pos: Ivec3 {
                        x: x as i32,
                        y: y as i32,
                        z: z as i32,
                    },
                    color,
                });
            } else {
                self.data
                    [((x + y * self.size[0] + z * self.size[1] * self.size[0]) as usize) * 4] =
                    color[0];
                self.data
                    [((x + y * self.size[0] + z * self.size[1] * self.size[0]) as usize) * 4 + 1] =
                    color[1];
                self.data
                    [((x + y * self.size[0] + z * self.size[1] * self.size[0]) as usize) * 4 + 2] =
                    color[2];
                self.data
                    [((x + y * self.size[0] + z * self.size[1] * self.size[0]) as usize) * 4 + 3] =
                    color[3];
            }
        }
    }
    pub fn voxelize_triangle(
        &mut self,
        v0: Vec3,
        v1: Vec3,
        v2: Vec3,
        color: [u8; 4],
        voxel_set: bool,
    ) {
        self.voxelize_triangle_in_bounds(v0, v1, v2, color, voxel_set, self.size, self.origin);
    }
    fn voxelize_triangle_in_bounds(
        &mut self,
        v0: Vec3,
        v1: Vec3,
        v2: Vec3,
        color: [u8; 4],
        voxel_set: bool,
        virtual_size: [u32; 3],
        virtual_origin: Vec3,
    ) {
        if virtual_size.iter().any(|dimension| *dimension == 0) {
            return;
        }
        let edge0 = v1 - v0;
        let edge1 = v2 - v0;
        let normal = edge0.cross(edge1);

        let abs_n = normal.abs();
        let dominant_axis = if abs_n.x >= abs_n.y && abs_n.x >= abs_n.z {
            0
        } else if abs_n.y >= abs_n.z {
            1
        } else {
            2
        };
        let tri_min = v0.min(v1).min(v2);
        let tri_max = v0.max(v1).max(v2);

        let min_vx = ((tri_min.x - virtual_origin.x) / self.voxel_size).floor() as isize;
        let min_vy = ((tri_min.y - virtual_origin.y) / self.voxel_size).floor() as isize;
        let min_vz = ((tri_min.z - virtual_origin.z) / self.voxel_size).floor() as isize;
        let max_vx = ((tri_max.x - virtual_origin.x) / self.voxel_size).floor() as isize;
        let max_vy = ((tri_max.y - virtual_origin.y) / self.voxel_size).floor() as isize;
        let max_vz = ((tri_max.z - virtual_origin.z) / self.voxel_size).floor() as isize;
        let x0 = min_vx.max(0);
        let y0 = min_vy.max(0);
        let z0 = min_vz.max(0);
        let x1 = max_vx.min(virtual_size[0] as isize - 1);
        let y1 = max_vy.min(virtual_size[1] as isize - 1);
        let z1 = max_vz.min(virtual_size[2] as isize - 1);
        if x0 > x1 || y0 > y1 || z0 > z1 {
            return;
        }
        for vz in z0..=z1 {
            for vy in y0..=y1 {
                for vx in x0..=x1 {
                    let voxel_center = virtual_origin
                        + Vec3 {
                            x: (vx as f32 + 0.5) * self.voxel_size,
                            y: (vy as f32 + 0.5) * self.voxel_size,
                            z: (vz as f32 + 0.5) * self.voxel_size,
                        };

                    if voxel_overlaps_triangle(
                        voxel_center,
                        self.voxel_size,
                        v0,
                        v1,
                        v2,
                        dominant_axis,
                    ) {
                        let cell_min = virtual_origin
                            + Vec3 {
                                x: vx as f32 * self.voxel_size,
                                y: vy as f32 * self.voxel_size,
                                z: vz as f32 * self.voxel_size,
                            };
                        let x_range = Self::overlapping_voxel_range(
                            cell_min.x,
                            cell_min.x + self.voxel_size,
                            self.origin.x,
                            self.voxel_size,
                            self.size[0],
                        );
                        let y_range = Self::overlapping_voxel_range(
                            cell_min.y,
                            cell_min.y + self.voxel_size,
                            self.origin.y,
                            self.voxel_size,
                            self.size[1],
                        );
                        let z_range = Self::overlapping_voxel_range(
                            cell_min.z,
                            cell_min.z + self.voxel_size,
                            self.origin.z,
                            self.voxel_size,
                            self.size[2],
                        );
                        if let (Some((x0, x1)), Some((y0, y1)), Some((z0, z1))) =
                            (x_range, y_range, z_range)
                        {
                            for out_z in z0..=z1 {
                                for out_y in y0..=y1 {
                                    for out_x in x0..=x1 {
                                        self.set(out_x, out_y, out_z, color, voxel_set);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn overlapping_voxel_range(
        cell_min: f32,
        cell_max: f32,
        output_origin: f32,
        voxel_size: f32,
        output_size: u32,
    ) -> Option<(u32, u32)> {
        if output_size == 0 {
            return None;
        }
        let start = ((cell_min - output_origin) / voxel_size).floor() as isize;
        let end = ((cell_max - output_origin) / voxel_size).ceil() as isize - 1;
        let start = start.max(0);
        let end = end.min(output_size as isize - 1);
        (start <= end).then_some((start as u32, end as u32))
    }
    pub fn voxelize_triangles(&mut self, vertices: Vec<f32>, tricolor: Vec<u8>, voxel_set: bool) {
        self.voxelize_triangles_in_bounds(vertices, tricolor, voxel_set, self.size, self.origin);
    }
    fn voxelize_triangles_in_bounds(
        &mut self,
        vertices: Vec<f32>,
        tricolor: Vec<u8>,
        voxel_set: bool,
        virtual_size: [u32; 3],
        virtual_origin: Vec3,
    ) {
        for i in (0..vertices.len()).step_by(9) {
            self.voxelize_triangle_in_bounds(
                Vec3 {
                    x: vertices[i],
                    y: vertices[i + 1],
                    z: vertices[i + 2],
                },
                Vec3 {
                    x: vertices[i + 3],
                    y: vertices[i + 4],
                    z: vertices[i + 5],
                },
                Vec3 {
                    x: vertices[i + 6],
                    y: vertices[i + 7],
                    z: vertices[i + 8],
                },
                [
                    tricolor[(i / 9) * 4],
                    tricolor[(i / 9) * 4 + 1],
                    tricolor[(i / 9) * 4 + 2],
                    tricolor[(i / 9) * 4 + 3],
                ],
                voxel_set,
                virtual_size,
                virtual_origin,
            );
        }
    }
    pub fn new(
        voxel_representation: Vec<u8>,
        voxel_size: f32,
        size: [u32; 3],
        origin: Vec3,
        levels: u32,
    ) -> VoxelScene {
        VoxelScene {
            data: voxel_representation,
            voxels: Vec::new(),
            levels: levels,
            voxel_size: voxel_size,
            origin: origin,
            size,
        }
    }
    pub fn from_vertices(vertices: Vec<f32>, voxel_size: f32, tricolor: Vec<u8>) -> VoxelScene {
        Self::from_vertices_with_settings(vertices, VoxelizerSettings::new(voxel_size), tricolor)
    }
    pub fn from_vertices_with_settings(
        vertices: Vec<f32>,
        settings: VoxelizerSettings,
        tricolor: Vec<u8>,
    ) -> VoxelScene {
        let voxel_size = settings.voxel_size;
        if voxel_size <= 0.0 || !voxel_size.is_finite() || vertices.len() < 9 {
            return Self::new_blank();
        }
        let mut scene_min = Vec3 {
            x: f32::INFINITY,
            y: f32::INFINITY,
            z: f32::INFINITY,
        };
        let mut scene_max = Vec3 {
            x: f32::NEG_INFINITY,
            y: f32::NEG_INFINITY,
            z: f32::NEG_INFINITY,
        };
        for vertex in vertices.chunks_exact(3) {
            let point = Vec3 {
                x: vertex[0],
                y: vertex[1],
                z: vertex[2],
            };
            scene_min = scene_min.min(point);
            scene_max = scene_max.max(point);
        }
        let virtual_origin = Vec3 {
            x: (scene_min.x / voxel_size).floor() * voxel_size,
            y: (scene_min.y / voxel_size).floor() * voxel_size,
            z: (scene_min.z / voxel_size).floor() * voxel_size,
        };
        let virtual_size = [
            ((scene_max.x - virtual_origin.x) / voxel_size).floor() as u32 + 1,
            ((scene_max.y - virtual_origin.y) / voxel_size).floor() as u32 + 1,
            ((scene_max.z - virtual_origin.z) / voxel_size).floor() as u32 + 1,
        ];
        let origin = settings.origin.unwrap_or(virtual_origin);
        let size = settings.size.map_or(virtual_size, |size| {
            [
                size.x.max(0) as u32,
                size.y.max(0) as u32,
                size.z.max(0) as u32,
            ]
        });
        let mut scn = Self::new(vec![], voxel_size, size, origin, 0);
        scn.data.resize(
            size[0] as usize * size[1] as usize * size[2] as usize * 4,
            0,
        );
        scn.voxelize_triangles_in_bounds(vertices, tricolor, false, virtual_size, virtual_origin);
        scn
    }
    pub fn from_vertices_svo(vertices: Vec<f32>, tricolor: Vec<u8>, levels: u32) -> VoxelScene {
        let bond = Self::get_boundaries(vertices.clone());
        let dsize = [
            (bond.1[0] - bond.0[0]).abs() as f32 as u32,
            (bond.1[1] - bond.0[1]).abs() as f32 as u32,
            (bond.1[2] - bond.0[2]).abs() as f32 as u32,
        ];
        let size = [2u32.pow(levels), 2u32.pow(levels), 2u32.pow(levels)];
        let voxel_size = *dsize.iter().max().unwrap_or(&1) as f32 / size[0] as f32;
        let mut scn = Self::new(
            vec![],
            voxel_size,
            size,
            Vec3 {
                x: bond.0[0] as f32,
                y: bond.0[1] as f32,
                z: bond.0[2] as f32,
            },
            levels,
        );
        //scn.data.resize(((size[0]*size[1]*size[2]*4) as f32) as usize, 0);
        scn.voxelize_triangles(vertices, tricolor, true);
        let svo = SVO::build_tree_recursive(
            scn.voxels.clone(),
            Ivec3 { x: 0, y: 0, z: 0 },
            Ivec3 {
                x: size[0] as i32,
                y: size[1] as i32,
                z: size[2] as i32,
            },
            levels,
            10,
        );

        scn.data = vec![];
        let mut datavec = vec![];
        let mut indvec = vec![];
        for i in (0..(levels + 1)).rev() {
            let mut dvec = vec![];
            svo.pack_recursive_layer(&mut dvec, i);
            fill_indices(&mut dvec, indvec);
            indvec = compute_raw_data_indices(&mut dvec);
            datavec.push(dvec);
        }
        datavec.reverse();
        let header_size = (levels * 4) as u32;
        let mut offset = header_size + datavec[0].len() as u32;
        for depth in 1..=levels {
            scn.data.extend_from_slice(&offset.to_le_bytes());
            offset += datavec[depth as usize].len() as u32;
        }

        for depth in 0..=levels {
            scn.data.extend_from_slice(&datavec[depth as usize]);
        }
        scn
    }
    pub fn from_file(path: &str) -> VoxelScene {
        let cont = readfs(path);
        let jsonend = u32::from_ne_bytes([cont[0], cont[1], cont[2], cont[3]]);
        let rjson = cont[4..jsonend as usize].to_vec();
        let mut size = [0u32, 0u32, 0u32];
        let mut origin = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut voxelsize = 1.0f32;
        let mut compression_active = false;
        let json = JsonF::from_text(&String::from_utf8(rjson).unwrap());
        let mut lvls = 0u32;
        let mut ln = 0u32;
        for i in 0..json.other_nodes.len() {
            match json.other_nodes[i].name.as_str() {
                "voxel_size" => voxelsize = json.other_nodes[i].numeral_val as f32,
                "size_x" => size[0] = json.other_nodes[i].numeral_val as u32,
                "size_y" => size[1] = json.other_nodes[i].numeral_val as u32,
                "size_z" => size[2] = json.other_nodes[i].numeral_val as u32,
                "origin_x" => origin.x = json.other_nodes[i].numeral_val as f32,
                "origin_y" => origin.y = json.other_nodes[i].numeral_val as f32,
                "origin_z" => origin.z = json.other_nodes[i].numeral_val as f32,
                "compression_active" => compression_active = json.other_nodes[i].bolean,
                "levels" => lvls = json.other_nodes[i].numeral_val as u32,
                "data_size" => ln = json.other_nodes[i].numeral_val as u32,
                _ => {}
            }
        }
        let payload = cont[(jsonend) as usize..cont.len()].to_vec();
        let data = if compression_active {
            if payload.len() < 8 {
                vec![]
            } else {
                let count = u64::from_ne_bytes(payload[0..8].try_into().unwrap()) as usize;
                let mut cursor = 8usize;
                let mut entries = Vec::new();
                for _ in 0..count {
                    if cursor + 17 > payload.len() {
                        break;
                    }
                    let mut data_byte = [0u8; 1];
                    data_byte.copy_from_slice(&payload[cursor..(cursor + 1)]);
                    let mut location_bytes = [0u8; 8];
                    location_bytes.copy_from_slice(&payload[(cursor + 1)..(cursor + 9)]);
                    let mut length_bytes = [0u8; 8];
                    length_bytes.copy_from_slice(&payload[(cursor + 9)..(cursor + 17)]);
                    entries.push(Rledata {
                        data: data_byte[0],
                        location: u64::from_ne_bytes(location_bytes),
                        length: u64::from_ne_bytes(length_bytes),
                    });
                    cursor += 17;
                }
                let tail = payload[cursor..payload.len()].to_vec();
                let expected_len = ((size[0] as f32) * (size[1] as f32) * (size[2] as f32)
                    / voxelsize.powi(3)) as usize
                    * 4;
                if ln == 0 {
                    ln = expected_len as u32;
                }
                Self::decompress_data_impl(&entries, &tail, ln as usize)
            }
        } else {
            payload
        };
        VoxelScene::new(data, voxelsize, size, origin, lvls)
    }
    pub fn save_file(&mut self, path: &str) {
        let mut s = String::new();
        s.push_str("{\n");
        s.push_str(&format!("  \"voxel_size\": {},\n", self.voxel_size));
        s.push_str(&format!("  \"size_x\": {},\n", self.size[0]));
        s.push_str(&format!("  \"size_y\": {},\n", self.size[1]));
        s.push_str(&format!("  \"size_z\": {},\n", self.size[2]));
        s.push_str(&format!("  \"origin_x\": {},\n", self.origin.x));
        s.push_str(&format!("  \"origin_y\": {},\n", self.origin.y));
        s.push_str(&format!("  \"origin_z\": {},\n", self.origin.z));
        s.push_str(&format!("  \"levels\": {},\n", self.levels));
        s.push_str(&format!("  \"data_size\": {},\n", self.data.len()));
        let (entries, tail) = Self::compress_data_impl(&self.data);
        let compression_active = !entries.is_empty();
        s.push_str(&format!(
            "  \"compression_active\": {}\n",
            if compression_active { "true" } else { "false" }
        ));
        s.push_str("}");
        let beggoff = ((s.len() + 4) as u32).to_ne_bytes();
        let mut rwd = beggoff.clone().to_vec();
        rwd.append(&mut s.into_bytes());

        let (entries, tail) = Self::compress_data_impl(&self.data);
        let compression_active = !entries.is_empty();
        if compression_active {
            let mut payload = Vec::new();
            payload.extend_from_slice(&(entries.len() as u64).to_ne_bytes());
            for entry in &entries {
                let mut entry_bytes = [0u8; 17];
                entry_bytes[0] = entry.data;
                entry_bytes[1..9].copy_from_slice(&entry.location.to_ne_bytes());
                entry_bytes[9..17].copy_from_slice(&entry.length.to_ne_bytes());
                payload.extend_from_slice(&entry_bytes);
            }
            payload.extend_from_slice(&tail);
            rwd.append(&mut payload);
        } else {
            rwd.append(&mut self.data.clone());
        }

        writefs(path, rwd);
    }
}

#[cfg(test)]
mod tests {
    use super::{VoxelScene, VoxelizerSettings};
    use crate::engine::math::{ivec3::Ivec3, vec3::Vec3};

    fn triangle() -> Vec<f32> {
        vec![0.1, 0.1, 0.2, 0.9, 0.1, 0.2, 0.1, 0.9, 0.2]
    }

    #[test]
    fn computes_output_bounds_when_settings_are_unset() {
        let color = [10, 20, 30, 255];
        let scene = VoxelScene::from_vertices_with_settings(
            triangle(),
            VoxelizerSettings::new(1.0),
            color.to_vec(),
        );

        assert_eq!(scene.size, [1, 1, 1]);
        assert_eq!(scene.origin.x, 0.0);
        assert_eq!(scene.data, color);
    }

    #[test]
    fn clips_virtual_voxels_to_explicit_output_bounds() {
        let color = [10, 20, 30, 255];
        let mut settings = VoxelizerSettings::new_with_size(1.0, Ivec3 { x: 1, y: 1, z: 1 });
        settings.origin = Some(Vec3 {
            x: 0.5,
            y: 0.0,
            z: 0.0,
        });
        let scene = VoxelScene::from_vertices_with_settings(triangle(), settings, color.to_vec());

        assert_eq!(scene.size, [1, 1, 1]);
        assert_eq!(scene.data, color);

        settings.origin = Some(Vec3 {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        });
        let clipped = VoxelScene::from_vertices_with_settings(triangle(), settings, color.to_vec());
        assert_eq!(clipped.data, vec![0; 4]);
    }
}
