use crate::prelude::*;
use crate::scripting::script::Script;
use crate::world::Plane;
use crate::world::Spatial;

pub struct Entity {
    pos: Vector,
    dir: Vector,

    // One entity owns its planes and children and is responsible for dropping them when it goes out of scope.
    // The Scene will hold references to the planes for rendering and collision detection.
    pub(crate) planes: Vec<*mut Plane>,
    pub(crate) _children: Vec<*mut Entity>,
    pub(crate) scripts: Vec<Box<dyn Script>>,
}

impl Entity {
    pub fn new(position: Vector) -> Self {
        Self {
            pos: position,
            dir: Vector::forward(),
            planes: Vec::new(),
            _children: Vec::new(),
            scripts: Vec::new(),
        }
    }

    /// Creates a new EntityNode from a Plane.
    /// The position of the EntityNode is set to the center of the Plane.
    pub fn from_plane(plane: Plane) -> Self {
        let pos = plane.start + plane.direction * 0.5;
        let dir = plane.dir();
        let ptr = Box::into_raw(Box::new(plane));

        Self {
            pos,
            dir,
            planes: vec![ptr],
            _children: Vec::new(),
            scripts: Vec::new(),
        }
    }

    pub fn add_script(&mut self, script: Box<dyn Script>) {
        self.scripts.push(script);
    }

    pub fn add_plane(&mut self, plane: Plane) {
        let ptr = Box::into_raw(Box::new(plane));
        self.planes.push(ptr);
    }

    pub fn add_child(&mut self, _child: Entity) {
        todo!()
    }
}

impl Drop for Entity {
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

impl Spatial for Entity {
    #[inline]
    fn pos(&self) -> Vector {
        self.pos
    }

    fn set_pos(&mut self, pos: Vector) {
        let delta = pos - self.pos;
        for &plane_ptr in &self.planes {
            unsafe {
                let plane = plane_ptr.as_mut().unwrap();
                plane.start = plane.start + delta;
            }
        }
        self.pos = pos
    }

    fn dir(&self) -> Vector {
        self.dir
    }

    fn set_dir(&mut self, value: Vector) {
        let factor = value.cdiv(self.dir);
        println!("Factor: {:?}", factor);
        for &plane_ptr in &self.planes {
            unsafe {
                let plane = plane_ptr.as_mut().unwrap();
                plane.direction = plane.direction.cmul(factor);
            }
        }
        self.dir = value;
    }

    fn angle(&self) -> f32 {
        self.dir.angle()
    }

    fn translate(&mut self, delta: Vector) {
        self.pos = self.pos + delta;
        for &plane_ptr in &self.planes {
            unsafe {
                let plane = plane_ptr.as_mut().unwrap();
                plane.start = plane.start + delta;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
