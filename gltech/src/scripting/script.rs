use std::time::Duration;

use crate::world::Entity;

pub struct StartContext;
pub struct UpdateContext<'a> {
    pub entity: &'a mut Entity<'a>,
    pub time: Duration,
    pub delta_time: Duration,
}
pub struct EndContext;

pub trait Script {
    fn start(&mut self, ctx: &StartContext);
    fn update(&mut self, ctx: &mut UpdateContext);
    fn end(&mut self, ctx: &EndContext);
}
