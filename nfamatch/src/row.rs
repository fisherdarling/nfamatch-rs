use log::*;

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

    pub fn set_from_id(&mut self, value: usize) {
        self.from_id = value;
    }

    pub fn get_to_id(&self) -> usize {
        self.to_id
    }

    pub fn set_to_id(&mut self, value: usize) {
        self.to_id = value;
    }

    pub fn get_transitions(&self) -> &Vec<char> {
        &self.transitions
    }

    // type Err = ();
    pub fn from_str_custom(input: &str) -> Result<Self, ()> {
        info!("Input for from_str_custom {}", input);
        let tokens: Vec<&str> = input.trim().split_whitespace().collect();

        match tokens.as_slice() {
            [accept, from_id, to_id, transitions @ ..] => {
                let is_accept = *accept == "+";
                let from_id = from_id.parse().unwrap();
                let to_id = to_id.parse().unwrap();
                let transitions: Vec<char> =
                    transitions.iter().map(|s| s.parse().unwrap()).collect();

                Ok(Row::new(is_accept, from_id, to_id, transitions))
            }
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::row::Row;

    #[test]
    #[should_panic]
    fn test_empty_str_parse() {
        let r = Row::from_str_custom("");
        r.unwrap();
    }

    #[test]
    fn test_accept_str_parse_accepting() {
        let r = Row::from_str_custom("+ 0 1 a b c").unwrap();
        assert_eq!(r.get_accepting_state(), true);
        assert_eq!(r.get_from_id(), 0);
        assert_eq!(r.get_to_id(), 1);
        assert_eq!(*r.get_transitions(), vec!['a', 'b', 'c']);
    }

    #[test]
    fn test_accept_str_parse_non_accepting() {
        let r = Row::from_str_custom("- 0 1 a b c").unwrap();
        assert_eq!(r.get_accepting_state(), false);
        assert_eq!(r.get_from_id(), 0);
        assert_eq!(r.get_to_id(), 1);
        assert_eq!(*r.get_transitions(), vec!['a', 'b', 'c']);
    }
}
