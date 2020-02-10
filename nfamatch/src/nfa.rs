use std::collections::{HashMap, BTreeSet};
use std::path::Path;
use std::iter::FromIterator;
use dfa_optimizer::{Row as DfaRow, Table as DfaTable};

pub type State = BTreeSet<usize>;
pub struct Nfa {
    // transition[start node][char][outgoing#] = end node
    // Starting state is always node 0
    // Lambda is always char 0
    transitions: Vec<Vec<Vec<usize>>>,
    accepting_states: Vec<usize>,
    character_map:HashMap<usize, char>,
}

impl Nfa {
    pub fn new() -> Self {
        todo!()
    }

    pub fn to_dfa(&self) -> DfaTable {
        /*
        let mut table = DfaTable::new();
        let mut statemap: HashMap<State, usize> = HashMap::new();
        let counter = 0;
        let mut L = Vec::new();
        let mut B = BTreeSet::new();
        B.insert(0); // insert starting node
        B = self.follow_lambda(B);
        L.push(B);
        statemap[B] = counter;
        counter = counter + 1;
        //TODO insert a blank row into table

        while L.len() > 0 {
            let S = L.pop();
            // psuedo code ahead 
        } */

        
        todo!()
        //return table
    }

    /*
    * returns the set of NFA states encountered by
    * recursively following only λ transitions.
    */
    fn follow_lambda(&self, states: &State) -> State {
        let mut L = Vec::from_iter(states.into_iter());
        let mut S = State::new(); 
        while L.len() > 0 {
            if let Some(t) = L.pop() {
                S.insert(*t);
                for l_tran in self.transitions[*t][0].iter() {
                    if !S.contains(&l_tran) {
                        S.insert(*l_tran);
                        L.push(l_tran);
                    }
                }
            }
        }
        S
    }

    /*
    * returns the set of NFA states obtained from following character c
    * from a set of states.
    */
    fn follow_char(&self, states: &mut State, c: usize) -> State {
        let mut follow = BTreeSet::new();
        for state in states.iter() {
            for transition in self.transitions[*state][c].iter() {
                follow.insert(*transition);
            }
        }
        follow
    }

    pub fn from_file<P: AsRef<Path>> (file:P) -> Self {
        todo!()
    }
}