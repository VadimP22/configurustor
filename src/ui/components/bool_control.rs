use crossterm::{style::Color, event::KeyCode};

use crate::{ui::{control_trait::Control, ui_component_trait::UiComponent}, input::{input_event::InputEvent, input_event_type_enum::InputEventType}};

use super::text_component::TextComponent;


#[derive(Debug)]
pub struct BoolControl {
    is_selected: bool,
    is_true: bool,
    key: String,
    checkbox_component: TextComponent,
    text_component: TextComponent
}


impl UiComponent for BoolControl {
    fn draw(&mut self, stdout: &mut std::io::Stdout) {
        self.checkbox_component.draw(stdout);
        self.text_component.draw(stdout);
    }

    fn smart_draw(&mut self, stdout: &mut std::io::Stdout) {
        self.checkbox_component.smart_draw(stdout);
        self.text_component.smart_draw(stdout);
    }

    fn get_width(&mut self) -> u16 {
        return self.text_component.get_width() + 4;
    }

    fn get_height(&mut self) -> u16 {
        return 1;
    }

    fn set_position(&mut self, x: u16, y: u16) {
        self.checkbox_component.set_position(x, y);
        self.text_component.set_position(x + 4, y);
    }

    fn set_color(&mut self, color: crossterm::style::Color) {
        self.checkbox_component.set_color(color);
        self.text_component.set_color(color);
    }
}


impl Control for BoolControl {
    fn on_input(&mut self, input_event: InputEvent) {
        if self.is_selected && matches!(input_event.input_event_type, InputEventType::Activate) {
            if self.is_true {
                self.is_true = false;
            } else {
                self.is_true = true;
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
}


impl BoolControl {
    pub fn default(key: &str, title: &str, x: u16, y: u16) -> BoolControl {
        let mut checkbox_component = TextComponent::default("[ ]", x, y);
        let mut text_component = TextComponent::default(title, x + 4, y);

        checkbox_component.set_color(Color::DarkGrey);
        text_component.set_color(Color::DarkGrey);

        return BoolControl {
            is_selected: false,
            is_true: false,
            key: key.to_string(),
            checkbox_component,
            text_component
        };
    }

    fn process(&mut self) {
        if self.is_selected {
            if self.is_true {
                self.set_color(Color::Blue);
                self.checkbox_component.set_text("[+]");
            } else {
                self.set_color(Color::DarkBlue);
                self.checkbox_component.set_text("[ ]");
            }
        } else {
            if self.is_true {
                self.set_color(Color::Grey);
                self.checkbox_component.set_text("[+]");
            } else {
                self.set_color(Color::DarkGrey);
                self.checkbox_component.set_text("[ ]");
            }
        }
    }
}