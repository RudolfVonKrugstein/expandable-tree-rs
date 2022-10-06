use crate::navigator::Navigator;
use crate::navigator::Neighbors;

#[derive(Debug)]
pub struct FlatTree<A> {
    pub(crate) nav: Navigator,
    pub(crate) values: Vec<A>
}

impl<A> FlatTree<A> {
    pub fn count(&self) -> usize {
        self.nav.all_neighbors().len()
    }

    pub fn get(&self, index: usize) -> Option<&A> {
        self.values.get(index)
    }

    pub fn get_index(&self) -> &Navigator {
        &self.nav
    }

    pub fn all_neighbors(&self) -> &Vec<Neighbors<usize>> {
        self.nav.all_neighbors()
    }

    pub fn children(&self, index: usize) -> Vec<&A> {
        self.nav.children(index).iter().map(
            |&i| self.values.get(i).unwrap()
        ).collect()
    }

    pub fn parent(&self, index: usize) -> Option<&A> {
        self.nav.parent(index).and_then(|i| self.values.get(i))
    }

    pub fn next_sibling(&self, index: usize) -> Option<&A> {
        self.nav.next_sibling(index).and_then(|i| self.values.get(i))
    }

    pub fn prev_sibling(&self, index: usize) -> Option<&A> {
        self.nav.prev_sibling(index).and_then(|i| self.values.get(i))
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
