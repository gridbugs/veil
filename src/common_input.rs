use std::time::{Duration, Instant};
use std::thread;
use input::{GameInput, ExternalEvent, InputEvent};
use frame::{Frame, AnimationMode, FrameId};

pub trait ToInputEvent {
    fn to_input_event(self) -> Option<InputEvent>;
}

/// A common interface to most input sources
pub trait CommonInput {
    type Event: ToInputEvent;
    fn wait_event(&mut self) -> Self::Event;
    fn poll_event(&mut self) -> Option<Self::Event>;
    fn wait_event_timeout(&mut self, timeout: Duration) -> Option<Self::Event>;
    fn frame_duration(&self) -> Duration;
    fn previous_frame_instant(&self) -> Instant;
    fn set_previous_frame_instant(&mut self, instant: Instant);
    fn animation_mode(&self) -> AnimationMode;
    fn set_animation_mode(&mut self, mode: AnimationMode);

    fn next_frame_id(&mut self) -> FrameId;

    fn clear_events(&mut self) {
        while self.poll_event().is_some() {}
    }
    fn next_frame_internal(&mut self, instant: Instant) -> Frame {
        let id = self.next_frame_id();
        self.set_previous_frame_instant(instant);
        Frame::new(id, self.animation_mode(), instant)
    }
}

impl<I: CommonInput> GameInput for I {
    fn next_input(&mut self) -> InputEvent {
        loop {
            if let Some(input_event) = self.wait_event().to_input_event() {
                return input_event;
            }
        }
    }
    fn next_frame(&mut self) -> Frame {
        let mut now = Instant::now();
        let since_last_frame = now - self.previous_frame_instant();

        if let Some(remaining) = self.frame_duration().checked_sub(since_last_frame) {
            thread::sleep(remaining);
            now = Instant::now();
        }

        self.clear_events();

        self.next_frame_internal(now)
    }
    fn next_external(&mut self) -> ExternalEvent {
        if self.animation_mode() == AnimationMode::TurnBased {
            let input = self.next_input();
            let frame = self.next_frame_internal(Instant::now());
            return ExternalEvent::new(input, frame);
        }
        loop {
            let now = Instant::now();
            let since_last_frame = now - self.previous_frame_instant();
            if let Some(remaining) = self.frame_duration().checked_sub(since_last_frame) {
                if let Some(event) = self.wait_event_timeout(remaining) {
                    if let Some(input_event) = event.to_input_event() {
                        return ExternalEvent::with_input(input_event);
                    }
                } else {
                    let frame = self.next_frame_internal(Instant::now());
                    return ExternalEvent::with_frame(frame);
                }
            } else {
                let frame = self.next_frame_internal(now);

                if let Some(event) = self.poll_event() {
                    if let Some(input_event) = event.to_input_event() {
                        return ExternalEvent::new(input_event, frame);
                    }
                }

                return ExternalEvent::with_frame(frame);
            }
        }
    }
}
