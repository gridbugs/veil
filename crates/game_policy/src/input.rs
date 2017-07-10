use frame::Frame;

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Quit,
    Escape,
    Return,
    Space,
}

pub struct ExternalEvent {
    input: Option<InputEvent>,
    frame: Option<Frame>,
}

impl ExternalEvent {
    pub fn new(input: InputEvent, frame: Frame) -> Self {
        ExternalEvent {
            input: Some(input),
            frame: Some(frame),
        }
    }

    pub fn with_input(input: InputEvent) -> Self {
        ExternalEvent {
            input: Some(input),
            frame: None,
        }
    }

    pub fn with_frame(frame: Frame) -> Self {
        ExternalEvent {
            input: None,
            frame: Some(frame),
        }
    }

    pub fn input(&self) -> Option<InputEvent> {
        self.input
    }

    pub fn frame(&self) -> Option<Frame> {
        self.frame
    }
}

pub trait GameInput {
    fn next_input(&mut self) -> InputEvent;
    fn next_frame(&mut self) -> Frame;
    fn next_external(&mut self) -> ExternalEvent;
}
