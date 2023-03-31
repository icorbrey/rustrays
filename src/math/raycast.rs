use crate::physical::{object::Object, scene::Scene};

use super::vector3::Vector3;

#[derive(Copy, Clone)]
pub struct Raycast {
    pub direction: Vector3,
    pub normal: Vector3,
    pub origin: Vector3,
    pub object: Object,
    pub point: Vector3,
    pub view: Vector3,
}

impl Raycast {
    pub fn compute(
        scene: &Scene,
        origin: Vector3,
        direction: Vector3,
        range: (f64, f64),
    ) -> Option<Self> {
        let mut closest_object: Option<Object> = None;
        let mut closest_t: Option<f64> = None;

        let (t_min, t_max) = range;

        for object in &scene.objects {
            if let Some((t1, t2)) = object.compute_intersections(origin, direction) {
                let current_closest_t = closest_t.unwrap_or(f64::INFINITY);

                if t_min < t1 && t1 < t_max && t1 < current_closest_t {
                    closest_object = Some(*object);
                    closest_t = Some(t1);
                }

                if t_min < t2 && t2 < t_max && t2 < current_closest_t {
                    closest_object = Some(*object);
                    closest_t = Some(t2);
                }
            }
        }

        if closest_object.is_none() {
            return None;
        }

        let point = origin + direction * closest_t.unwrap();
        let object = closest_object.unwrap();

        Some(Raycast {
            normal: object.compute_normal(point),
            view: -direction,
            direction,
            object,
            origin,
            point,
        })
    }
}
