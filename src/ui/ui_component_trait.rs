use std::io::Stdout;

use crossterm::style::Color;

pub trait UiComponent {
    fn draw(&mut self, stdout: &mut Stdout);
    fn smart_draw(&mut self, stdout: &mut Stdout);

    fn get_width(&mut self) -> u16;
    fn get_height(&mut self) -> u16;

    fn set_position(&mut self, x: u16, y: u16);
    fn set_color(&mut self, color: Color);
}
