use crate::{Scene, engine::Input, world::Entity};
pub struct StartContext;

pub struct UpdateContext<'a> {
    pub entity: &'a mut Entity,
    pub input: Input,
    pub scene: &'a mut Scene,
}

pub struct EndContext;

pub trait Script {
    fn start(&mut self, ctx: &StartContext);
    fn update(&mut self, ctx: UpdateContext);
    fn end(&mut self, ctx: &EndContext);
}
