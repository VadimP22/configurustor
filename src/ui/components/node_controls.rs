use crate::{
    exit,
    input::{input_event::InputEvent, input_event_type_enum::InputEventType},
    ui::{control_component::ControlComponent, ui_component_trait::UiComponent},
};

use super::{bool_control::BoolControl, box_component::BoxComponent};

pub struct NodeControls {
    x: u16,
    y: u16,

    box_component: BoxComponent,
    controls: Vec<Box<dyn ControlComponent>>,
    selected_index: usize,

    have_changes_to_draw: bool,
}

impl UiComponent for NodeControls {
    fn draw(&mut self, stdout: &mut std::io::Stdout) {
        self.process_selection();
        self.box_component.draw(stdout);

        for control in &mut self.controls {
            control.draw(stdout);
        }

        self.box_component.draw(stdout);
    }

    fn smart_draw(&mut self, stdout: &mut std::io::Stdout) {
        if self.have_changes_to_draw {
            self.process_selection();

            self.box_component.smart_draw(stdout);
            for control in &mut self.controls {
                control.smart_draw(stdout);
            }

            self.have_changes_to_draw = false;
        }
    }

    fn get_width(&mut self) -> u16 {
        let mut max = 1;

        for control in &mut self.controls {
            if control.get_width() > max {
                max = control.get_width();
            }
        }

        return max;
    }

    fn get_height(&mut self) -> u16 {
        return self.controls.len() as u16;
    }

    fn set_position(&mut self, x: u16, y: u16) {
        todo!()
    }

    fn set_color(&mut self, color: crossterm::style::Color) {}
}

impl NodeControls {
    pub fn default(x: u16, y: u16) -> NodeControls {
        return NodeControls {
            x,
            y,
            box_component: BoxComponent::default(x, y, 1, 1),
            controls: Vec::new(),
            selected_index: 0,
            have_changes_to_draw: true,
        };
    }

    pub fn add_bool_control(&mut self, key: String, title: String) {
        self.controls.push(Box::new(BoolControl::default(
            key,
            title,
            self.x + 1,
            self.y + 1 + self.controls.len() as u16,
        )));

        self.update_box();
        self.have_changes_to_draw = true;
    }

    pub fn on_input(&mut self, input_event: InputEvent) {
        if input_event.key == 'q' {
            exit();
        }

        match input_event.input_event_type {
            InputEventType::Next => {
                self.selected_index += 1;
                if self.selected_index >= self.controls.len() {
                    self.selected_index = 0;
                }
            }
            InputEventType::Prev => {
                if self.selected_index >= 1 {
                    self.selected_index -= 1;
                } else {
                    self.selected_index = self.controls.len() - 1;
                }
            }
            _ => self
                .controls
                .get_mut(self.selected_index)
                .expect("error")
                .on_input(input_event),
        }

        self.have_changes_to_draw = true;
    }

    fn update_box(&mut self) {
        self.box_component =
            BoxComponent::default(self.x, self.y, self.get_width() + 2, self.get_height() + 2);

        self.have_changes_to_draw = true;
    }

    fn process_selection(&mut self) {
        for i in 0..self.controls.len() {
            let control = self.controls.get_mut(i).expect("error");
            if self.selected_index == i {
                if control.is_selectable() {
                    control.select();
                } else {
                    self.selected_index += 1;
                }
            } else {
                control.deselect();
            }
        }
    }
}
