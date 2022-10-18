use crossterm::style::Color;

use crate::{
    input::{input_event::InputEvent, input_event_type_enum::InputEventType},
    ui::{control_trait::Control, ui_component_trait::UiComponent},
};

use super::text_component::TextComponent;

#[derive(Debug)]
pub struct GroupControl {
    is_selected: bool,
    is_active: bool,
    pub key: String,
    text_component: TextComponent,
}

impl UiComponent for GroupControl {
    fn draw(&mut self, stdout: &mut std::io::Stdout) {
        self.text_component.draw(stdout);
    }

    fn smart_draw(&mut self, stdout: &mut std::io::Stdout) {
        self.text_component.smart_draw(stdout);
    }

    fn get_width(&mut self) -> u16 {
        return self.text_component.get_width();
    }

    fn get_height(&mut self) -> u16 {
        return 1;
    }

    fn set_position(&mut self, x: u16, y: u16) {
        self.text_component.set_position(x, y);
    }

    fn set_color(&mut self, color: crossterm::style::Color) {
        self.text_component.set_color(color);
    }
}

impl Control for GroupControl {
    fn on_input(&mut self, input_event: InputEvent) {
        if self.is_selected {
            if matches!(input_event.input_event_type, InputEventType::In)
                || matches!(input_event.input_event_type, InputEventType::Activate)
            {
                self.is_active = true;
            }
        }
        self.process();
    }

    fn select(&mut self) {
        self.is_selected = true;
        self.process();
    }

    fn deselect(&mut self) {
        self.is_selected = false;
        self.process();
    }

    fn is_selectable(&mut self) -> bool {
        return true;
    }

    fn get_state(&mut self) -> (String, String) {
        return ("group".to_string(), "group".to_string());
    }
}

impl GroupControl {
    pub fn default(key: String, title: String, x: u16, y: u16) -> GroupControl {
        let mut text_component = TextComponent::default(title, x, y);
        text_component.set_color(Color::DarkGrey);

        return GroupControl {
            is_selected: false,
            is_active: false,
            key: key.to_string(),
            text_component,
        };
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    fn process(&mut self) {
        if self.is_selected {
            self.set_color(Color::DarkBlue);
        } else {
            if self.is_active {
                self.text_component.set_color(Color::Grey);
            } else {
                self.text_component.set_color(Color::DarkGrey);
            }
        }
    }
}
