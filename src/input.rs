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

pub enum ExternalEvent {
    Input(InputEvent),
    Frame(Frame),
    InputAndFrame(InputEvent, Frame),
}

pub trait GameInput {
    fn next_input(&mut self) -> InputEvent;
    fn next_frame(&mut self) -> Frame;
    fn next_external(&mut self) -> ExternalEvent;
}
