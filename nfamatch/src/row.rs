use std::fmt;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct Row {
    accepting_state: bool,
    from_id: usize,
    to_id: usize,
    transitions: Vec<char>,
}

impl Row {
    fn new(accepting_state: bool, from_id: usize, to_id: usize, transitions: Vec<char>) -> Self {
        Self {
            accepting_state,
            from_id,
            to_id,
            transitions,
        }
    }

    pub fn get_accepting_state(&self) -> bool {
        self.accepting_state
    }

    pub fn get_from_id(&self) -> usize {
        self.from_id
    }

    pub fn get_to_id(&self) -> usize {
        self.to_id
    }

    pub fn get_transitions(&self) -> &Vec<char> {
        &self.transitions
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // writeln!(f, "{}", something)...
        todo!()
    }
}

impl FromStr for Row {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = input.trim().split(" ").collect();

        match tokens.as_slice() {
            [accept, from_id, to_id, transitions @ ..] => {
                let is_accept = *accept == "+";
                let from_id = from_id.parse().unwrap();
                let to_id = to_id.parse().unwrap();
                let transitions: Vec<char> = transitions
                    .iter()
                    .map(|s| {
                        //println!("Trying to parse s: {}", s);
                        s.parse().unwrap()
                    })
                    .collect();

                Ok(Row::new(is_accept, from_id, to_id, transitions))
            }
            _ => unreachable!(),
        }
    }
}
