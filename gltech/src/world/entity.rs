use std::ptr::NonNull;
use std::time::Duration;

use crate::scripting::script::Script;
use crate::world::Plane;
use crate::world::empty::Empty;
use crate::{Camera, UpdateContext, prelude::*};

pub(crate) enum EntityInner {
    Empty(Empty),
    Plane(Plane),
    Camera(Camera),
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
            EntityInner::Camera(ref camera) => camera.pos,
        }
    }

    fn inner_dir(&self) -> Vector {
        match self.inner {
            EntityInner::Empty(ref empty) => empty.dir,
            EntityInner::Plane(ref plane) => plane.dir(),
            EntityInner::Camera(ref camera) => camera.dir,
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
            EntityInner::Camera(ref mut camera) => {
                camera.pos = pos.into();
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
            EntityInner::Camera(ref mut camera) => {
                camera.dir = dir.into();
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

    pub fn z(&self) -> f32 {
        if let EntityInner::Camera(camera) = &self.inner {
            camera.z
        } else {
            0.0
        }
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

    pub fn set_z(&mut self, z: f32) {
        if let EntityInner::Camera(camera) = &mut self.inner {
            camera.z = z;
        }
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

    pub(crate) fn scripts_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn Script>> + use<'_> {
        self.scripts.iter_mut()
    }

    pub(crate) fn update(&mut self, scene: &mut Scene, delta_time: Duration, time: Duration) {
        let self_ptr = self as *mut Entity;
        let scripts = self.scripts.iter_mut().collect::<Vec<_>>();
        for script in scripts {
            let ctx = UpdateContext {
                entity: unsafe { &mut *self_ptr },
                scene,
                time,
                delta_time,
            };

            script.update(ctx);
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

impl From<Camera> for Entity {
    fn from(camera: Camera) -> Self {
        Self {
            relative_pos: camera.pos,
            relative_dir: camera.dir,
            parent: None,
            inner: EntityInner::Camera(camera),
            scripts: Vec::new(),
        }
    }
}
