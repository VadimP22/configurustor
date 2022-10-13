use super::input_event_type_enum::InputEventType;


#[derive(Copy, Clone, Debug)]
pub struct InputEvent {
    pub key: char,
    pub input_event_type: InputEventType
}