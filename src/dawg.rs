/// dawg.rs

use std::collections::HashMap;

//use registry::Registry;
use states::{StateId, StateSet, State, StateHash, Transition, HashIncrement};

pub struct DAWG {
    states: Vec<State>,
}


impl DAWG {

    pub fn new() -> DAWG {
        DAWG {
            states: vec![],
        }
    }

    pub fn get_start_state(&self) -> &State {
        &self.states[0]
    }

    pub fn delta(&self, q1: StateId, c: char) -> Option<(StateId, HashIncrement)> {
        let state1 = &self.states[q1];
        state1.react(c)
    }

    pub fn print(&self) {
        info!("Printing DAWG...");
        for q in &self.states {
            q.print();
        }
    }
}


pub struct DawgBuilder {

    states: StateSet,
    registry: HashMap<StateHash, StateId>,

}

impl DawgBuilder {

    pub fn new() -> DawgBuilder {
        DawgBuilder {
            states: StateSet::new(),
            registry: HashMap::new(),
        }
    }

    pub fn build(mut self) -> DAWG {
        self.replace_or_register(0);
        let q0 = self.states.at(0);
        let key0 = q0.registry_key();
        self.registry.insert(key0, 0);
        // Now remove unregistered states, yielding a "compressed"
        // state vector that will go in the DAWG.
        // Note that the use of a free-list in StateSet assures that our
        // final state IDs make a proper prefix of the natural numbers.
        // So we can use the state ID as an index into the final state vector.
        let size = self.registry.len();
        let mut states: Vec<State> = Vec::with_capacity(size);
        unsafe { states.set_len(size); }
        for v in self.registry.values() {
            states[*v] = self.states.at(*v).clone();
        }
        // We should be able to discard the DawgBuilder data members at this point.
        DawgBuilder::add_hash_increments(&mut states);
        DAWG {
            states: states,
        }
    }

    fn add_hash_increments(states: &mut Vec<State>) {
        let size = states.len();
        let mut memo: Vec<HashIncrement> = Vec::with_capacity(size);
        for _ in 0..size {
            memo.push(0);
        }
        for id in 0..size {
            DawgBuilder::hashes_for_state(states, id, &mut memo);
        }
    }

    fn hashes_for_state(states: &mut Vec<State>, id: StateId, memo: &mut Vec<HashIncrement>) {
        let mut counter: HashIncrement = 0;
        for tid in 0..states[id].arcs.len() {
            counter = DawgBuilder::compute_trans_hash(&mut counter, states, id, tid, memo);
        }
    }

    fn compute_trans_hash(
        counter: &mut HashIncrement,
        states:  &mut Vec<State>,
        state_id: StateId,
        trans_id: usize,
        memo: &mut Vec<HashIncrement>
    ) -> HashIncrement {
        let mut increment = *counter;
        let target = states[state_id].arcs[trans_id].target;
        if states[target].is_final() {
            increment += 1;
        }
        states[state_id].arcs[trans_id].hash_increment = increment;
        *counter + DawgBuilder::right_lang_size(states, target, memo)
    }

    fn right_lang_size(
        states: &mut Vec<State>, 
        id: StateId, 
        memo: &mut Vec<HashIncrement>
    ) -> HashIncrement {
        let mut counter = memo[id];
        if counter > 0 {
            return counter;
        }
        if states[id].is_final() {
            counter = 1;
        }
        for tid in 0..states[id].arcs.len() {
            let tgt = states[id].arcs[tid].target;
            counter += DawgBuilder::right_lang_size(states, tgt, memo);
        }
        memo[id] = counter;
        counter
    }

    pub fn add_word(mut self, word: &str) -> DawgBuilder {
        trace!("add_word: Adding word '{}'", word);
        let (common_prefix, last_state) = self.recognized_prefix(word);
        trace!("add_word: common_prefix: '{}'; last_state: {}", common_prefix, last_state);
        if self.has_any_children(last_state) {
            self.replace_or_register(last_state);
        }
        let current_suffix = &word[common_prefix.len()..];
        self.add_suffix(last_state, current_suffix);

        self
    }

    fn recognized_prefix<'a>(&self, word: &'a str) -> (&'a str, StateId) {
        let mut q1 = 0;     // 0 is by convention always the start state 
        let mut end = 0;
        for c in word.chars() {
            let state1 = self.states.at(q1);
            match state1.react(c) {
                None => {
                    break;
                }
                Some((q2, _h)) => {
                    end += 1;
                    q1 = q2;
                }
            }
        }
        (&word[0..end], q1)
    }

    fn add_suffix(&mut self, q: StateId, suff: &str) {
        trace!("add_suffix: q = {}, suff = '{}'", q, suff);
        let mut q1 = q;
        for ch in suff.chars() {
            let q2 = self.make_state();
            self.add_child(q1, ch, q2);
            q1 = q2;
        }
        self.set_final(q1, true);
    }


    //---------------------------------------------------------------------


    fn has_any_children(&self, q: StateId) -> bool {
        self.states.at(q).has_any_children()
    }

    fn get_last_child(&self, q: StateId) -> StateId {
        let s: &State = self.states.at(q);
        s.get_last_child()
    }

    fn remove_state(&mut self, q: StateId) {
        self.states.remove(q);
    }

    fn make_state(&mut self) -> StateId {
        self.states.make_state()
    }

    /**
     * This assumes that state q2 exists!
     */
    fn add_child(&mut self, q1: StateId, ch: char, q2: StateId) {
        //trace!("add_child: q1 = {}; ch = '{}'; q2 = {}", q1, ch, q2);
        let mut state = self.states.at_mut(q1);
        state.add_trans(ch, q2);
    }

    fn set_final(&mut self, q: StateId, is_fin: bool) {
        let mut state = self.states.at_mut(q);
        state.is_final = is_fin;
    }


    //---------------------------------------------------------------------


    /**
     * Don't call this unless you know that q has some children.
     * Note that q itself is not added to the registry here. 
     * That means that state 0 (and only state 0) will need to be added
     * by hand.
     */
    fn replace_or_register(&mut self, parent_id: StateId) {
        trace!("replace_or_register: parent_id = {}", parent_id);
        let child_id = self.get_last_child(parent_id);
        if self.has_any_children(child_id) {
            self.replace_or_register(child_id);
        }
        // On return from recursion, all states reachable from child_id
        // are from the registry. child_id itself may or may not be in 
        // the registry, so check that now.
        self.merge_equiv_states(parent_id, child_id);
    }

    fn merge_equiv_states(&mut self, parent_id: StateId, child_id: StateId) {
        let child_key = self.get_key_for(child_id);
        let opt_id = self.get_registered(child_key);
        match opt_id {
            Some(eq_id) => {
                trace!("replace_or_register: Found equiv: {} = {}", child_id, eq_id);
                self.replace_last_child(parent_id, eq_id);
                self.remove_state(child_id);
            }
            None => {
                trace!("replace_or_register: Registering {}", child_id);
                self.register(child_id);
            }
        }
    }

    /**
     * This is critical to satisfying the borrow checker. Basically, a
     * straight HashMap::get() would return an Option<&usize>, leaving
     * a reference trail back to self. Here we break that trail by replacing
     * the reference with a clone, basically.
     */
    fn get_registered(&self, key: StateHash) -> Option<usize> {
        match self.registry.get(&key) {
            Some(eq_id) => Some(*eq_id),
            _ => None
        }
    }

    fn get_key_for(&self, id: StateId) -> StateHash {
        let state = self.states.at(id);
        state.registry_key()
    }

    fn register(&mut self, id: StateId) {
        let state = self.states.at(id);
        let key = state.registry_key();
        self.registry.insert(key, id);
    }

    fn replace_last_child(&mut self, par_id: StateId, eq_id: StateId) {
        let parent = self.states.at_mut(par_id);
        parent.set_last_child(eq_id);
    }

}
