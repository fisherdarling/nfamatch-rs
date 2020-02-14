pub mod nfa;
pub mod row;
pub mod table;

pub use nfa::Nfa;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
