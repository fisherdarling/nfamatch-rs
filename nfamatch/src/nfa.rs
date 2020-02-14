use dfa_optimizer::{Row as DfaRow, Table as DfaTable};
use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::path::Path;

pub type StateSet = BTreeSet<usize>;
pub struct Nfa {
    // transition[start node][char][outgoing#] = end node
    // Starting state is always node 0
    // states_to_processambda is always char 0
    transitions: Vec<Vec<Vec<usize>>>,
    accepting_states: BTreeSet<usize>,
    character_map: HashMap<char, usize>,
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
        let mut row_number = 0;

        // TODO: Stack of &StateSet
        let mut states_to_process = Vec::new();
        let mut initial_state = BTreeSet::new();

        initial_state.insert(0); // insert starting node
        initial_state = self.follow_lambda(&initial_state);
        let new_row = DfaRow::blank_row(false, row_number, alpha_len);
        table.push_row(new_row);

        seen_states.insert(initial_state.clone(), row_number);
        states_to_process.push(initial_state);
        row_number += 1;

        while let Some(next_state_to_process) = states_to_process.pop() {
            for character in self.character_map.values() {
                let lambda_closure =
                    self.follow_lambda(&self.follow_char(&next_state_to_process, *character));
                let lambda_clone = lambda_closure.clone();

                if !seen_states.contains_key(&lambda_closure) {
                    let accepting_state = lambda_closure
                        .intersection(&self.accepting_states)
                        .next()
                        .is_some();

                    let new_row = DfaRow::blank_row(accepting_state, row_number, alpha_len);

                    table.push_row(new_row);

                    seen_states.insert(lambda_closure.clone(), row_number);
                    states_to_process.push(lambda_closure);

                    row_number += 1;
                }

                let current_row = seen_states[&next_state_to_process];
                let transition = seen_states[&lambda_clone];
                table[current_row][*character] = Some(transition);
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
        let mut lambda_closure = StateSet::new();
        while let Some(t) = states_to_process.pop() {
            lambda_closure.insert(*t);
            for l_tran in self.transitions[*t][0].iter() {
                if !lambda_closure.contains(&l_tran) {
                    lambda_closure.insert(*l_tran);
                    states_to_process.push(l_tran);
                }
            }
        }

        lambda_closure
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

    pub fn from_file<P: AsRef<Path>>(file: P) -> Self {
        todo!()
    }
}
