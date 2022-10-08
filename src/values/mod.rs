mod borrowed_value;
mod zip2_ref_values;
mod zip2_values;
mod zip3_values;

pub use zip2_ref_values::Zip2RefValues;
pub use zip2_values::Zip2Values;
pub use zip3_values::Zip3Values;

use self::borrowed_value::BorrowedValue;
pub use self::borrowed_value::ItemOf;

/** TreeValues is a trait describing containers holding _owned_
* values for a FlatTree.*/
pub trait TreeValues: for<'any> BorrowedValue<'any> {
    fn get(&self, index: usize) -> ItemOf<'_, Self>;
}

/** RefTreeValues is a trait describing contianers holding _borrowed_
* value for a tree (A RefFlatTree)
*/
// pub trait RefTreeValues {
//     type Item;
//     fn get(&self, index: usize) -> &Self::Item;
// }

impl<'a, A> BorrowedValue<'a> for Vec<A> {
    type Item = &'a A;
}

impl<A> TreeValues for Vec<A> {
    fn get(&self, index: usize) -> ItemOf<'_, Self> {
        &self[index]
    }
}
