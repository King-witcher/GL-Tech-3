use crate::{Ray, Vector, world::*};

// The Scene owns its entities and is responsible for dropping them when it goes out of scope. However, auxiliar structs
// like planes are owned by entities and the Scene only holds references to them for rendering and collision detection.
pub struct Scene {
    pub camera: Entity,
    pub(crate) planes: Vec<*mut Plane>,
    pub(crate) entities: Vec<Entity>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            camera: Entity::new(Vector(0.0, 0.0)),
            planes: Vec::new(),
            entities: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Entity) {
        let planes = node.planes.clone();
        self.planes.extend(planes);
        self.entities.push(node);
    }

    pub(crate) fn raycast(&self, ray: Ray) -> Option<(&Plane, (f32, f32))> {
        let mut rs = (f32::INFINITY, f32::INFINITY);
        let mut nearest_plane = None;

        for plane in self.planes.iter().map(|&p| unsafe { &*p }) {
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
