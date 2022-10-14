use super::ui_event_type_enum::UiEventType;

#[derive(Debug, Clone)]
pub struct UiEvent {
    pub event_type: UiEventType,
    pub key: String,
    pub new_value: bool
}