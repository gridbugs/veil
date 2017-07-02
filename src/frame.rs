use std::time::Instant;

pub type FrameId = u64;

#[derive(Debug, Clone, Copy)]
pub struct Frame {
    id: FrameId,
    instant: Instant,
}

impl Frame {
    pub fn id(&self) -> FrameId { self.id }
    pub fn instant(&self) -> Instant { self.instant }

    pub fn new(id: FrameId, instant: Instant) -> Self {
        Frame {
            id: id,
            instant: instant,
        }
    }

    pub fn now(id: FrameId) -> Self {
        Frame::new(id, Instant::now())
    }
}
