use crate::vector::Vector;

pub trait Entity {
    fn pos(&self) -> Vector;
    fn dir(&self) -> Vector;
}
