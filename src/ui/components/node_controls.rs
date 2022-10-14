use crossterm::{execute, style::{SetForegroundColor, Color, Print}, cursor};

use crate::{ui::{ui_component_trait::UiComponent, control_trait::Control, ui_event::UiEvent, ui_event_type_enum::UiEventType}, input::input_event::InputEvent};

use super::{bool_control::BoolControl, group_control::GroupControl, box_component::BoxComponent, text_component::TextComponent};

#[derive(Debug)]
pub struct NodeControls {
    selected_index: usize,
    x: u16,
    y: u16,
    have_changes_to_draw: bool,
    title: String,

    group_controls: Vec<GroupControl>,
    bool_controls: Vec<BoolControl>,
    box_component: BoxComponent,

    title_box: BoxComponent,
    title_text: TextComponent
}

impl UiComponent for NodeControls {
    fn draw(&mut self, stdout: &mut std::io::Stdout) {
        self.process();
        self.box_component.draw(stdout);
        self.title_box.draw(stdout);
        self.draw_t_symbols(stdout);
        self.title_text.draw(stdout);

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
            self.box_component.smart_draw(stdout);
            self.title_box.smart_draw(stdout);
            self.draw_t_symbols(stdout);
            self.title_text.smart_draw(stdout);

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
        return self.box_component.get_width();
    }

    fn get_height(&mut self) -> u16 {
        return return self.box_component.get_height();
    }

    fn set_position(&mut self, x: u16, y: u16) {

        for i in 0..self.group_controls.len() {
            let group_control = self.group_controls.get_mut(i).expect("error");
            group_control.set_position(2 + x, 3 + self.y + i as u16);


        }

        for j in 0..self.bool_controls.len() {
            let bool_control = self.bool_controls.get_mut(j).expect("error");
            bool_control.set_position(2 + x, 4 + self.y + self.group_controls.len() as u16 + j as u16);
        }

        self.box_component = BoxComponent::default(self.x, self.y, self.calculate_text_width() + 4, self.calculate_text_height() + 4);
        self.title_box = BoxComponent::default(self.x, self.y, self.calculate_text_width() + 4, 3);

        self.title_text.set_position(x + 2, y + 1);

        self.have_changes_to_draw = true;
    }

    fn set_color(&mut self, color: crossterm::style::Color) {
        self.have_changes_to_draw = true;
    }
}


impl NodeControls {
    pub fn default(x: u16, y: u16, title: &str) -> NodeControls {
        let mut title_text = TextComponent::default(title, x + 2, y + 1);
        title_text.set_color(Color::DarkGrey);

        return NodeControls {
            title: title.to_string(),
            selected_index: 0,
            x,
            y,
            have_changes_to_draw: true,
            group_controls: Vec::new(),
            bool_controls: Vec::new(),
            box_component: BoxComponent::default(x, y, title.len() as u16 + 2, 3),
            title_box: BoxComponent::default(x, y, title.len() as u16 + 2, 3),
            title_text
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

    pub fn get_ui_event() -> UiEvent {
        return UiEvent {
            event_type: UiEventType::NoEvent,
            key: "key".to_string(),
            new_value: false,
        }
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

    fn calculate_text_width(&mut self) -> u16 {
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

        if self.title.len() as u16 > max {
            max = self.title.len() as u16;
        }

        return max;
    }

    fn calculate_text_height(&mut self) -> u16 {
        return (self.group_controls.len() + self.bool_controls.len() + 1) as u16;
    }

    fn draw_t_symbols(&mut self, stdout: &mut std::io::Stdout) {
        execute!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            cursor::MoveTo(self.x, self.y + 2),
            Print('├'),
            cursor::MoveTo(self.x + self.get_width() - 1, self.y + 2),
            Print('┤'),
            cursor::Hide
        ).expect("error");
    }
}
