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
    row_assignments: Vec<usize>,
    rows: Vec<Row>,
}

impl Table {
    pub fn new(rows: Vec<Row>, num_rows: usize) -> Self {
        let row_assignments = (0..num_rows).collect();
        Self {
            rows,
            row_assignments,
        }
    }

    pub fn blank_table(size_of_alpha: usize) -> Self {
        let row_assignments = (0..size_of_alpha).collect();
        Self {
            rows: Vec::new(),
            row_assignments,
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
        debug!("running does match on input: {:?}", input);
        if self.rows.is_empty() {
            debug!("rows are empty");
            return Some(0);
        }

        if input.is_empty() && !self.rows[0].is_accepting() {
            debug!("accepting empty state");
            return Some(0);
        }

        let mut current_state = 0;
        let mut chars = input.char_indices();

        while let Some((n, character)) = chars.next() {
            debug!("{:?}", (n, character));
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
        // self.remove_dead_state_simple();
        // self.remove_dead_states();

        // Deal with borrows.
        let Self {
            row_assignments,
            rows,
        } = self;

        debug!("Alpha assigns after optimize {:?}", row_assignments);

        // Update all assignments to reflect reality.
        for row in rows {
            for idx in row.transitions_mut().iter_mut().flatten() {
                *idx = row_assignments[*idx];
            }

            row.id = row_assignments[row.id];
        }

        info!("Optimized Table: \n{}", self);
    }

    fn optimize_step(&mut self) -> bool {
        info!("optimize step");

        info!("remove dead states");
        // self.remove_dead_states();
        self.remove_dead_branches();

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
        info!("Alpha assignments: {:?}", self.row_assignments);

        let (accepting_states, na_states): (State, State) = self
            .rows()
            .iter()
            .map(|r| r.id)
            .partition(|index| self[self.row_assignments[*index]].is_accepting());

        stack.push((accepting_states, 0));
        stack.push((na_states, 0));

        while let Some((state, idx)) = stack.pop() {
            debug!("char: {}, state: {:?}", idx, state);

            let mut character_aggregate: BTreeMap<Option<usize>, State> = BTreeMap::new();

            debug!("Aggregating States on: {}", idx);
            for s in state {
                let transition = self[self.row_assignments[s]][idx].map(|i| self.row_assignments[i]);
                character_aggregate.entry(transition).or_default().insert(s);
            }

            debug!("Aggregates:");
            for (key, value) in character_aggregate.iter() {
                debug!("{:?} => {:?}", key, value);
            }

            for (_, state) in character_aggregate.into_iter().filter(|(_, s)| s.len() > 1) {
                if idx + 1 >= alpha.len() {
                    debug!("Merging: {:?}", state);
                    merge_set.insert(state);
                } else {
                    debug!("Pushing: {:?}", state);
                    stack.push((state, idx + 1));
                }
            }

            debug!("");
        }

        info!("DFS dead_state removal");
        let ret = !merge_set.is_empty();// || self.remove_dead_states();
        debug!("ret: {}", ret);

        for state in merge_set {
            self.merge(state);
        }

        ret
    }

    pub fn remove_dead_branches(&mut self) {
        let mut marked: Vec<usize> = Vec::new();
        for row in 0..self.rows.len() {
            if marked.contains(&row) {
                continue;
            }
            self.dead_bfs(row, &mut marked, &mut BTreeSet::new());
        }
        marked.sort();
        while let Some(row) = marked.pop() {
            self.rows.remove(row);
        }
        self.make_indexable();
    }

    fn dead_bfs(
        &self,
        row: usize,
        mut marked: &mut Vec<usize>,
        mut seen: &mut BTreeSet<usize>,
    ) -> bool {
        if self.rows[row].is_accepting() {
            return true;
        }
        if seen.contains(&row) {
            return false;
        }
        if marked.contains(&row) {
            return false;
        }
        let mut is_alive = false;
        seen.insert(row);

        for transition in self.rows[row].transitions() {
            match transition {
                Some(t) => is_alive = is_alive | self.dead_bfs(*t, &mut marked, &mut seen),
                None => {}
            }
        }
        if !is_alive {
            marked.push(row);
        }

        is_alive
    }
    fn make_indexable(&mut self) {
        let mut state_map: BTreeMap<usize, usize> = BTreeMap::new();
        state_map.insert(0, 0); // Start node is ALWAYS 0

        for row in self.rows.iter() {
            // Populate a map
            let id = row.id;
            if !state_map.contains_key(&id) {
                state_map.insert(id, state_map.len());
            }
        }

        for row in self.rows_mut() {
            // Change all the transitions to correct index
            if let Some(id) = state_map.get(&row.id) {
                row.id = *id;
            }
            for transition in row.transitions_mut() {
                match transition {
                    Some(t) => {
                        if let Some(trans) = state_map.get(t) {
                            *transition = Some(*trans);
                        } else {
                            // Get rid of transitions to nodes that do not exist
                            *transition = None;
                        }
                    }
                    None => (),
                };
            }
        }
    }

    // pub fn remove_dead_states(&mut self) {
    //     let mut marked: BTreeSet<usize> = BTreeSet::new();

    //     debug!("dead states: {:?}", dead_states);
    //     for dead_state in &dead_states {
    //         self.remove_row_id(*dead_state);
    //     }

    //     debug!("table after removing dead states: \n{}", self);

    //     !dead_states.is_empty()
    // }

    fn leads_to_accepting(&self, state: usize, seen: &mut BTreeSet<usize>) -> bool {
        debug!("l2a {}: {:?}", state, seen);

        if self.rows[self.row_assignments[state]].is_accepting() {
            debug!("true: {}", state);
            return true;
        }

        if seen.contains(&state) {
            return false;
        }

        seen.insert(state);

        for transition in self.rows[self.row_assignments[state]].transitions().iter().flatten() {
            debug!("checking transition: {}", transition);

            if self.leads_to_accepting(*transition, seen) {
                debug!("leads to accepting: {}", transition);
                return true;
            }
        }

        false
    }

    pub fn remove_row_id(&mut self, row_id: usize) {
        let row_idx = self.row_assignments[row_id];
        
        self.rows.remove(row_idx);

        for row in self.rows_mut() {
            // Remove all transitions that reference this state
            for transition in row.transitions_mut() {
                if *transition == Some(row_id) {
                    *transition = None;
                }
            }
        }

        for t in self.row_assignments.iter_mut() {
            if *t > row_idx {
                *t -= 1;
            }
        }
    }

    pub fn merge(&mut self, state: State) {
        debug!("Merging state: {:#?}", state);
        debug!("Alpha assigns before merge: {:?}", self.row_assignments);
        debug!("Table before merge: \n{}", self);
        let mut states: Vec<usize> = state.into_iter().collect();

        while states.len() > 1 {
            let to_remove = states.pop().unwrap();
            let to_keep = states.pop().unwrap();
            let to_remove_idx = self.row_assignments[to_remove];
            let to_keep_idx = self.row_assignments[to_keep];
            debug!("To Keep: {} => {}", to_keep, to_keep_idx);
            debug!("To Remove: {} => {}", to_remove, to_remove_idx);

            self.merge_two(to_keep, to_remove);
            
            debug!("Table after merging {}, {}:\n{}", to_keep, to_remove, self);
            debug!(
                "Alpha assigns after merging of two states: {:?}",
                self.row_assignments
            );
            
            states.push(to_keep);
        }

        debug!("Alpha assigns after merge: {:?}", self.row_assignments);
        debug!("Table after merge: \n{}", self);
    }
    // TODO comment this code
    pub fn merge_two(&mut self, to_keep: usize, to_remove: usize) {
        debug!("Keep: {} , Remove {}", to_keep, to_remove);
        debug!("Self at the start of merge_two \n{}", *self);
        let is_accepting = self[to_keep].is_accepting() || self[to_remove].is_accepting();
        self.rows[to_keep].set_accepting(is_accepting);

        self.rows.remove(to_remove);
        // for t in self.row_assignments.iter_mut() {
        //     if *t == to_remove {
        //         *t = to_keep;
        //     } else if *t > to_remove {
        //         *t -= 1;
        //     }
        // }

        for row in self.rows_mut() {
            for trans in row.transitions_mut() {
                if let Some(t) = trans {
                    if *t == to_remove {
                        *t = to_keep;
                    }
                }
            }
        }
        self.make_indexable();
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
