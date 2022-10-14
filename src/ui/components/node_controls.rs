use crate::{ui::{ui_component_trait::UiComponent, control_trait::Control}, input::input_event::InputEvent};

use super::{bool_control::BoolControl, group_control::GroupControl};

#[derive(Debug)]
pub struct NodeControls {
    selected_index: usize,
    x: u16,
    y: u16,
    have_changes_to_draw: bool,

    group_controls: Vec<GroupControl>,
    bool_controls: Vec<BoolControl>
}

impl UiComponent for NodeControls {
    fn draw(&mut self, stdout: &mut std::io::Stdout) {
        self.process();
        for group_control in &mut self.group_controls {
            group_control.draw(stdout);
        }

        for bool_control in &mut self.bool_controls {
            bool_control.draw(stdout);
        }
    }

    fn smart_draw(&mut self, stdout: &mut std::io::Stdout) {
        if self.have_changes_to_draw {
            self.process();
            for group_control in &mut self.group_controls {
                group_control.smart_draw(stdout);
            }

            for bool_control in &mut self.bool_controls {
                bool_control.smart_draw(stdout);
            }
            self.have_changes_to_draw = false;
        }
    }

    fn get_width(&mut self) -> u16 {
        let mut max: u16 = 0;
        for group_control in &mut self.group_controls {
            if group_control.get_width() > max {
                max = group_control.get_width();
            }
        }

        for bool_control in &mut self.bool_controls {
            if bool_control.get_width() > max {
                max = bool_control.get_width();
            }
        }

        return max;
    }

    fn get_height(&mut self) -> u16 {
        return (self.group_controls.len() + self.bool_controls.len()) as u16;
    }

    fn set_position(&mut self, x: u16, y: u16) {

        for i in 0..self.group_controls.len() {
            let group_control = self.group_controls.get_mut(i).expect("error");
            group_control.set_position(x, self.y + i as u16);


        }

        for j in 0..self.bool_controls.len() {
            let bool_control = self.bool_controls.get_mut(j).expect("error");
            bool_control.set_position(x, 1 + self.y + self.group_controls.len() as u16 + j as u16);
        }
        self.have_changes_to_draw = true;
    }

    fn set_color(&mut self, color: crossterm::style::Color) {
        self.have_changes_to_draw = true;
    }
}


impl NodeControls {
    pub fn default(x: u16, y: u16) -> NodeControls {
        return NodeControls {
            selected_index: 0,
            x,
            y,
            have_changes_to_draw: true,
            group_controls: Vec::new(),
            bool_controls: Vec::new()
        };
    }

    pub fn add_group_control(&mut self, key: &str, title: &str) {
        self.group_controls.push(GroupControl::default(key, title, 0, 0));
        self.set_position(self.x, self.y);
        self.have_changes_to_draw = true;
    }

    pub fn add_bool_control(&mut self, key: &str, title: &str) {
        self.bool_controls.push(BoolControl::default(key, title, 0, 0));
        self.set_position(self.x, self.y);
        self.have_changes_to_draw = true;
    }

    pub fn on_input(&mut self, input_event: InputEvent) {
        // do something
        self.have_changes_to_draw = true;
    }

    fn process(&mut self) {
        for i in 0..self.group_controls.len() {
            let group_control = self.group_controls.get_mut(i).expect("error");
            if (i == self.selected_index) {
                group_control.select();
            } else {
                group_control.deselect()
            }
        }

        for j in 0..self.bool_controls.len() {
            let index = j + self.bool_controls.len();
            let bool_control = self.bool_controls.get_mut(j).expect("error");
            if self.selected_index == index {
                bool_control.select();
            } else {
                bool_control.deselect();
            }
        }
    }
}
