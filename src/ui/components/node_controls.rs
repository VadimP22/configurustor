use crossterm::{execute, style::{SetForegroundColor, Color, Print}, cursor};

use crate::{ui::{ui_component_trait::UiComponent, control_trait::Control, ui_event::UiEvent, ui_event_type_enum::UiEventType}, input::{input_event::InputEvent, input_event_type_enum::InputEventType}, exit};

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
    title_text: TextComponent,

    ui_event: UiEvent
}

impl UiComponent for NodeControls {
    fn draw(&mut self, stdout: &mut std::io::Stdout) {
        self.process();
        self.box_component.draw(stdout);
        self.title_box.draw(stdout);
        self.draw_t_symbols(stdout);
        self.title_text.draw(stdout);
        self.draw_separator(stdout);

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
            self.draw_separator(stdout);

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
        return self.box_component.get_height();
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
        panic!("You can't set color of NodeControls!");
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
            title_text,
            ui_event: UiEvent::no_event()
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

        match input_event.input_event_type {
            InputEventType::Next => {
                self.selected_index += 1;
                if self.selected_index >= (self.group_controls.len() + self.bool_controls.len()) {
                    self.selected_index = 0;
                }
            },

            InputEventType::Prev => {
                if self.selected_index == 0 {
                    self.selected_index = self.group_controls.len() + self.bool_controls.len();
                }
                self.selected_index -= 1;
            },

            InputEventType::Activate => {
                if self.selected_index < self.group_controls.len() {
                    return;
                }
                let index = self.selected_index - self.group_controls.len();
                let selected_bool_control = self.bool_controls.get_mut(index).expect("error");

                selected_bool_control.on_input(input_event);
                self.ui_event = UiEvent::set(selected_bool_control.key.clone(), selected_bool_control.is_true);
            },

            InputEventType::In => {
                if self.selected_index >= self.group_controls.len() {
                    return;
                }
                let selected_group_control = self.group_controls.get_mut(self.selected_index).expect("error");

                selected_group_control.on_input(input_event);
                self.ui_event = UiEvent::navigate(selected_group_control.key.clone());
            },

            InputEventType::Key => {
                if input_event.key == 'q' {
                    exit();
                }
            }
            _ => {}
        }
        self.have_changes_to_draw = true;
    }

    pub fn get_ui_event(&mut self) -> UiEvent {
        let event = self.ui_event.clone();
        self.ui_event = UiEvent::no_event();
        return event;
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
            if self.selected_index == index - 1 {
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

    fn draw_separator(&mut self, stdout: &mut std::io::Stdout) {
        for i in 0..self.get_width() {
            execute!(
                stdout,
                SetForegroundColor(Color::DarkGrey),
                cursor::MoveTo(self.x + i, 3 + self.y + self.group_controls.len() as u16),
                Print('─'),
                cursor::Hide
            ).expect("error");
        }

        execute!(
                stdout,
                SetForegroundColor(Color::DarkGrey),
                cursor::MoveTo(self.x, 3 + self.y + self.group_controls.len() as u16),
                Print('├'),
                cursor::MoveTo(self.x + self.get_width() - 1, 3 + self.y + self.group_controls.len() as u16),
                Print('┤'),
                cursor::Hide
        ).expect("error");
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
