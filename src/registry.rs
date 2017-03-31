/// registry.rs

use dawg::DAWG;
use states::{State, StateId, StateHash};
use std::collections::HashMap;

pub struct Registry {
    states: HashMap<StateHash, StateId>,
}


impl Registry {

    pub fn new() -> Registry {
        Registry {
            states: HashMap::new(),
        }
    }

    /**
     * Don't call this unless you know that q has some children.
     * Note that q itself is not added to the registry here. 
     * That means that state 0 (and only state 0) will need to be added
     * by hand.
     */
    pub fn replace_or_register(&mut self, parent_id: StateId, dawg: &mut DAWG) {
        trace!("replace_or_register: parent_id = {}", parent_id);
        let child_id = dawg.get_last_child(parent_id);
        if dawg.has_any_children(child_id) {
            self.replace_or_register(child_id, dawg);
        }
        // On return from recursion, all states reachable from child_id
        // are from the registry. child_id itself may or may not be in 
        // the registry, so check that now.
        if let Some(eq) = self.search_for_equiv(child_id, dawg) {
            trace!("replace_or_register: Found equiv: {} = {}", child_id, *eq);
            dawg.set_last_child(parent_id, *eq);
            dawg.remove_state(child_id);
            return;
        } 
        trace!("replace_or_register: Registering {}", child_id);
        self.add(child_id, dawg);
        // The above cannot use if-else, otherwise the borrow checker 
        // will not accept it. This way the first use of self is out of
        // scope by the time we hit the second.
    }

    fn search_for_equiv(&self, q: StateId, dawg: &DAWG) -> Option<&StateId> {
        let key = dawg.state_hash(q);
        self.states.get(&key)
    }

    pub fn add(&mut self, q: StateId, dawg: &DAWG) {
        let key = dawg.state_hash(q);
        self.states.entry(key).or_insert(q);
    }
}
