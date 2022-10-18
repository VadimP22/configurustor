use std::io::Stdout;

use crossterm::{
    cursor, execute,
    style::{Color, Print, SetForegroundColor},
};

use crate::ui::ui_component_trait::UiComponent;

#[derive(Debug)]
pub struct TextComponent {
    color: Color,
    x: u16,
    y: u16,
    text: String,
    have_changes_to_draw: bool,
}

impl UiComponent for TextComponent {
    fn draw(&mut self, stdout: &mut Stdout) {
        execute!(
            stdout,
            SetForegroundColor(self.color),
            cursor::MoveTo(self.x, self.y),
            Print(self.text.to_owned()),
            cursor::Hide
        )
        .expect("error");
    }

    fn smart_draw(&mut self, stdout: &mut Stdout) {
        if self.have_changes_to_draw {
            self.draw(stdout);
            self.have_changes_to_draw = false;
        }
    }

    fn get_width(&mut self) -> u16 {
        return self.text.len() as u16;
    }

    fn get_height(&mut self) -> u16 {
        return 1;
    }

    fn set_position(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;

        self.have_changes_to_draw = true;
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;

        self.have_changes_to_draw = true;
    }
}

impl TextComponent {
    pub fn default(text: String, x: u16, y: u16) -> TextComponent {
        return TextComponent {
            color: Color::Cyan,
            text,
            x,
            y,
            have_changes_to_draw: true,
        };
    }

    pub fn set_text(&mut self, new_text: &str) {
        self.text = new_text.to_string();
        self.have_changes_to_draw = true;
    }
}
