#![feature(never_type)]

pub(crate) mod row;
pub(crate) mod table;

pub use row::Row;
pub use table::Table;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
