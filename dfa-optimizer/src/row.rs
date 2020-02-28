use log::*;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Default, Debug, Clone)]
pub struct Row {
    accepting_state: bool,
    pub id: usize,
    transitions: Vec<Option<usize>>,
}

impl Row {
    pub fn from_str_custom(input: &str) -> Result<Row, ()> {
        println!("parsing input to row: {}", input);
        let tokens: Vec<&str> = input.split(' ').collect();

        match tokens.as_slice() {
            [accept, id, transitions @ ..] => {
                let is_accept = *accept == "+";
                let id = id.parse().unwrap();
                let transitions: Vec<Option<usize>> = transitions
                    .iter()
                    .map(|s| {
                        if *s == "E" {
                            None
                        } else {
                            Some(s.parse().unwrap())
                        }
                    })
                    .collect();

                Ok(Row::new(is_accept, id, transitions))
            }
            _ => Err(()),
        }
    }

    pub fn new(accepting_state: bool, id: usize, transitions: Vec<Option<usize>>) -> Self {
        Self {
            accepting_state,
            id,
            transitions,
        }
    }

    pub fn blank_row(accepting_state: bool, id: usize, num_transitions: usize) -> Self {
        Self {
            accepting_state,
            id,
            transitions: (0..num_transitions).map(|_| None).collect(), // All transitions are None
        }
    }

    pub fn is_accepting(&self) -> bool {
        self.accepting_state
    }

    pub fn set_accepting(&mut self, accepting: bool) {
        self.accepting_state = accepting;
    }

    pub fn transitions(&self) -> &[Option<usize>] {
        &self.transitions
    }

    pub fn transitions_mut(&mut self) -> &mut [Option<usize>] {
        &mut self.transitions
    }
}

impl Index<usize> for Row {
    type Output = Option<usize>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.transitions[index]
    }
}

impl IndexMut<usize> for Row {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.transitions[index]
    }
}

impl FromStr for Row {
    type Err = ();

    fn from_str(input: &str) -> Result<Row, ()> {
        println!("parsing input to row: {}", input);
        let tokens: Vec<&str> = input.split(' ').collect();

        match tokens.as_slice() {
            [accept, id, transitions @ ..] => {
                let is_accept = *accept == "+";
                let id = id.parse().unwrap();
                let transitions: Vec<Option<usize>> = transitions
                    .iter()
                    .map(|s| {
                        if *s == "E" {
                            None
                        } else {
                            Some(s.parse().unwrap())
                        }
                    })
                    .collect();

                Ok(Row::new(is_accept, id, transitions))
            }
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let accepting = if self.is_accepting() { "+" } else { "-" };

        // write!(f, "{} ", accepting)?;

        let values: Vec<String> = self
            .transitions
            .iter()
            .map(|t| {
                if let Some(v) = t {
                    v.to_string()
                } else {
                    "E".to_owned()
                }
            })
            .collect();

        write!(
            f,
            "{} {} {}",
            accepting,
            self.id,
            values.as_slice().join(" ")
        )
    }
}
