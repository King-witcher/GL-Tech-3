use std::time::{Duration, Instant};

static mut START: Option<Instant> = None;
static mut LAST: Option<Instant> = None;

pub(crate) fn init_time() {
    unsafe {
        START = Some(Instant::now());
        LAST = Some(Instant::now());
    }
}

pub(crate) fn reset_frame() {
    unsafe {
        LAST = Some(Instant::now());
    }
}

pub(crate) fn clear_time() {
    unsafe {
        START = None;
        LAST = None;
    }
}

pub fn delta_time() -> Duration {
    unsafe {
        let last = LAST.expect("called delta_time() while time was not initizlied");
        last.elapsed()
    }
}

pub fn time() -> Duration {
    unsafe {
        let start = START.expect("called time() while time was not initizlied");
        start.elapsed()
    }
}
