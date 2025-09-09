use crate::{vector::Vector, world::Plane};

pub struct EntityNode {
    position: Vector,
    _transform: Vector,

    // One entity owns its planes and children and is responsible for dropping them when it goes out of scope.
    // The Scene will hold references to the planes for rendering and collision detection.
    pub(crate) planes: Vec<*mut Plane>,
    pub(crate) _children: Vec<*mut EntityNode>,
}

pub trait Entity {
    fn pos(&self) -> Vector;
    fn transform(&self) -> Vector;
}

impl EntityNode {
    pub fn new(position: Vector) -> Self {
        Self {
            position,
            _transform: Vector::forward(),
            planes: Vec::new(),
            _children: Vec::new(),
        }
    }

    /// Creates a new EntityNode from a Plane.
    /// The position of the EntityNode is set to the center of the Plane.
    pub fn from_plane(plane: Plane) -> Self {
        let position = plane.start + plane.direction * 0.5;
        let transform = plane.transform();
        let ptr = Box::into_raw(Box::new(plane));

        Self {
            position,
            _transform: transform,
            planes: vec![ptr],
            _children: Vec::new(),
        }
    }

    pub fn add_plane(&mut self, plane: Plane) {
        let ptr = Box::into_raw(Box::new(plane));
        self.planes.push(ptr);
    }

    pub fn add_child(&mut self, _child: EntityNode) {
        todo!()
    }
}

impl Drop for EntityNode {
    fn drop(&mut self) {
        for &plane_ptr in &self.planes {
            unsafe {
                let _ = Box::from_raw(plane_ptr);
            }
        }

        for &child_ptr in &self._children {
            unsafe {
                let _ = Box::from_raw(child_ptr);
            }
        }
    }
}

impl Entity for EntityNode {
    #[inline]
    fn pos(&self) -> Vector {
        self.position
    }

    #[inline]
    fn transform(&self) -> Vector {
        self._transform
    }
}
