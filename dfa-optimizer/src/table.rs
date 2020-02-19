// - 0 2 1 2
// - 1 3 2 2
// - 2 2 2 2
// - 3 6 3 4
// - 6 6 7 5
// + 7 2 2 2

// - 0 1 2 1
// - 1 1 1 1
// - 2 3 1 1
// - 3 4 5 3
// - 4 4 8 6
// + 8 1 1 1

use log::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::ops::{Index, IndexMut};

use crate::row::Row;

pub type State = BTreeSet<usize>;
pub type Alphabet = Vec<usize>;

pub struct Table {
    alpha_assignments: Vec<usize>,
    rows: Vec<Row>,
}

impl Table {
    pub fn new(rows: Vec<Row>, alpha: usize) -> Self {
        debug!("Table new alpha: {}", alpha);
        let alpha_assignments = (0..alpha).collect();
        Self {
            rows,
            alpha_assignments,
        }
    }

    pub fn blank_table(size_of_alpha: usize) -> Self {
        let alpha_assignments = (0..size_of_alpha).collect();
        Self {
            rows: Vec::new(),
            alpha_assignments,
        }
    }

    // pub fn get_start(&self) -> &Row {
    //     &self.start
    // }

    pub fn rows(&self) -> &[Row] {
        &self.rows
    }

    pub fn rows_mut(&mut self) -> &mut [Row] {
        &mut self.rows
    }

    pub fn push_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    pub fn does_match(&self, input: &str, mapping: &BTreeMap<char, usize>) -> Option<usize> {
        debug!("running does match on input: {}", input);
        let mut current_state = 0;
        let mut chars = input.char_indices();

        while let Some((n, character)) = chars.next() {
            let transition = mapping.get(&character);

            if let Some(&transition) = transition {
                // If the current character matches some transition,
                // the option will be some:
                if let Some(next_state) = self[current_state][transition] {
                    // if let Some(next_state) = self[current_state][transition] {
                    current_state = next_state;
                } else {
                    // Match failed, return the character that caused it to fail:
                    return Some(n + 1);
                }
            } else {
                return Some(n + 1);
            }
        }

        // If we end at an accepting state we have matched the characters.
        if self[current_state].is_accepting() {
            None
        } else {
            Some(input.len() + 1)
        }
    }

    pub fn optimize(&mut self) {
        info!("un-optimized table: \n{}", self);
        // Optimize until completed
        while self.optimize_step() {}

        // Deal with borrows.
        let Self {
            alpha_assignments,
            rows,
        } = self;

        debug!("Alpha assigns after optimize {:?}", alpha_assignments);

        // Update all assignments to reflect reality.
        for row in rows {
            for data in row.transitions_mut() {
                if let Some(idx) = data {
                    *idx = alpha_assignments[*idx];
                }
            }
        }

        info!("Optimized Table: \n{}", self);
    }

    fn optimize_step(&mut self) -> bool {
        info!("optimize step");
        // Alpha is just a lookup table for our index optimization.
        let alpha: Alphabet = (0..self[0].transitions().len()).collect();
        info!("Alphabet: {:?}", alpha);

        // The stack of states
        let mut stack: Vec<(State, usize)> = Vec::with_capacity(10);

        // The set of all states that we need to merge together
        let mut merge_set: BTreeSet<State> = BTreeSet::new();

        // Starting state, just partition based off of accepting
        // and not accepting states.
        info!("Partitioning states");
        info!("Alpha assignments: {:?}", self.alpha_assignments);
        let (accepting_states, na_states): (State, State) = self
            .rows()
            .iter()
            .enumerate()
            .map(|(i, _)| i)
            // .inspect(|i| debug!("{}", i))
            .partition(|index| self[*index].is_accepting());

        stack.push((accepting_states, 0));
        stack.push((na_states, 0));

        while let Some((state, idx)) = stack.pop() {
            if let Some(c) = alpha.get(idx).copied() {
                let mut agg: BTreeMap<Option<usize>, State> = BTreeMap::new();

                debug!("State: {:?}", state);
                debug!("Character: {}", c);
                for s in state {
                    let row = &self[s];
                    debug!("Current state: {:?}", s);
                    debug!("Current row: \n {}", row);
                    debug!("Character inside s in state: {}", c);
                    let transition = row[c];
                    debug!("Transition: {:?}", transition);
                    let transition = transition.map(|i| self.alpha_assignments[i]);
                    agg.entry(transition).or_default().insert(s);
                }

                for (_, state) in agg.into_iter().filter(|(_, s)| s.len() > 1) {
                    if c + 1 >= alpha.len() {
                        merge_set.insert(state);
                    } else {
                        stack.push((state, idx + 1));
                    }
                }
            }
        }

        // println!("{:?}", merge_set);
        let ret = !merge_set.is_empty();

        for state in merge_set {
            self.merge(state);
        }

        ret
    }

    pub fn merge(&mut self, state: State) {
        debug!("Merging state: {:#?}", state);
        debug!("Alpha assigns before merge: {:?}", self.alpha_assignments);
        debug!("Transition table before merge: \n{}", self);
        let mut states: Vec<usize> = state.into_iter().collect();

        while states.len() > 1 {
            let to_remove = states.pop().unwrap();
            let to_keep = states.pop().unwrap();
            debug!("To Remove: {:?}", to_remove);
            debug!("To keep: {:?}", to_keep);
            self.merge_two(to_keep, to_remove);
            debug!(
                "Alpha assigns after merging of two states: {:?}",
                self.alpha_assignments
            );
            states.push(to_keep);
        }

        debug!("Alpha assigns after merge: {:?}", self.alpha_assignments);
    }

    // TODO comment this code
    pub fn merge_two(&mut self, to_keep: usize, to_remove: usize) {
        let is_accepting = self[to_keep].is_accepting() || self[to_remove].is_accepting();
        self.rows[to_keep].set_accepting(is_accepting);

        self.rows.remove(to_remove);
        for t in self.alpha_assignments.iter_mut() {
            if *t == to_remove {
                *t = to_keep;
            } else if *t > to_remove {
                *t -= 1;
            }
        }

        for row in self.rows_mut() {
            if row.id > to_remove {
                row.id -= 1;
            }
        }
    }
}

impl From<Vec<Row>> for Table {
    fn from(rows: Vec<Row>) -> Self {
        let len = rows.len();
        Table::new(rows, len)
    }
}

impl Index<usize> for Table {
    type Output = Row;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl IndexMut<usize> for Table {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rows[index]
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.rows() {
            writeln!(f, "{}", row)?;
        }

        Ok(())
    }
}
