use std::time;

pub type FrameId = u64;

#[derive(Debug)]
pub struct Frame {
    id: FrameId,
    instant: time::Instant,
}

impl Frame {
    pub fn id(&self) -> FrameId { self.id }
    pub fn instant(&self) -> time::Instant { self.instant }

    pub fn new(id: FrameId, instant: time::Instant) -> Self {
        Frame {
            id: id,
            instant: instant,
        }
    }

    pub fn new_now(id: FrameId) -> Self {
        Frame {
            id: id,
            instant: time::Instant::now(),
        }
    }
}
