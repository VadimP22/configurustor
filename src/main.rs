use std::{io::stdout};

use input::input_manager::InputManager;
use ui::{ui_component_trait::UiComponent, components::{node_controls::NodeControls}};
use crossterm::{execute, terminal::{self, enable_raw_mode, disable_raw_mode}, cursor, style};


mod ui;
mod input;
mod store;


fn exit() {
    disable_raw_mode().expect("error");
    execute!(stdout(), style::ResetColor, cursor::Show, terminal::LeaveAlternateScreen, terminal::Clear(terminal::ClearType::All)).expect("error");

    std::process::exit(0);
}


fn main() {
    let mut stdo = stdout();
    let mut input_manager = InputManager::new();
    let mut node_controls = NodeControls::default(1, 1, "General settings!");

    node_controls.add_group_control("key1", "Cat settings");
    node_controls.add_group_control("key2", "CPU settings");
    node_controls.add_group_control("key3", "(DANGER) System settings");

    node_controls.add_bool_control("key4", "Do you have a cat?");
    node_controls.add_bool_control("key5", "Just bool control.");
    node_controls.add_bool_control("key6", "Yet another boolean control...");
    node_controls.add_bool_control("key7", "Make me happy :)");

    enable_raw_mode().expect("error");

    execute!(
        stdo,
        terminal::Clear(terminal::ClearType::All)
    ).expect("error");

    loop {
        node_controls.smart_draw(&mut stdo);

        input_manager.read_crossterm_event();

        node_controls.on_input(input_manager.to_input_event());
    }
}
