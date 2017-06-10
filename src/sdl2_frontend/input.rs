use std::time::{Duration, Instant};
use std::thread;
use sdl2::EventPump;
use sdl2::keyboard::{self, Keycode, Mod};
use sdl2::event::Event;
use input::{GameInput, InputEvent, ExternalEvent};
use frame::{Frame, FrameId, AnimationMode};

const MILLIS_PER_SEC: u32 = 1_000;
const NANOS_PER_MILLI: u32 = 1_000_000;

const MIN_WAIT_TIME_MS: u32 = 0;

pub struct SdlGameInput {
    event_pump: EventPump,
    frame_duration: Duration,
    previous_frame_instant: Instant,
    next_frame_id: FrameId,
    animation_mode: AnimationMode,
}

impl SdlGameInput {
    pub fn new(event_pump: EventPump, fps: u32, animation_mode: AnimationMode) -> Self {
        SdlGameInput {
            event_pump: event_pump,
            frame_duration: Duration::from_millis((MILLIS_PER_SEC / fps) as u64),
            previous_frame_instant: Instant::now(),
            next_frame_id: 0,
            animation_mode: animation_mode,
        }
    }

    fn next_frame_id(&mut self) -> FrameId {
        let frame_id = self.next_frame_id;
        self.next_frame_id += 1;
        frame_id
    }

    fn next_frame_internal(&mut self, instant: Instant) -> Frame {
        let id = self.next_frame_id();
        self.previous_frame_instant = instant;
        Frame::new(id, self.animation_mode, instant)
    }
}

fn is_shift_pressed(keymod: &Mod) -> bool {
    keymod.contains(keyboard::LSHIFTMOD) ||
        keymod.contains(keyboard::RSHIFTMOD)
}

fn to_char_event(ch: char, keymod: &Mod) -> Option<InputEvent> {
    if ch.is_alphabetic() {
        if is_shift_pressed(keymod) {
            let chars = ch.to_uppercase().collect::<Vec<char>>();
            return Some(InputEvent::Char(chars[0]));
        } else {
            // ch must be lowercase
            return Some(InputEvent::Char(ch));
        }
    }

    let translated_ch = if is_shift_pressed(keymod) {
        match ch {
            '1' => '!',
            '2' => '@',
            '3' => '#',
            '4' => '$',
            '5' => '%',
            '6' => '^',
            '7' => '&',
            '8' => '*',
            '9' => '(',
            '0' => ')',
            '.' => '>',
            ',' => '<',
            '/' => '?',
            _ => return None,
        }
    } else {
        ch
    };

    Some(InputEvent::Char(translated_ch))
}

fn keycode_to_event(keycode: Keycode, keymod: &Mod) -> Option<InputEvent> {
    match keycode {
        Keycode::Up => Some(InputEvent::Up),
        Keycode::Down => Some(InputEvent::Down),
        Keycode::Left => Some(InputEvent::Left),
        Keycode::Right => Some(InputEvent::Right),
        Keycode::Space => Some(InputEvent::Space),
        Keycode::Escape => Some(InputEvent::Escape),
        Keycode::Return => Some(InputEvent::Return),
        Keycode::A => to_char_event('a', keymod),
        Keycode::B => to_char_event('b', keymod),
        Keycode::C => to_char_event('c', keymod),
        Keycode::D => to_char_event('d', keymod),
        Keycode::E => to_char_event('e', keymod),
        Keycode::F => to_char_event('f', keymod),
        Keycode::G => to_char_event('g', keymod),
        Keycode::H => to_char_event('h', keymod),
        Keycode::I => to_char_event('i', keymod),
        Keycode::J => to_char_event('j', keymod),
        Keycode::K => to_char_event('k', keymod),
        Keycode::L => to_char_event('l', keymod),
        Keycode::M => to_char_event('m', keymod),
        Keycode::N => to_char_event('n', keymod),
        Keycode::O => to_char_event('o', keymod),
        Keycode::P => to_char_event('p', keymod),
        Keycode::Q => to_char_event('q', keymod),
        Keycode::R => to_char_event('r', keymod),
        Keycode::S => to_char_event('s', keymod),
        Keycode::T => to_char_event('t', keymod),
        Keycode::U => to_char_event('u', keymod),
        Keycode::V => to_char_event('v', keymod),
        Keycode::W => to_char_event('w', keymod),
        Keycode::X => to_char_event('x', keymod),
        Keycode::Y => to_char_event('y', keymod),
        Keycode::Z => to_char_event('z', keymod),
        Keycode::Num0 => to_char_event('0', keymod),
        Keycode::Num1 => to_char_event('1', keymod),
        Keycode::Num2 => to_char_event('2', keymod),
        Keycode::Num3 => to_char_event('3', keymod),
        Keycode::Num4 => to_char_event('4', keymod),
        Keycode::Num5 => to_char_event('5', keymod),
        Keycode::Num6 => to_char_event('6', keymod),
        Keycode::Num7 => to_char_event('7', keymod),
        Keycode::Num8 => to_char_event('8', keymod),
        Keycode::Num9 => to_char_event('9', keymod),
        Keycode::Period => to_char_event('.', keymod),
        Keycode::Comma => to_char_event(',', keymod),
        Keycode::Slash => to_char_event('/', keymod),
        _ => None,
    }
}

fn convert_event(event: Event) -> Option<InputEvent> {
    match event {
        Event::Quit { .. } => return Some(InputEvent::Quit),
        Event::KeyDown { keycode: Some(keycode), ref keymod, .. } => {
            return keycode_to_event(keycode, keymod);
        }
        _ => return None,
    }
}

impl GameInput for SdlGameInput {
    fn next_input(&mut self) -> InputEvent {
        loop {
            let event = self.event_pump.wait_event();
            if let Some(input_event) = convert_event(event) {
                return input_event;
            }
        }
    }

    fn next_frame(&mut self) -> Frame {
        let mut now = Instant::now();
        let since_last_frame = now - self.previous_frame_instant;

        if let Some(remaining) = self.frame_duration.checked_sub(since_last_frame) {
            thread::sleep(remaining);
            now = Instant::now();
        }

        // drain pending input
        while self.event_pump.poll_event().is_some() {}

        self.next_frame_internal(now)
    }

    fn next_external(&mut self) -> ExternalEvent {
        if self.animation_mode == AnimationMode::TurnBased {
            let input = self.next_input();
            let frame = self.next_frame_internal(Instant::now());
            return ExternalEvent::new(input, frame);
        }
        loop {
            let now = Instant::now();
            let since_last_frame = now - self.previous_frame_instant;
            if let Some(remaining) = self.frame_duration.checked_sub(since_last_frame) {
                let timeout_ms = remaining.as_secs() as u32 * MILLIS_PER_SEC +
                    remaining.subsec_nanos() / NANOS_PER_MILLI;
                if let Some(event) = self.event_pump.wait_event_timeout(timeout_ms) {
                    if let Some(input_event) = convert_event(event) {
                        return ExternalEvent::with_input(input_event);
                    }
                } else {
                    let frame = self.next_frame_internal(Instant::now());
                    return ExternalEvent::with_frame(frame);
                }
            } else {
                let frame = self.next_frame_internal(now);

                if let Some(event) = self.event_pump.poll_event() {
                    if let Some(input_event) = convert_event(event) {
                        return ExternalEvent::new(input_event, frame);
                    }
                }

                return ExternalEvent::with_frame(frame);
            }
        }
    }
}
