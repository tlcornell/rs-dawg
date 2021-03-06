/// states.rs

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub type StateId = usize;
pub type HashIncrement = usize;
pub type StateHash = u64; //String;

pub struct StateSet {

    states: Vec<State>,
    free: Vec<StateId>,

}

/**
 * A StateSet should never be empty. An "empty" StateSet still contains
 * a single non-final state with no arcs. That's what new() constructs.
 */
impl StateSet {

    pub fn new() -> StateSet {
        let q1 = State {
            id: 0,
            is_final: false,
            arcs: vec![],
        };
        StateSet { 
            states: vec![q1],
            free: vec![],
        }
    }

    pub fn at(&self, id: StateId) -> &State {
        &self.states[id]
    }

    pub fn at_mut(&mut self, id: StateId) -> &mut State {
        &mut self.states[id]
    }

    /**
     * We call this from replace_or_register(), at a point where we have
     * just made id unreachable. So leaving it in the vector until it is
     * overwritten should be okay...
     */
    pub fn remove(&mut self, id: StateId) {
        self.free.push(id);
    }

    pub fn make_state(&mut self) -> StateId {
        let mut q = State {
            id: 0,
            is_final: false,
            arcs: vec![],
        };
        let id;
        if self.free.is_empty() {
            id = self.states.len();
            q.id = id;
            self.states.push(q);
        } else {
            id = self.free.pop().unwrap();
            q.id = id; // make borrow checker happy
            self.states[id] = q;
        }
        id
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        trace!("DAWG size: {}", self.states.len());
        for (id, state) in self.states.iter().enumerate() {
            if self.free.contains(&id) {
                continue;
            }
            state.print();
        }
    }


}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    id: StateId,
    pub is_final: bool,
    pub arcs: Vec<Transition>,
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Transition {
    label: char,
    pub hash_increment: HashIncrement,
    pub target: StateId,
}



impl State {

    pub fn add_trans(&mut self, lbl: char, tgt: StateId) {
        self.arcs.push(Transition {
            label: lbl,
            hash_increment: 0,
            target: tgt,
        });
    }

    pub fn react(&self, c: char) -> Option<(StateId, HashIncrement)> {
        for t in &self.arcs {
            if c == t.label {
                return Some((t.target, t.hash_increment));
            }
        }
        None
    }

    pub fn is_final(&self) -> bool {
        self.is_final
    }

    pub fn has_any_children(&self) -> bool {
        !self.arcs.is_empty()
    }

    pub fn get_last_child(&self) -> StateId {
        let last_arc = self.arcs.last().unwrap();
        last_arc.target
    }

    pub fn set_last_child(&mut self, q: StateId) {
        let last_arc: &mut Transition = self.arcs.last_mut().unwrap();
        last_arc.target = q;
    }

    pub fn registry_key(&self) -> StateHash {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        let result = hasher.finish();
        trace!("state_hash: {} #-> {:x}", self.id, result);
        result
    }

    pub fn print(&self) {
        if self.is_final {
            print!("*");
        } else {
            print!(" ");
        }
        print!("{}:", self.id);
        if self.arcs.is_empty() { println!(); }
        for t in self.arcs.iter() {
            println!("\t{} -> {} ({})", t.label, t.target, t.hash_increment);
        }
    }

}


impl Hash for State {
    /**
     * We have to implement Hash for State so that we can suppress the
     * id field, which introduces artificial distinctions.
     */
    fn hash<H: Hasher>(&self, hasher_state: &mut H) {
        self.is_final.hash(hasher_state);
        self.arcs.hash(hasher_state);
    }
}
