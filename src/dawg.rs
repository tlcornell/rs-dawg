/// dawg.rs

use registry::Registry;
use states::{StateId, StateSet, State, StateHash};

type HashIncrement = usize;

pub struct DAWG {
    states: StateSet,
}


impl DAWG {

    pub fn new() -> DAWG {
        DAWG {
            states: StateSet::new(),
        }
    }

    pub fn get_start_state(&self) -> &State {
        self.states.at(0)
    }

    pub fn delta(&self, q1: StateId, c: char) -> Option<(StateId, HashIncrement)> {
        let state1 = &self.states.at(q1);
        state1.react(&self.states, c)
    }

    pub fn has_any_children(&self, q: StateId) -> bool {
        self.states.at(q).has_any_children()
    }

    pub fn get_last_child(&self, q: StateId) -> StateId {
        let s: &State = self.states.at(q);
        s.get_last_child()
    }

    pub fn set_last_child(&mut self, q1: StateId, q2: StateId) {
        let s: &mut State = self.states.at_mut(q1);
        s.set_last_child(q2);
    }

    pub fn remove_state(&mut self, q: StateId) {
        self.states.remove(q);
    }

    pub fn state_hash(&self, id: StateId) -> StateHash {
        let state = self.states.at(id);
        state.registry_key(id)
    }

    pub fn make_state(&mut self) -> StateId {
        self.states.make_state()
    }

    /**
     * This assumes that state q2 exists!
     */
    pub fn add_child(&mut self, q1: StateId, ch: char, q2: StateId) {
        //trace!("add_child: q1 = {}; ch = '{}'; q2 = {}", q1, ch, q2);
        let mut state = self.states.at_mut(q1);
        state.add_trans(ch, q2);
    }

    pub fn set_final(&mut self, q: StateId, is_fin: bool) {
        let mut state = self.states.at_mut(q);
        state.is_final = is_fin;
    }

    pub fn print(&self) {
        info!("Printing DAWG...");
        self.states.print();
    }
}


pub struct DawgBuilder {

    registry: Registry,
    dawg: DAWG,

}

impl DawgBuilder {

    pub fn new() -> DawgBuilder {
        DawgBuilder {
            registry: Registry::new(),
            dawg: DAWG::new(),
        }
    }

    pub fn build(mut self) -> DAWG {
        /*
        State* q0 = getStartState();
        m_Reg->replace_or_register (q0);
        delete m_Reg;
        m_Reg = NULL;
        */
        self.registry.replace_or_register(0, &mut self.dawg);
        self.dawg
    }

    pub fn add_word(mut self, word: &str) -> DawgBuilder {
        trace!("add_word: Adding word '{}'", word);
        let (common_prefix, last_state) = self.recognized_prefix(word);
        trace!("add_word: common_prefix: '{}'; last_state: {}", common_prefix, last_state);
        if self.dawg.has_any_children(last_state) {
            self.registry.replace_or_register(last_state, &mut self.dawg);
        }
        let current_suffix = &word[common_prefix.len()..];
        self.add_suffix(last_state, current_suffix);

        self
    }

    fn recognized_prefix<'a>(&self, word: &'a str) -> (&'a str, StateId) {
        let mut q1 = 0;     // 0 is by convention always the start state 
        let mut end = 0;
        for c in word.chars() {
            match self.dawg.delta(q1, c) {
                None => {
                    break;
                }
                Some((q2, h)) => {
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
            let q2 = self.dawg.make_state();
            self.dawg.add_child(q1, ch, q2);
            q1 = q2;
        }
        self.dawg.set_final(q1, true);
    }

}
