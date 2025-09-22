use std::time::Duration;

use crate::{Scene, world::Entity};

pub struct StartContext;
pub struct UpdateContext<'a> {
    pub scene: &'a mut Scene,
    pub entity: &'a mut Entity,
    pub time: Duration,
    pub delta_time: Duration,
}
pub struct EndContext;

pub trait Script {
    fn start(&mut self, ctx: &StartContext);
    fn update(&mut self, ctx: UpdateContext);
    fn end(&mut self, ctx: &EndContext);
}
