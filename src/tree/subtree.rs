use crate::navigator::Navigator;
use crate::values::{ItemOf, TreeValues};

pub struct Subtree<'a, TV>
where
    TV: TreeValues,
{
    values: &'a TV,
    nav: &'a Navigator,
    pos: usize,
}

impl<'a, TV> Subtree<'a, TV>
where
    TV: TreeValues,
{
    pub(crate) fn new(values: &'a TV, nav: &'a Navigator, pos: usize) -> Subtree<'a, TV> {
        Subtree { values, nav, pos }
    }

    pub fn value(&self) -> ItemOf<'_, TV> {
        self.values.get(self.pos)
    }

    pub fn children(&self) -> Vec<Subtree<'a, TV>> {
        self.nav
            .children(self.pos)
            .iter()
            .map(|i| Subtree::new(self.values, self.nav, *i))
            .collect()
    }

    pub fn children_values(&self) -> Vec<ItemOf<'_, TV>> {
        self.nav
            .children(self.pos)
            .iter()
            .map(|i| self.values.get(*i))
            .collect()
    }

    pub fn parent(&self) -> Option<Subtree<'a, TV>> {
        self.nav.parent(self.pos).map(|i| Subtree {
            values: self.values,
            nav: self.nav,
            pos: i,
        })
    }

    pub fn prev_sibling(&self) -> Option<Subtree<'a, TV>> {
        self.nav.prev_sibling(self.pos).map(|i| Subtree {
            values: self.values,
            nav: self.nav,
            pos: i,
        })
    }

    pub fn next_sibling(&self) -> Option<Subtree<'a, TV>> {
        self.nav.next_sibling(self.pos).map(|i| Subtree {
            values: self.values,
            nav: self.nav,
            pos: i,
        })
    }

    pub fn first_child(&self) -> Option<Subtree<'a, TV>> {
        self.nav.first_child(self.pos).map(|i| Subtree {
            values: self.values,
            nav: self.nav,
            pos: i,
        })
    }
}
