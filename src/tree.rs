use crate::navigator::Navigator;
use crate::navigator::Neighbors;
use crate::subtree::Subtree;

#[derive(Debug)]
pub struct FlatTree<A> {
    pub(crate) nav: Navigator,
    pub(crate) values: Vec<A>
}

impl<A> FlatTree<A> {
    pub fn root(&self) -> Subtree<A> {
        Subtree::new(
            &self.values,
            &self.nav,
            0
        )
    }

    pub fn count(&self) -> usize {
        self.nav.all_neighbors().len()
    }

    pub fn all_neighbors(&self) -> &Vec<Neighbors<usize>> {
        self.nav.all_neighbors()
    }

    pub fn depth_first_map<B, F>(&self, f: F) -> Vec<B>
        where
            F: Fn(&A, Vec<(&A,&B)>) -> B,
            B: Default {
        let mut res = Vec::with_capacity(self.nav.all_neighbors().len());
        for _ in 0..self.nav.all_neighbors().len() {
            res.push(B::default());
        }
        self.nav.for_each_depth_first(
            |i, cs| {
                res[i] = f(&self.values[i], cs.iter().map(|ci| (&self.values[*ci], &res[*ci])).collect());
            }
        );
        res
    }
}
