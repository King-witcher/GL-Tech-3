use std::rc::Rc;

use sdl2::event::Event;

#[derive(Debug, Clone)]
pub struct FrameEvents(Rc<[Event]>);

impl FromIterator<Event> for FrameEvents {
    fn from_iter<T: IntoIterator<Item = Event>>(iter: T) -> Self {
        FrameEvents(Rc::from_iter(iter))
    }
}

impl FrameEvents {
    pub fn iter(&self) -> impl Iterator<Item = &Event> {
        self.0.iter()
    }
}
