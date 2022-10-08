//! This module exposes container for holding values
//! that can be used inside a FlatTree.

mod borrowed_value;
mod zip2_ref_values;
mod zip2_values;
mod zip3_ref_values;
mod zip3_values;

pub use zip2_ref_values::Zip2RefValues;
pub use zip2_values::Zip2Values;
pub use zip3_ref_values::Zip3RefValues;
pub use zip3_values::Zip3Values;

use self::borrowed_value::BorrowedValue;
pub use self::borrowed_value::ItemOf;

/** TreeValues is a trait describing containers holding _owned_
* values for a FlatTree.*/
pub trait TreeValues: for<'any> BorrowedValue<'any> {
    fn get(&self, index: usize) -> ItemOf<'_, Self>;
}

// Implementation for `Vec`
impl<'a, A> BorrowedValue<'a> for Vec<A> {
    type Item = &'a A;
}

impl<A> TreeValues for Vec<A> {
    fn get(&self, index: usize) -> ItemOf<'_, Self> {
        &self[index]
    }
}
