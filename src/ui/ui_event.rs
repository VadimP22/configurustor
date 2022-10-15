use super::ui_event_type_enum::UiEventType;

#[derive(Debug, Clone)]
pub struct UiEvent {
    pub event_type: UiEventType,
    pub key: String,
    pub new_value: bool
}


impl UiEvent {
    pub fn no_event() -> UiEvent {
        return UiEvent {
            event_type: UiEventType::NoEvent,
            key: "no_key".to_string(),
            new_value: false,
        }
    }

    pub fn navigate(key: String) -> UiEvent {
        return UiEvent {
            event_type: UiEventType::Navigate,
            key,
            new_value: false
        }
    }

    pub fn set(key: String, new_value: bool) -> UiEvent {
        return UiEvent {
            event_type: UiEventType::Set,
            key,
            new_value,
        }
    }
}