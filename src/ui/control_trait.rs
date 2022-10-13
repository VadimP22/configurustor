use crate::input::input_event::InputEvent;

pub trait Control {
    fn on_input(&mut self, input_event: InputEvent);
    fn select(&mut self);
    fn deselect(&mut self);
}