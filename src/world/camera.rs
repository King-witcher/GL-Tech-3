use crate::{vector::Vector, world::Entity};

pub struct Camera {
    pub position: Vector,
    pub rotation: f32,
}

impl Entity for Camera {
    #[inline]
    fn pos(&self) -> Vector {
        self.position
    }

    #[inline]
    fn dir(&self) -> Vector {
        Vector::from_angle(self.rotation)
    }
}
