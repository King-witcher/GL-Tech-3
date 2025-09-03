use std::sync::Arc;

use crate::imaging::Image;

pub struct Texture {
    source: Image,
    hoffset: f32,
    voffset: f32,
    hrepeat: f32,
    vrepeat: f32,
}
