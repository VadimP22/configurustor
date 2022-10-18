use std::collections::HashMap;

use super::group_node::GroupNode;

pub struct GroupTree {
    map: HashMap<String, GroupNode>,
}

impl GroupTree {
    pub fn new() -> GroupTree {
        return GroupTree {
            map: HashMap::new(),
        };
    }
}
