use crate::row::Row;
use dfa_optimizer::{Row as DfaRow, Table as DfaTable};
use log::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::path::Path;

use log::debug;

pub type StateSet = BTreeSet<usize>;

#[derive(Debug, Clone, Default)]
pub struct Nfa {
    // transition[start node][char][outgoing#] = end node
    // Starting state is always node 0
    lambda_char: char,
    transitions: Vec<Vec<Vec<usize>>>, // potentially refactor this to map?
    accepting_states: BTreeSet<usize>,
    character_map: BTreeMap<char, usize>,
}

impl Nfa {
    pub fn new() -> Self {
        todo!()
    }

    pub fn lambda_char(&self) -> char {
        self.lambda_char
    }

    pub fn character_map(&self) -> &BTreeMap<char, usize> {
        &self.character_map
    }

    pub fn to_dfa(&self) -> DfaTable {
        info!("character map: {:?} ", self.character_map());
        info!("self at the start of to_dfa {:#?}", self);
        let mut dfa_char_map = self.character_map().clone();
        dfa_char_map.remove(&self.lambda_char);
        let alpha_len = dfa_char_map.len(); // length of the new dfa alphabet

        let mut dfa_rows = Vec::new();
        let mut seen_states: BTreeMap<StateSet, usize> = BTreeMap::new();
        let mut row_number = 0;

        // TODO: Stack of &StateSet
        let mut states_to_process = Vec::new();
        let mut initial_state = BTreeSet::new();

        initial_state.insert(0); // insert starting node
        initial_state = self.follow_lambda(&initial_state);
        debug!("Initial Lambda Closure: {:?}", initial_state);

        let initial_lambda_accepting = initial_state
            .intersection(&self.accepting_states)
            .next()
            .is_some();

        let new_row = DfaRow::blank_row(initial_lambda_accepting, row_number, alpha_len);
        dfa_rows.push(new_row);
        seen_states.insert(initial_state.clone(), row_number);
        states_to_process.push(initial_state);
        row_number += 1;

        while let Some(next_state_to_process) = states_to_process.pop() {
            debug!("Next State: {:?}", next_state_to_process);
            for character in dfa_char_map.values() {
                let lambda_closure =
                    self.follow_lambda(&self.follow_char(&next_state_to_process, *character));
                debug!("{} => {:?}", character, lambda_closure);

                let lambda_clone = lambda_closure.clone();

                if !seen_states.contains_key(&lambda_closure) {
                    debug!("Lambda closure in seen_states {:?}of", lambda_closure);
                    let accepting_state = lambda_closure
                        .intersection(&self.accepting_states)
                        .next()
                        .is_some();

                    info!("Is the new row an accepting state? {}", accepting_state);
                    let new_row = DfaRow::blank_row(accepting_state, row_number, alpha_len);

                    dfa_rows.push(new_row);

                    seen_states.insert(lambda_closure.clone(), row_number);
                    states_to_process.push(lambda_closure);

                    row_number += 1;
                }

                let current_row = seen_states[&next_state_to_process];
                let transition = seen_states[&lambda_clone];
                dfa_rows[current_row][*character - 1] = Some(transition);
            }
        }

        let len = dfa_rows.len();
        DfaTable::new(dfa_rows, len)
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

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut all_rows = reader.lines().flatten();
        let first_line = all_rows.next().unwrap();

        let (character_map, lambda_char) = get_char_map(&first_line);
        let num_states: usize = get_num_states(&first_line);

        // let mut rows: Vec<Row> = all_rows.map(|r| r.parse().unwrap()).collect();
        let mut rows: Vec<Row> = Vec::new();
        for row in all_rows {
            match Row::from_str_custom(&row) {
                Ok(row) => rows.push(row),
                _ => break,
            }
            // rows.push(Row::from_str_custom(&row).unwrap());
        }

        make_indexable(&mut rows);

        let accepting_state_from_ids: Vec<usize> = rows
            .iter()
            .filter(|r| r.get_accepting_state())
            .map(|r| r.get_from_id().to_owned())
            .collect();

        let transitions: Vec<Vec<Vec<usize>>> = get_transitions(&rows, &character_map, num_states);

        Ok(Self {
            lambda_char,
            transitions,
            character_map,
            accepting_states: BTreeSet::from_iter(accepting_state_from_ids),
        })
    }
}

fn make_indexable(rows: &mut Vec<Row>) {
    let mut state_map: BTreeMap<usize, usize> = BTreeMap::new();
    state_map.insert(0, 0); // Start node is ALWAYS 0

    for row in rows.iter() {
        let from_index = row.get_from_id();
        let to_index = row.get_to_id();
        if !state_map.contains_key(&from_index) {
            state_map.insert(from_index, state_map.len());
        }
        if !state_map.contains_key(&to_index) {
            state_map.insert(to_index, state_map.len());
        }
    }

    for row in rows {
        let from_index = state_map.get(&row.get_from_id()).unwrap();
        let to_index = state_map.get(&row.get_to_id()).unwrap();
        row.set_from_id(*from_index);
        row.set_to_id(*to_index);
    }
}

fn get_transitions(
    rows: &[Row],
    char_map: &BTreeMap<char, usize>,
    num_states: usize,
) -> Vec<Vec<Vec<usize>>> {
    let mut outer: Vec<Vec<Vec<usize>>> = vec![vec![Vec::new(); char_map.len()]; num_states];
    for row in rows {
        let from_index = row.get_from_id();
        let to_index = row.get_to_id();
        for c in row.get_transitions() {
            let char_index = char_map[c];
            outer[from_index][char_index].push(to_index);
        }
    }
    outer
}
fn get_num_states(first_lines: &str) -> usize {
    first_lines
        .split(' ')
        .collect::<Vec<&str>>()
        .into_iter()
        .next()
        .unwrap()
        .parse()
        .unwrap()
}
fn get_char_map(first_line: &str) -> (BTreeMap<char, usize>, char) {
    let alphabet_letters: Vec<&str> = first_line
        .split(' ')
        .skip(1) // skip num of states
        .collect();
    let lambda_char = alphabet_letters[0].parse().unwrap();
    let mut map = BTreeMap::new();

    for (i, v) in alphabet_letters.iter().enumerate() {
        map.insert(v.parse().expect("Error while looking at alphabet"), i);
    }

    (map, lambda_char)
}
