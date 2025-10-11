use crate::{Scene, SystemContext, engine::Input, world::Entity};

pub struct StartContext<'a> {
    pub system: &'a mut SystemContext,
    pub entity: &'a mut Entity,
    pub scene: &'a mut Scene,
}

pub struct UpdateContext<'a> {
    pub system: &'a mut SystemContext,
    pub entity: &'a mut Entity,
    pub input: Input,
    pub scene: &'a mut Scene,
}

pub struct EndContext<'a> {
    pub system: &'a mut SystemContext,
    pub entity: &'a mut Entity,
    pub scene: &'a mut Scene,
}

pub trait Script {
    fn start(&mut self, ctx: StartContext);
    fn tick(&mut self, ctx: UpdateContext);
    fn end(&mut self, ctx: EndContext);
}
