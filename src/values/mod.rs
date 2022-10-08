mod borrowed_value;
mod vec_values;
mod zip2_values;
mod zip3_values;

pub use vec_values::VecValues;
pub use zip2_values::Zip2Values;
pub use zip3_values::Zip3Values;

use self::borrowed_value::BorrowedValue;
pub use self::borrowed_value::ItemOf;

pub trait TreeValues: for<'any> BorrowedValue<'any> {
    fn get(&self, index: usize) -> ItemOf<'_, Self>;
}
