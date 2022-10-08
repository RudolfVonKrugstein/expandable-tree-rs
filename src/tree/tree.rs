use super::subtree::Subtree;
use super::RefFlatTree;
use crate::navigator::Navigator;
use crate::values::{TreeValues, Zip2RefValues, Zip2Values, Zip3Values};

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
    pub fn flange<B>(self: Self, new_values: Vec<B>) -> FlatTree<Zip2Values<A, B>> {
        FlatTree {
            nav: self.nav,
            values: Zip2Values::from_vecs((self.values, new_values)),
        }
    }

    pub fn ref_flange<'a, B>(
        &'a self,
        new_values: &'a Vec<B>,
    ) -> RefFlatTree<'a, Zip2RefValues<A, B>> {
        RefFlatTree {
            nav: &self.nav,
            values: Zip2RefValues::from_vecs((&self.values, new_values)),
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
