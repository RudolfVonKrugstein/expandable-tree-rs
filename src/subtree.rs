use crate::FlatTree;
use crate::navigator::Navigator;

pub struct Subtree<'a, A> {
    values: &'a Vec<A>,
    nav: &'a Navigator,
    pos: usize,
}

impl<'a, A> Subtree<'a, A> {
    pub(crate) fn new(values: &'a Vec<A>, nav: &'a Navigator, pos: usize) -> Subtree<'a, A> {
        Subtree {
            values,
            nav,
            pos
        }
    }

    pub fn value(&self) -> &'a A {
        &self.values[self.pos]
    }

    pub fn children(&self) -> Vec<Subtree<'a, A>> {
        self.nav.children(self.pos).iter().map(
            |i| Subtree::new(
                self.values,
                self.nav,
                *i
            )
        ).collect()
    }

    pub fn children_values(&self) -> Vec<&'a A> {
        self.nav.children(self.pos).iter().map(
            |i| &self.values[*i]
        ).collect()
    }

    pub fn parent(&self) -> Option<Subtree<'a, A>> {
        self.nav.parent(self.pos).map(
            |i| Subtree {
                values: self.values,
                nav: self.nav,
                pos: i,
            }
        )
    }

    pub fn prev_sibling(&self) -> Option<Subtree<'a, A>> {
        self.nav.prev_sibling(self.pos).map(
            |i| Subtree {
                values: self.values,
                nav: self.nav,
                pos: i,
            }
        )
    }

    pub fn next_sibling(&self) -> Option<Subtree<'a, A>> {
        self.nav.next_sibling(self.pos).map(
            |i| Subtree {
                values: self.values,
                nav: self.nav,
                pos: i,
            }
        )
    }

    pub fn first_child(&self) -> Option<Subtree<'a, A>> {
        self.nav.first_child(self.pos).map(
            |i| Subtree {
                values: self.values,
                nav: self.nav,
                pos: i,
            }
        )
    }
}
