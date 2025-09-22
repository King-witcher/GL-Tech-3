use crate::{Scene, world::Entity};
use sdl2::event::Event;
use std::rc::Rc;

pub struct StartContext;

pub struct UpdateContext<'a> {
    pub entity: &'a mut Entity,
    pub scene: &'a mut Scene,
}

pub struct EndContext;

pub trait Script {
    fn start(&mut self, ctx: &StartContext);
    fn update(&mut self, ctx: UpdateContext);
    fn end(&mut self, ctx: &EndContext);
}
