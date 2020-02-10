use std::collections::{BTreeSet, HashMap, HashSet};
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
        let alpha_assignments = (0..alpha).collect();
        Self {
            rows,
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

    pub fn does_match(&self, input: String) -> Option<u32> {
        todo!()
    }

    pub fn optimize(&mut self) {
        // Optimize until completed
        while self.optimize_step() {}

        // Deal with borrows.
        let Self {
            alpha_assignments,
            rows,
        } = self;

        // Update all assignments to reflect reality.
        for row in rows {
            for data in row.transitions_mut() {
                if let Some(idx) = data {
                    *idx = alpha_assignments[*idx];
                }
            }
        }
    }

    fn optimize_step(&mut self) -> bool {
        // Alpha is just a lookup table for our index optimization.
        let alpha: Alphabet = (0..self[0].transitions().len()).collect();

        // The stack of states
        let mut stack: Vec<(State, usize)> = Vec::with_capacity(10);

        // The set of all states that we need to merge together
        let mut merge_set: HashSet<State> = HashSet::new();

        // Starting state, just partition based off of accepting
        // and not accepting states.
        let (accepting_states, na_states): (State, State) = self
            .rows()
            .iter()
            .map(|r| r.id)
            .partition(|id| self[*id].is_accepting());

        stack.push((accepting_states, 0));
        stack.push((na_states, 0));

        while let Some((state, idx)) = stack.pop() {
            if let Some(c) = alpha.get(idx).copied() {
                let mut agg: HashMap<Option<usize>, State> = HashMap::new();

                for s in state {
                    let transition = self[s][c].map(|i| self.alpha_assignments[i]);
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
        let states: Vec<usize> = state.into_iter().collect();

        &states.windows(2).rev().for_each(|w| match w {
            &[a, b] => self.merge_two(a, b),
            _ => unreachable!(),
        });
    }

    pub fn merge_two(&mut self, a: usize, b: usize) {
        let is_accepting = self[a].is_accepting() || self[b].is_accepting();
        self.rows[a].set_accepting(is_accepting);

        self.rows.remove(b);
        self.alpha_assignments[b] = a;
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
