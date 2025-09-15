use crate::prelude::*;
use crate::scripting::script::Script;
use crate::world::Plane;

pub struct Entity {
    pos: Vector,
    dir: Vector,

    // One entity owns its planes and children and is responsible for dropping them when it goes out of scope.
    // The Scene will hold references to the planes for rendering and collision detection.
    planes: Vec<Plane>,
    // pub(crate) _children: Vec<*mut Entity<'a>>,
    scripts: Vec<Box<dyn Script>>,
}

impl Entity {
    pub fn new(position: Vector) -> Self {
        Self {
            pos: position,
            dir: Vector::forward(),
            planes: Vec::new(),
            // _children: Vec::new(),
            scripts: Vec::new(),
        }
    }

    /// Creates a new EntityNode from a Plane.
    /// The position of the EntityNode is set to the center of the Plane.
    pub fn from_plane(plane: Plane) -> Self {
        let pos = plane.segment.start + plane.segment.dir * 0.5;
        let dir = plane.dir();

        Self {
            pos,
            dir,
            planes: vec![plane],
            // _children: Vec::new(),
            scripts: Vec::new(),
        }
    }

    pub fn add_script(&mut self, script: Box<dyn Script>) {
        self.scripts.push(script);
    }

    pub fn add_plane(&mut self, plane: Plane) {
        self.planes.push(plane);
    }

    pub fn add_child(&mut self, _child: Entity) {
        todo!()
    }

    pub(crate) fn planes(&self) -> impl Iterator<Item = &Plane> {
        self.planes.iter()
    }

    pub(crate) fn scripts_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn Script>> + use<'_> {
        self.scripts.iter_mut()
    }
}

impl Spatial for Entity {
    #[inline]
    fn pos(&self) -> Vector {
        self.pos
    }

    fn set_pos(&mut self, pos: Vector) {
        let delta = pos - self.pos;
        for plane in &mut self.planes {
            plane.segment.start = plane.segment.start + delta;
        }
        self.pos = pos
    }

    fn dir(&self) -> Vector {
        self.dir
    }

    fn set_dir(&mut self, value: Vector) {
        let factor = value.cdiv(self.dir);
        for plane in &mut self.planes {
            plane.segment.dir = plane.segment.dir.cmul(factor);
        }
        self.dir = value;
    }

    fn angle(&self) -> f32 {
        self.dir.angle()
    }

    fn translate(&mut self, delta: Vector) {
        self.pos = self.pos + delta;
        for plane in &mut self.planes {
            plane.segment.start = plane.segment.start + delta;
        }
    }
}
