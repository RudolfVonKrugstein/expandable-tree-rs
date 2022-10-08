/** BorrowedValue value is nothing but a reference.
* We need this type (trait) anyway, because
* it allows us to bind the lifetime of the
* reference used in a trait to the lifetime
* of Self.
* See also:
* https://users.rust-lang.org/t/lifetime-bound-of-a-trait-that-is-a-member-of-a-struct/82379/2
*/
pub trait BorrowedValue<'a, _BoundBY = &'a Self> {
    type Item: 'a;
}

pub type ItemOf<'a, T> = <T as BorrowedValue<'a>>::Item;
