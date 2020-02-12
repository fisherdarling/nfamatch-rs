#![feature(never_type)]

pub mod row;
pub mod table;

pub use row::Row;
pub use table::Table;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
