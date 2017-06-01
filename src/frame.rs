use std::time::Instant;

pub type FrameId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationMode {
    RealTime,
    TurnBased,
}

#[derive(Debug, Clone, Copy)]
pub struct Frame {
    id: FrameId,
    instant: Instant,
    animation_mode: AnimationMode,
}

impl Frame {
    pub fn id(&self) -> FrameId { self.id }
    pub fn instant(&self) -> Instant { self.instant }
    pub fn animation_mode(&self) -> AnimationMode { self.animation_mode }

    pub fn new(id: FrameId, animation_mode: AnimationMode, instant: Instant) -> Self {
        Frame {
            id: id,
            instant: instant,
            animation_mode: animation_mode,
        }
    }
}
