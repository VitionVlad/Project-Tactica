use crate::engine::{math::vec3::Vec3, physics::physics::PhysicsObject};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum ForceKind {
    Global { direction: Vec3 },
    Directional { origin: Vec3, direction: Vec3, radius: f32, max_distance: f32, falloff_distance: f32 },
    Point { position: Vec3, radius: f32, falloff_radius: f32 },
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Force {
    pub magnitude: f32,
    pub force_kind: ForceKind,
}

impl Force {
    #[allow(dead_code)]
    pub fn new(magnitude: f32, force_kind: ForceKind) -> Self {
        Self { magnitude, force_kind }
    }
    #[allow(dead_code)]
    pub fn compute_force(&mut self, object: &mut PhysicsObject){
        if object.compute_force{
            match self.force_kind {
                ForceKind::Global { direction } => {
                    self.force_kind = ForceKind::Global { direction: direction.normalize() };
                    let new_acceleration = (direction * self.magnitude)/object.mass;
                    object.acceleration += new_acceleration;
                },
                ForceKind::Directional { origin, direction, radius, max_distance, falloff_distance } => {
                    let direction = direction.normalize();
                    if direction.length_squared() == 0.0 || radius < 0.0 || max_distance <= 0.0 {
                        return;
                    }

                    let radius_squared = radius * radius;
                    let mut distance_along_axis = None;

                    for corner in object.collider.corners.iter() {
                        let offset = *corner - origin;
                        let distance = offset.dot(direction);
                        if distance < 0.0 || distance > max_distance {
                            continue;
                        }

                        let radial = offset - direction * distance;
                        if radial.length_squared() <= radius_squared {
                            distance_along_axis = Some(distance_along_axis
                                .map_or(distance, |current: f32| current.min(distance)));
                        }
                    }

                    let Some(distance) = distance_along_axis else {
                        return;
                    };

                    let coefficient = if distance <= falloff_distance {
                        1.0
                    } else if max_distance > falloff_distance {
                        (max_distance - distance) / (max_distance - falloff_distance)
                    } else {
                        0.0
                    };

                    object.acceleration += direction * (self.magnitude * coefficient / object.mass);
                },
                ForceKind::Point { position, radius, falloff_radius } => {
                    let closest_point = Self::closest_point(position, object);
                    let offset = closest_point - position;
                    let distance = offset.length_squared().sqrt();
                    let direction = offset.normalize();
                    let coefficient = if distance <= falloff_radius {
                        1.0
                    } else if radius > falloff_radius && distance < radius {
                        (radius - distance) / (radius - falloff_radius)
                    } else {
                        0.0
                    };

                    object.acceleration += direction * (self.magnitude * coefficient / object.mass);
                },
            }
        }
    }

    fn closest_point(position: Vec3, object: &PhysicsObject) -> Vec3 {
        let mut min = object.collider.corners[0];
        let mut max = object.collider.corners[0];

        for corner in object.collider.corners.iter().skip(1) {
            min = min.min(*corner);
            max = max.max(*corner);
        }

        Vec3 {
            x: position.x.clamp(min.x, max.x),
            y: position.y.clamp(min.y, max.y),
            z: position.z.clamp(min.z, max.z),
        }
    }
}