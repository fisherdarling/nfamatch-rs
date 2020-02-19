pub mod nfa;
pub use nfa::Nfa;
pub mod row;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
