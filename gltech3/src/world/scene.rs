use crate::{Segment, Vector, world::*};

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

    pub(crate) fn nearest_plane(&self, origin: Vector, direction: Vector) -> Option<(&Plane, f32)> {
        let mut nearest_r = f32::INFINITY;
        let mut nearest_plane = None;
        let segment = Segment::new(origin, direction);

        for plane in self.planes.iter().map(|&p| unsafe { &*p }) {
            let plane_segment = Segment::new(plane.start, plane.direction);
            let (r, s) = segment.get_rs(plane_segment);
            if r < 0.0 || s < 0.0 || s >= 1.0 {
                continue;
            };

            if r < nearest_r {
                nearest_r = r;
                nearest_plane = Some(plane);
            }
        }

        Some((nearest_plane?, nearest_r))
    }
}
