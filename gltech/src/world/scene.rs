use std::time::Duration;

use crate::{Ray, world::*};

// The Scene owns its entities and is responsible for dropping them when it goes out of scope. However, auxiliar structs
// like planes are owned by entities and the Scene only holds references to them for rendering and collision detection.
pub struct Scene {
    pub camera: Entity,
    children: Vec<Entity>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            camera: Camera::default().into(),
            children: Vec::new(),
        }
    }

    pub fn add(&mut self, node: impl Into<Entity>) {
        self.children.push(node.into());
    }

    // FIXME: Iterate the whole tree
    pub fn entities_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.children.iter_mut()
    }

    pub fn planes(&self) -> impl Iterator<Item = &Plane> {
        self.children
            .iter()
            .filter(|e| matches!(e.inner, EntityInner::Plane(_)))
            .map(|e| {
                if let EntityInner::Plane(ref plane) = e.inner {
                    plane
                } else {
                    unreachable!()
                }
            })
    }

    pub(crate) fn update(&mut self, delta_time: Duration, time: Duration) {
        let ptr = self as *mut Scene;
        let current_entities = self.entities_mut().collect::<Vec<_>>();
        for entity in current_entities {
            let second_ref = unsafe { &mut *ptr };
            entity.update(second_ref, delta_time, time);
        }
    }

    pub fn raycast(&self, ray: Ray) -> Option<(&Plane, (f32, f32))> {
        let mut rs = (f32::INFINITY, f32::INFINITY);
        let mut nearest_plane = None;

        for plane in self.planes() {
            let (distance, split) = ray.get_rs(plane.segment);

            if distance < 0.0 || split < 0.0 || split >= 1.0 {
                continue;
            };

            if distance < rs.0 {
                rs = (distance, split);
                nearest_plane = Some(plane);
            }
        }

        Some((nearest_plane?, rs))
    }
}
