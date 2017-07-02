use std::time::{Duration, Instant};
use std::thread;
use frame::{Frame, FrameId};

const MILLIS_PER_SEC: u32 = 1_000;

pub struct CommonInput {
    pub frame_duration: Duration,
    pub previous_frame_instant: Instant,
    pub next_frame_id: FrameId,
}

impl CommonInput {
    pub fn from_fps(fps: u32) -> Self {
        CommonInput {
            frame_duration: Duration::from_millis((MILLIS_PER_SEC / fps) as u64),
            previous_frame_instant: Instant::now(),
            next_frame_id: 0,
        }
    }

    pub fn next_frame_id(&mut self) -> FrameId {
        let frame_id = self.next_frame_id;
        self.next_frame_id += 1;
        frame_id
    }

    pub fn next_frame(&mut self, instant: Instant) -> Frame {
        let id = self.next_frame_id();
        self.previous_frame_instant = instant;
        Frame::new(id, instant)
    }

    pub fn wait_for_next_frame(&mut self) -> Frame {
        let mut now = Instant::now();
        let since_last_frame = now - self.previous_frame_instant;

        if let Some(remaining) = self.frame_duration.checked_sub(since_last_frame) {
            thread::sleep(remaining);
            now = Instant::now();
        }

        self.next_frame(now)
    }
}
