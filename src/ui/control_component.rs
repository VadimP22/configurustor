use std::fmt::Debug;

use super::{control_trait::Control, ui_component_trait::UiComponent};

pub trait ControlComponent: Debug + UiComponent + Control {}
