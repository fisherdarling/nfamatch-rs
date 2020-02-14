use std::collections::{HashMap, BTreeSet};
use std::path::Path;
use std::iter::FromIterator;
use std::fs::File;
use std::io::{BufRead, BufReader};
use dfa_optimizer::{Row as DfaRow, Table as DfaTable};

pub type StateSet = BTreeSet<usize>;
pub struct Nfa {
    // transition[start node][char][outgoing#] = end node
    // Starting state is always node 0
    // states_to_processambda is always char 0
    transitions: Vec<Vec<Vec<usize>>>,
    accepting_states: BTreeSet<usize>,
    character_map:HashMap<char, usize>,
}

impl Nfa {
    pub fn new() -> Self {
        todo!()
    }

    pub fn character_map(&self) -> &HashMap<char, usize> {
        &(self.character_map)
    }

    pub fn to_dfa(&self) -> DfaTable {
        let alpha_len = self.character_map.len() - 1;
        let mut table = DfaTable::blank_table(alpha_len);
        let mut seen_states: HashMap<StateSet, usize> = HashMap::new();
        let mut counter = 0;
        // TODO: Stack of &StateSet, rename states_to_process => state_stack
        let mut states_to_process = Vec::new();
        let mut B = BTreeSet::new();

        B.insert(0); // insert starting node
        B = self.follow_lambda(&B);
        let new_row = DfaRow::blank_row(
            false,
            counter,
            alpha_len,
        ); // -1 because the DFA does not have lambda
        table.push_row(new_row);
        let B2 = B.clone();
        seen_states.insert(B, counter);
        counter += 1;

        states_to_process.push(B2);
        //End of firs slide
        counter = counter + 1;
        //TODO insert a blank row into table

        while let Some(S) = states_to_process.pop() {
            for character in self.character_map.values() {
                let R = self.follow_lambda(&self.follow_char(&S, *character));
                let R3 = R.clone();
                if !seen_states.contains_key(&R) {
                    let accepting_state = R.intersection(&self.accepting_states).next().is_some();

                    let new_row = DfaRow::blank_row(
                        accepting_state,
                        counter,
                        alpha_len
                    );

                    table.push_row(new_row);
                    
                    let R2 = R.clone();
                    seen_states.insert(R, counter);

                    counter += 1;
                    states_to_process.push(R2)
                }
                let curr_row = seen_states[&S];
                let tran_row = seen_states[&R3];
                table[curr_row][*character] = Some(tran_row);
            }
        }
        table
    }

    /*
    * returns the set of NFA states encountered by
    * recursively following only λ transitions.
    */
    fn follow_lambda(&self, states: &StateSet) -> StateSet {
        let mut states_to_process = Vec::from_iter(states.into_iter());
        let mut S = StateSet::new(); 
        while let Some(t) = states_to_process.pop() {
            S.insert(*t);
            for l_tran in self.transitions[*t][0].iter() {
                if !S.contains(&l_tran) {
                    S.insert(*l_tran);
                    states_to_process.push(l_tran);
                }
            }
        }

        S
    }

    /*
    * returns the set of NFA states obtained from following character c
    * from a set of states.
    */
    fn follow_char(&self, states: &StateSet, c: usize) -> StateSet {
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