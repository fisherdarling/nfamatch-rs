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
    // Lambda is always char 0
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
        let mut statemap: HashMap<StateSet, usize> = HashMap::new();
        let mut counter = 0;
        let mut L = Vec::new();
        let mut B = BTreeSet::new();

        B.insert(0); // insert starting node
        B = self.follow_lambda(&B);
        let mut new_row = DfaRow::blank_row(
            false,
            counter,
            self.character_map.len() - 1
        ); // -1 because the DFA does not have lambda
        table.push_row(new_row);
        let B2 = B.clone();
        statemap.insert(B, counter);
        counter += 1;

        L.push(B2);
        //End of firs slide
        counter = counter + 1;
        //TODO insert a blank row into table

        while L.len() > 0 {
            let S = L.pop().unwrap();
            for (_, character) in self.character_map.iter() {
                let R = self.follow_lambda(&self.follow_char(&S, *character));
                let R3 = R.clone();
                if !(statemap.contains_key(&R)) {
                    let mut new_row = DfaRow::blank_row(
                        false,
                        counter,
                        alpha_len
                    );
                    table.push_row(new_row);

                    match R.intersection(&self.accepting_states).next() {
                        Some(_) => table[counter].set_accepting(true),
                        None => (),
                    }
                    
                    let R2 = R.clone();
                    statemap.insert(R, counter);

                    counter += 1;
                    L.push(R2)
                }
                let curr_row = statemap[&S];
                let tran_row = statemap[&R3];
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
        let mut L = Vec::from_iter(states.into_iter());
        let mut S = StateSet::new(); 
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