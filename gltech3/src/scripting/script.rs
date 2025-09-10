use crate::world::EntityNode;

pub struct StartContext;
pub struct UpdateContext<'a> {
    pub entity: &'a mut EntityNode,
}
pub struct EndContext;

pub trait Script {
    fn start(&mut self, ctx: &StartContext);
    fn update(&mut self, ctx: &mut UpdateContext);
    fn end(&mut self, ctx: &EndContext);
}
