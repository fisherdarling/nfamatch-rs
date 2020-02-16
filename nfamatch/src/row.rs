use std::fmt;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct Row {
    accepting_state: bool,
    pub id: usize,
    transitions: Vec<char>,
}

impl Row {
    fn new(accepting_state: bool, id: usize, transitions: Vec<char>) -> Self {
        Self {
            accepting_state,
            id,
            transitions,
        }
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // writeln!(f, "{}", something)...
        todo!()
    }
}

impl FromStr for Row {
    type Err = !;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = input.split(" ").collect();

        match tokens.as_slice() {
            [accept, id, transitions @ ..] => {
                let is_accept = *accept == "+";
                let id = id.parse().unwrap();
                let transitions: Vec<char> =
                    transitions.iter().map(|s| s.parse().unwrap()).collect();

                Ok(Row::new(is_accept, id, transitions))
            }
            _ => unreachable!(),
        }
    }
}
