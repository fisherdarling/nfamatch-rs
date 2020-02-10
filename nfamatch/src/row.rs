use std::str::FromStr;
use std::fmt;

#[derive(Debug, Default)]
pub struct Row {}

impl Row {
    fn new(/* Add Args */) -> Self {
        todo!()
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
        // let mut tokens: Vec<&str> = input.split(" ").collect();
        todo!()
    }
}
