use crate::input::input_event::InputEvent;

pub trait Control {
    fn deselect(&mut self);
    fn is_selectable(&mut self) -> bool;
    fn on_input(&mut self, input_event: InputEvent);
    fn select(&mut self);
    /// return (key, value)
    fn get_state(&mut self) -> (String, String);
}
