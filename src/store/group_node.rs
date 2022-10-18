use std::collections::HashMap;

pub struct GroupNode {
    map: *mut HashMap<String, GroupNode>,
    child_keys: Vec<String>,
    pub config_entries: HashMap<String, bool>,
}
