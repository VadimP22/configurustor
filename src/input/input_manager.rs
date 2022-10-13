use crossterm::event::{Event, KeyCode, KeyEvent};

use super::{input_event::InputEvent, input_event_type_enum::InputEventType};

pub struct InputManager {
    event: Event
}


impl InputManager {
    pub fn new() -> InputManager {
        return InputManager {
            event: Event::Key(KeyCode::Char('f').into())
        };
    }

    pub fn read_crossterm_event(&mut self) {
        self.event = crossterm::event::read().expect("error");

//        if event == Event::Key(KeyCode::Char('c').into()) {
//
//        }
    }

    pub fn to_input_event(&mut self) -> InputEvent {
        if self.event == Event::Key(KeyCode::Up.into()) {
            return InputEvent {
                key: ' ',
                input_event_type: InputEventType::Prev
            };
        } else if self.event == Event::Key(KeyCode::Down.into()){
            return InputEvent {
                key: ' ',
                input_event_type: InputEventType::Next
            };
        } else if self.event == Event::Key(KeyCode::Left.into()){
            return InputEvent {
                key: ' ',
                input_event_type: InputEventType::Out
            };
        } else if self.event == Event::Key(KeyCode::Right.into()){
            return InputEvent {
                key: ' ',
                input_event_type: InputEventType::In
            };
        } else if self.event == Event::Key(KeyCode::Char(' ').into()){
            return InputEvent {
                key: ' ',
                input_event_type: InputEventType::Activate
            };
        }
        else {
            match self.event {
                Event::Key(KeyEvent { code: KeyCode::Char(c), .. }) => return InputEvent {
                    key: c,
                    input_event_type: InputEventType::Key
                },
                _ => return InputEvent {
                    key: ' ',
                    input_event_type: InputEventType::NoKeyboardInput,
                }
            }

        }
    }
}