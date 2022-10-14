use crossterm::{style::{Color, SetForegroundColor, Print}, execute, cursor};

use crate::ui::ui_component_trait::UiComponent;


#[derive(Debug)]
pub struct BoxComponent {
    color: Color,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    have_changes_to_draw: bool
}


impl UiComponent for BoxComponent {
    fn draw(&mut self, stdout: &mut std::io::Stdout) {
        for x in 0..self.width {
            for y in 0..self.height {
                if y == 0 || y == self.height - 1{
                    execute!(
                        stdout,
                        SetForegroundColor(self.color),
                        cursor::MoveTo(self.x + x, self.y + y),
                        Print('─'),
                        cursor::Hide
                    ).expect("error");
                }

                if x == 0 || x == self.width - 1{
                    execute!(
                        stdout,
                        SetForegroundColor(self.color),
                        cursor::MoveTo(self.x + x, self.y + y),
                        Print('│'),
                        cursor::Hide
                    ).expect("error");
                }

                execute!(
                    stdout,
                    SetForegroundColor(self.color),
                    cursor::MoveTo(self.x, self.y),
                    Print('┌'),
                    cursor::MoveTo(self.x + self.width - 1, self.y),
                    Print('┐'),
                    cursor::MoveTo(self.x + self.width - 1, self.y + self.height - 1),
                    Print('┘'),
                    cursor::MoveTo(self.x, self.y + self.height - 1),
                    Print('└'),
                    cursor::Hide
                ).expect("error");
            }
        }
    }

    fn smart_draw(&mut self, stdout: &mut std::io::Stdout) {
        if self.have_changes_to_draw {
            self.draw(stdout);
            self.have_changes_to_draw = false;
        }
    }

    fn get_width(&mut self) -> u16 {
        return self.width;
    }

    fn get_height(&mut self) -> u16 {
        return self.height;
    }

    fn set_position(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
        self.have_changes_to_draw = true;
    }

    fn set_color(&mut self, color: crossterm::style::Color) {
        self.color = color;
        self.have_changes_to_draw = true
    }
}

impl BoxComponent {
    pub fn default(x: u16, y: u16, width: u16, height: u16) -> BoxComponent {
        return BoxComponent {
            color: Color::DarkGrey,
            x,
            y,
            width,
            height,
            have_changes_to_draw: true
        };
    }
}