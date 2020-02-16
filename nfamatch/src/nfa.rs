use crate::row::Row;
use dfa_optimizer::{Row as DfaRow, Table as DfaTable};
use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::path::Path;

use log::debug;

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
        debug!("Alphabet length: {}", alpha_len);
        let mut table = DfaTable::blank_table(alpha_len);
        let mut seen_states: HashMap<StateSet, usize> = HashMap::new();
        let mut row_number = 0;

        // TODO: Stack of &StateSet
        let mut states_to_process = Vec::new();
        let mut initial_state = BTreeSet::new();

        initial_state.insert(0); // insert starting node
        initial_state = self.follow_lambda(&initial_state);
        debug!("Initial Lambda Closure: {:?}", initial_state);

        let new_row = DfaRow::blank_row(false, row_number, alpha_len);
        table.push_row(new_row);

        seen_states.insert(initial_state.clone(), row_number);
        states_to_process.push(initial_state);
        row_number += 1;

        while let Some(next_state_to_process) = states_to_process.pop() {
            debug!("Next State: {:?}", next_state_to_process);
            for character in self.character_map.values() {
                let lambda_closure =
                    self.follow_lambda(&self.follow_char(&next_state_to_process, *character));
                debug!("{} => {:?}", character, lambda_closure);


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
        debug!("Final DFA:\n{}", table);
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

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<(Self), Box<dyn std::error::Error>> {
        //        transitions: Vec<Vec<Vec<usize>>>,
        //      accepting_states: BTreeSet<usize>,
        //        character_map: HashMap<char, usize>,
        let file = File::open(path)?;
        println!("File : {:?}", file);
        let reader = BufReader::new(file);

        let all_rows: Vec<String> = reader
            .lines()
            .flatten()
            .map(|r| r.parse().unwrap())
            .collect();

        println!("Rows as str: {:#?}", all_rows);

        let (first_line, rows_as_str) = all_rows.split_first().expect("Unable to parse input file");

        let char_map: HashMap<char, usize> = get_char_map(&first_line);

        println!("Char map: {:#?}", char_map);

        let rows: Vec<Row> = rows_as_str
            .get(0..) // take a look at this again, for some reason before it was getting the first line even though we split it earlier
            .unwrap()
            .iter()
            .map(|r| r.parse().unwrap())
            .collect();
        println!("Rows as data: {:#?}", rows);

        let accepting_state_from_ids: &Vec<usize> = &rows
            .iter()
            .filter(|r| r.get_accepting_state())
            .map(|r| r.get_from_id().to_owned())
            .collect();
        println!("Accepting state ids: {:#?}", accepting_state_from_ids);

        let transitions: Vec<Vec<Vec<usize>>> = get_transitions(&rows, &char_map);

        // Move this do different place? Not sure why it has to be here

        fn get_transitions(
            rows: &Vec<Row>,
            char_map: &HashMap<char, usize>,
        ) -> Vec<Vec<Vec<usize>>> {
            let mut outer: Vec<Vec<Vec<usize>>> = Vec::new();

            for row in rows {
                let mut middle: Vec<Vec<usize>> = Vec::new();
                let from_id = row.get_from_id();
                let to_id = row.get_to_id();
                for c in row.get_transitions() {
                    let mut inner: Vec<usize> = Vec::new();
                    println!(
                        "char we are pushing: {} corosponding number: {} ",
                        c,
                        *char_map.get(c).unwrap()
                    );
                    inner.push(*char_map.get(c).unwrap());
                }
            }

            outer
        }
        fn get_char_map(first_line: &String) -> HashMap<char, usize> {
            let alphabet_letters: Vec<&str> = first_line
                .split(' ')
                .collect::<Vec<&str>>()
                .into_iter()
                .skip(1) // Remove the first two chars (num cols and lambda char)
                .collect();

            let mut map = HashMap::new();
            for (i, v) in alphabet_letters.iter().enumerate() {
                map.insert(v.parse().expect("Error while looking at alphabet"), i);
            }

            map
        }

        // This is an empty thing to please the compiler as I test
        Ok(Self {
            transitions: Vec::new(),
            accepting_states: BTreeSet::new(),
            character_map: HashMap::new(),
        })
    }
}
