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
     */
    pub fn replace_or_register(&mut self, q: StateId, dawg: &mut DAWG) {
        trace!("replace_or_register: q = {}", q);
        let q_child = dawg.get_last_child(q);
        if dawg.has_any_children(q_child) {
            self.replace_or_register(q_child, dawg);
        }
        // On return from recursion, all states reachable from q_child
        // are from the registry. q_child itself may or may not be in 
        // the registry, so check that now.
        if let Some(eq) = self.search_for_equiv(q_child, dawg) {
            trace!("replace_or_register: Found equiv: {} = {}", q_child, *eq);
            dawg.set_last_child(q, *eq);
            dawg.remove_state(q_child);
            return;
        } 
        trace!("replace_or_register: Registering {}", q_child);
        self.add(q_child, dawg);
        // The above cannot use if-else, otherwise the borrow checker 
        // will not accept it. This way the first use of self is out of
        // scope by the time we hit the second.
    }

    fn search_for_equiv(&self, q: StateId, dawg: &DAWG) -> Option<&StateId> {
        let key = dawg.state_hash(q);
        self.states.get(&key)
    }

    fn add(&mut self, q: StateId, dawg: &DAWG) {
        let key = dawg.state_hash(q);
        self.states.entry(key).or_insert(q);
    }
}
