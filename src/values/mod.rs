mod borrowed_value;
mod zip2_values;
mod zip3_values;

pub use zip2_values::Zip2Values;
pub use zip3_values::Zip3Values;

use self::borrowed_value::BorrowedValue;
pub use self::borrowed_value::ItemOf;

pub trait TreeValues: for<'any> BorrowedValue<'any> {
    fn get(&self, index: usize) -> ItemOf<'_, Self>;
}

impl<'a, A> BorrowedValue<'a> for Vec<A> {
    type Item = &'a A;
}

impl<A> TreeValues for Vec<A> {
    fn get(&self, index: usize) -> ItemOf<'_, Self> {
        &self[index]
    }
}
