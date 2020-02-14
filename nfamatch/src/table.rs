use std::fmt;
use std::str::FromStr;

use dfa_optimizer::Table as DfaTable;

use crate::row::Row;

#[derive(Debug, Default)]
pub struct Table {}

impl Table {
    fn new(/* Add Args */) -> Self {
        todo!()
    }

    fn as_dfa(&self) -> DfaTable {
        todo!()
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // writeln!(f, "{}", something)...
        todo!()
    }
}

impl From<Vec<Row>> for Table {
    fn from(input: Vec<Row>) -> Self {
        todo!()
    }
}

impl FromStr for Table {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
