#![feature(never_type)]

pub mod row;
pub mod table;
pub mod nfa;

pub use nfa::Nfa;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
