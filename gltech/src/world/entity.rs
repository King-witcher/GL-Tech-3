use std::ptr::NonNull;
use std::time::Duration;

use crate::engine::Input;
use crate::scripting::script::Script;
use crate::world::Plane;
use crate::world::empty::Empty;
use crate::{StartContext, SystemContext, UpdateContext, prelude::*};

pub(crate) enum EntityInner {
    Empty(NonNull<Empty>),
    Plane(NonNull<Plane>),
}

pub struct Entity {
    pub(crate) inner: EntityInner,
    parent: Option<NonNull<Entity>>,
    scene: Option<NonNull<Scene>>,
    children: Vec<NonNull<Entity>>,

    relative: Pose,
    scripts: Vec<Box<dyn Script>>,
}

impl Entity {
    pub fn add_script(&mut self, script: Box<dyn Script>) {
        self.scripts.push(script);
    }

    pub fn parent(&self) -> Option<&mut Entity> {
        match self.parent {
            Some(mut parent) => unsafe { Some(parent.as_mut()) },
            None => None,
        }
    }

    pub fn set_parent(&mut self, parent: Option<&mut Entity>) {
        if let Some(ref parent) = parent {
            assert!(
                self.scene == parent.scene,
                "Cannot set parent from a different scene"
            );
        }

        unsafe {
            let self_ptr = NonNull::new_unchecked(self as *mut Entity);

            // Remove from the previous parent's children list
            if let Some(parent) = self.parent() {
                parent.children.retain(|&c| c != self_ptr);
            }

            match parent {
                Some(new_parent) => {
                    new_parent.children.push(self_ptr);

                    self.parent = Some(NonNull::new_unchecked(new_parent as *mut Entity));
                }
                None => {
                    self.parent = None;
                }
            }
        }
    }

    fn inner_pose(&self) -> Pose {
        unsafe {
            match self.inner {
                EntityInner::Empty(empty) => empty.as_ref().pose,
                EntityInner::Plane(plane) => plane.as_ref().pose,
            }
        }
    }

    fn set_inner_pose(&mut self, pose: Pose) {
        unsafe {
            match self.inner {
                EntityInner::Empty(mut empty) => empty.as_mut().pose = pose,
                EntityInner::Plane(mut plane) => plane.as_mut().pose = pose,
            }
        }
    }

    /// Update relative components based on parent's and own absolute components.
    ///
    /// Called when either parent element or its absolute position changes.
    fn update_relative(&mut self) {
        match self.parent {
            Some(parent) => {
                let parent = unsafe { parent.as_ref() };
                self.relative = self.inner_pose().as_relative_to(parent.inner_pose());
            }
            None => {
                self.relative = self.inner_pose();
            }
        }
    }

    /// Update real components based on relative components and parent's real components, if any.
    fn follow_parent(&mut self) {
        match self.parent {
            Some(parent) => {
                println!("Following parent");
                let parent = unsafe { parent.as_ref() };
                self.set_inner_pose(self.relative.as_absolute_from(parent.inner_pose()));
            }
            None => {
                println!("Following none");
                self.set_inner_pose(self.relative);
            }
        }

        for child in self.children.iter_mut() {
            println!("Making child follow parent");
            dbg!(*child);
            unsafe { child.as_mut().follow_parent() };
        }
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

    pub(crate) fn tick(
        &mut self,
        scene: &mut Scene,
        time: Duration,
        delta_time: Duration,
        input: Input,
        system: &mut SystemContext,
    ) {
        let _ = time;
        let self_ptr = self as *mut Entity;
        let scripts = self.scripts.iter_mut().collect::<Vec<_>>();
        for script in scripts {
            let ctx = UpdateContext {
                entity: unsafe { &mut *self_ptr },
                input: input.clone(),
                time,
                delta_time,
                system,
                scene,
            };

            script.tick(ctx);
        }
    }
}

// TODO: Optimize
impl Posed for Entity {
    fn pose(&self) -> Pose {
        self.relative
    }

    fn set_pose(&mut self, pose: Pose) {
        self.relative = pose;
        self.follow_parent();
    }
}

impl From<Plane> for Entity {
    fn from(plane: Plane) -> Self {
        let relative = plane.pose;

        let inner = unsafe {
            let boxed = Box::new(plane);
            let raw = Box::into_raw(boxed);
            NonNull::new_unchecked(raw)
        };

        Self {
            relative,
            parent: None,
            scene: None,
            inner: EntityInner::Plane(inner),
            children: Vec::new(),
            scripts: Vec::new(),
        }
    }
}

impl From<Empty> for Entity {
    fn from(empty: Empty) -> Self {
        let relative = empty.pose;

        let inner = unsafe {
            let boxed = Box::new(empty);
            let raw = Box::into_raw(boxed);
            NonNull::new_unchecked(raw)
        };

        Self {
            relative,
            scene: None,
            parent: None,
            inner: EntityInner::Empty(inner),
            children: Vec::new(),
            scripts: Vec::new(),
        }
    }
}
