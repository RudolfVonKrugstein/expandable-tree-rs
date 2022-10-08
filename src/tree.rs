use crate::navigator::Navigator;
use crate::subtree::Subtree;
use crate::values::{TreeValues, Zip2Values, Zip3Values};

#[derive(Debug)]
pub struct FlatTree<TV>
where
    TV: TreeValues,
{
    pub(crate) nav: Navigator,
    pub(crate) values: TV,
}

impl<TV: TreeValues> FlatTree<TV> {
    pub fn root(&self) -> Subtree<TV> {
        Subtree::new(&self.values, &self.nav, 0)
    }
}

impl<A> FlatTree<Vec<A>> {
    // Destructive!
    pub fn expand<B>(self: Self, new_values: Vec<B>) -> FlatTree<Zip2Values<A, B>> {
        FlatTree {
            nav: self.nav,
            values: Zip2Values::from_vecs(self.values, new_values),
        }
    }
}

impl<A, B> FlatTree<Zip2Values<A, B>> {
    // Destructive!
    pub fn flange<N>(self: Self, new_values: Vec<N>) -> FlatTree<Zip3Values<A, B, N>> {
        FlatTree {
            nav: self.nav,
            values: self.values.zip(new_values),
        }
    }

    pub fn un_flange(self: Self) -> (FlatTree<Vec<A>>, Vec<B>) {
        let (values, output) = self.values.split();
        (
            FlatTree {
                nav: self.nav,
                values,
            },
            output,
        )
    }
}
