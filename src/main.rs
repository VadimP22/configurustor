use std::{io::stdout, thread::Thread, time::Duration, process::exit};

use input::input_manager::InputManager;
use ui::{ui_component_trait::UiComponent, components::{text_component::TextComponent, bool_control::BoolControl, node_controls::NodeControls}, control_trait::Control};
use crossterm::{execute, terminal, event::{Event, KeyCode}};

mod ui;
mod input;

fn main() {
    let mut stdo = stdout();
    let mut input_manager = InputManager::new();
    let mut node_controls = NodeControls::default(1, 1);

    node_controls.add_group_control("key1", "Common setting");
    node_controls.add_group_control("key2", "Additional setting");
    node_controls.add_bool_control("key3", "Enable if you have a cat!");
    node_controls.add_bool_control("key3", "Yet another control...");
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
