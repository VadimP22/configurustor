use crossterm::{
    cursor, execute, style,
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use input::input_manager::InputManager;
use ui::{components::node_controls::NodeControls, ui_component_trait::UiComponent};
// use ui::ui_component_trait::UiComponent;

mod input;
mod store;
mod ui;

fn exit() {
    disable_raw_mode().expect("error");
    execute!(
        std::io::stdout(),
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen,
        terminal::Clear(terminal::ClearType::All)
    )
    .expect("error");

    std::process::exit(0);
}

fn main() {
    let mut stdout = std::io::stdout();
    let mut input_manager = InputManager::new();
    let mut node_control = NodeControls::default(1, 1);
    node_control.add_bool_control("key1".to_string(), "Just title1".to_string());
    node_control.add_bool_control("key2".to_string(), "Just title2".to_string());
    node_control.add_bool_control("key3".to_string(), "Just title2".to_string());
    node_control.add_bool_control("key4".to_string(), "Just title2".to_string());
    node_control.add_bool_control("key5".to_string(), "Just title, no more :)".to_string());

    enable_raw_mode().expect("error");

    execute!(stdout, terminal::Clear(terminal::ClearType::All)).expect("error");

    loop {
        node_control.smart_draw(&mut stdout);
        input_manager.read_crossterm_event();
        node_control.on_input(input_manager.to_input_event());
    }
}
