use std::{io::stdout, thread::Thread, time::Duration, process::exit};

use input::input_manager::InputManager;
use ui::{ui_component_trait::UiComponent, components::{text_component::TextComponent, bool_control::BoolControl, bool_controls_list::BoolControlsList}, control_trait::Control};
use crossterm::{execute, terminal, event::{Event, KeyCode}};

mod ui;
mod input;

fn main() {
    let mut stdo = stdout();
    let mut input_manager = InputManager::new();
    let mut bool_control_list = BoolControlsList::default(1, 1);

    bool_control_list.add_bool_control("key0", "Hello, my friend");
    bool_control_list.add_bool_control("key1", "Just toggle this boolean controller");
    bool_control_list.add_bool_control("key2", "Or this!");
    bool_control_list.add_bool_control("key3", "Yet another control...");
    bool_control_list.add_bool_control("key4", "Toggle me if you like cats!");
    bool_control_list.add_bool_control("key5", "Achtung!");

    execute!(
        stdo,
        terminal::Clear(terminal::ClearType::All)
    ).expect("error");

    loop {
        bool_control_list.smart_draw(&mut stdo);

        input_manager.read_crossterm_event();

        bool_control_list.on_input(input_manager.to_input_event());
//        println!("{:?}", input_manager.to_input_event());
    }


}
