use std::ptr::NonNull;

use crate::engine::Input;
use crate::scripting::script::Script;
use crate::world::Plane;
use crate::world::empty::Empty;
use crate::{StartContext, SystemContext, UpdateContext, prelude::*};

pub(crate) enum EntityInner {
    Empty(Empty),
    Plane(Plane),
}

pub struct Entity {
    pub(crate) inner: EntityInner,

    relative_pos: Vector,
    relative_dir: Vector,
    parent: Option<NonNull<Entity>>,
    scripts: Vec<Box<dyn Script>>,
}

impl Entity {
    pub fn add_script(&mut self, script: Box<dyn Script>) {
        self.scripts.push(script);
    }

    fn inner_pos(&self) -> Vector {
        match self.inner {
            EntityInner::Empty(ref empty) => empty.pos,
            EntityInner::Plane(ref plane) => plane.pos(),
        }
    }

    fn inner_dir(&self) -> Vector {
        match self.inner {
            EntityInner::Empty(ref empty) => empty.dir,
            EntityInner::Plane(ref plane) => plane.dir(),
        }
    }

    fn set_inner_pos(&mut self, pos: impl Into<Vector>) {
        match self.inner {
            EntityInner::Empty(ref mut empty) => {
                empty.pos = pos.into();
            }
            EntityInner::Plane(ref mut plane) => {
                plane.segment.start = pos.into();
            }
        }
    }

    fn set_inner_dir(&mut self, dir: impl Into<Vector>) {
        match self.inner {
            EntityInner::Empty(ref mut empty) => {
                empty.dir = dir.into();
            }
            EntityInner::Plane(ref mut plane) => {
                plane.segment.dir = dir.into();
            }
        }
    }

    fn follow_parent(&mut self) {
        match self.parent {
            Some(_) => {
                todo!()
            }
            None => {}
        }
    }

    pub fn pos(&self) -> Vector {
        if self.parent.is_none() {
            self.inner_pos()
        } else {
            self.relative_pos
        }
    }

    pub fn dir(&self) -> Vector {
        if self.parent.is_none() {
            self.inner_dir()
        } else {
            self.relative_dir
        }
    }

    pub fn angle(&self) -> f32 {
        self.dir().angle()
    }

    pub fn set_pos(&mut self, pos: impl Into<Vector>) {
        match self.parent {
            Some(_) => {
                self.relative_pos = pos.into();
                self.follow_parent();
            }
            None => {
                self.set_inner_pos(pos.into());
            }
        }
    }

    pub fn set_dir(&mut self, dir: impl Into<Vector>) {
        match self.parent {
            Some(_) => {
                self.relative_dir = dir.into();
                self.follow_parent();
            }
            None => {
                self.set_inner_dir(dir.into());
            }
        }
    }

    pub fn set_angle(&mut self, angle: f32) {
        let new_dir = Vector::from_deg(angle);
        self.set_dir(new_dir);
    }

    pub fn translate(&mut self, delta: impl Into<Vector>) {
        let new_pos = self.pos() + delta.into();
        self.set_pos(new_pos);
    }

    pub fn transform(&mut self, transformation: impl Into<Vector>) {
        let dir = self.dir();
        let new_dir = dir.cmul(transformation.into());
        self.set_dir(new_dir);
    }

    pub fn rotate(&mut self, angle: f32) {
        let dir = self.dir();
        let new_dir = dir.cmul(Vector::from_deg(angle));
        self.set_dir(new_dir);
    }

    pub(crate) fn start(&mut self, scene: &mut Scene, system: &mut SystemContext) {
        let self_ptr = self as *mut Entity;
        let scripts = self.scripts.iter_mut().collect::<Vec<_>>();
        for script in scripts {
            let ctx = StartContext {
                entity: unsafe { &mut *self_ptr },
                system,
                scene,
            };

            script.start(ctx);
        }
    }

    pub(crate) fn update(&mut self, scene: &mut Scene, input: Input, system: &mut SystemContext) {
        let self_ptr = self as *mut Entity;
        let scripts = self.scripts.iter_mut().collect::<Vec<_>>();
        for script in scripts {
            let ctx = UpdateContext {
                entity: unsafe { &mut *self_ptr },
                input: input.clone(),
                system,
                scene,
            };

            script.tick(ctx);
        }
    }
}

impl From<Plane> for Entity {
    fn from(plane: Plane) -> Self {
        let pos = plane.segment.start + plane.segment.dir * 0.5;
        let dir = plane.dir();

        Self {
            relative_pos: pos,
            relative_dir: dir,
            parent: None,
            inner: EntityInner::Plane(plane),
            scripts: Vec::new(),
        }
    }
}

impl From<Empty> for Entity {
    fn from(empty: Empty) -> Self {
        Self {
            relative_pos: empty.pos,
            relative_dir: empty.dir,
            parent: None,
            inner: EntityInner::Empty(empty),
            scripts: Vec::new(),
        }
    }
}
