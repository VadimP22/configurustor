use crate::{ui::{ui_component_trait::UiComponent, control_trait::Control}, input::{input_event::InputEvent, input_event_type_enum::InputEventType}};

use super::bool_control::BoolControl;


#[derive(Debug)]
pub struct BoolControlsList {
    selected_index: usize,
    x: u16,
    y: u16,
    have_changes_to_draw: bool,
    bool_controls: Vec<BoolControl>
}


impl UiComponent for BoolControlsList {
    fn draw(&mut self, stdout: &mut std::io::Stdout) {
        self.process();
        for control in &mut self.bool_controls {
            control.draw(stdout)
        }
    }

    fn smart_draw(&mut self, stdout: &mut std::io::Stdout) {
        self.process();
        if self.have_changes_to_draw {
            for control in &mut self.bool_controls {
                control.smart_draw(stdout)
            }
        }
    }

    fn get_width(&mut self) -> u16 {
        let mut max: u16 = 0;

        for control in &mut self.bool_controls {
            if control.get_width() > max {
                max = control.get_width()
            }
        }

        return max;
    }

    fn get_height(&mut self) -> u16 {
        return self.bool_controls.len() as u16;
    }

    fn set_position(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
        for i in 0..self.bool_controls.len() {
            self.bool_controls.get_mut(i).expect("error").set_position(x, y + (i as u16));
        }
    }

    fn set_color(&mut self, color: crossterm::style::Color) {
        for control in &mut self.bool_controls {
            control.set_color(color);
        }
    }
}


impl BoolControlsList {
    pub fn default(x: u16, y: u16) -> BoolControlsList {
        return BoolControlsList {
            selected_index: 0,
            x,
            y,
            have_changes_to_draw: true,
            bool_controls: Vec::new()
        }
    }

    pub fn add_bool_control(&mut self, key: &str, title: &str) {
        self.bool_controls.push(BoolControl::default(key, title, self.x, self.y + (self.bool_controls.len() as u16)));
        self.have_changes_to_draw = true;
    }

    pub fn on_input(&mut self, input_event: InputEvent) {
        if matches!(input_event.input_event_type, InputEventType::Next) {
            self.selected_index = (self.selected_index + 1) % self.bool_controls.len();
            self.have_changes_to_draw = true;
        } else if matches!(input_event.input_event_type, InputEventType::Prev) {
            if self.selected_index > 0 {
                self.selected_index -= 1;
            } else if self.selected_index == 0 {
                self.selected_index = self.bool_controls.len() - 1;
            }
            self.have_changes_to_draw = true;
        } else if matches!(input_event.input_event_type, InputEventType::Activate) {
            self.bool_controls.get_mut(self.selected_index).expect("error").on_input(input_event);
        }
    }

    fn process(&mut self) {
        for i in 0..self.bool_controls.len() {
            let control = self.bool_controls.get_mut(i).expect("error");
            if self.selected_index == i {
                control.select();
            } else {
                control.deselect();
            }
        }
    }
}